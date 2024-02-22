use super::{meet_info::MeetInfo, Entries};
extern crate chrono;
extern crate serde_xml_rs;
use self::chrono::{DateTime, Local};
use self::serde_xml_rs::from_str;
use std::error::Error;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// # Errors
/// returns error if:
/// - native TLS backend cannot be initialized
/// - supplied Url cannot be parsed
/// - there was an error while sending request
/// - redirect loop was detected
/// - redirect limit was exhausted
/// - failed to decode response from the server

pub fn get_meet_list(date: DateTime<Local>) -> Result<Vec<MeetInfo>, Box<dyn Error>> {
    let meets_url = "http://medley.no/tidsjekk/stevneoppsett.asmx/VisStevneoppsett?FraNr=1&FraDato="
        .to_owned() + &date.format("%Y%m%d").to_string();

    let response = reqwest::blocking::get(meets_url)?.text()?;

    let tmp: Entries = from_str(&response)?;

    Ok(tmp.meet_setup_entries)
}

/// generate a filename for a meet from it's name
fn meet_filename(meet: &MeetInfo) -> String {
    meet.name.replace(' ', "_").to_lowercase()
}

/// Download all `meetsetup.xml` files into specified directory.
/// Skips meet on error and prints the error message to `stderr`.
/// # Panics
/// will panic when `meets_directory` doesn't exist and cannot be created
pub fn download_meets(meets_directory: &Path, meet_infos: Vec<MeetInfo>) {
    if !meets_directory.exists() {
        fs::create_dir_all(meets_directory).expect("could not create requested directory");
    }
    let web_client = reqwest::blocking::Client::new();
    for meet_info in meet_infos {
        let meet_path = meets_directory.join(meet_filename(&meet_info));

        let (meet_config_decoded, mut meet_config_file) = match fs::try_exists(&meet_path) {
            Err(err) => {
                // could not check existance of the file. Means we also will not be able to save it
                // there probably
                panic!("{}", err.to_string());
            }
            Ok(true) => {
                println!(
                    "[INFO]: skipping {} beacuse it already exists in the cache directory",
                    meet_info.name
                );
                continue;
            }
            Ok(false) => {
                // fetch the remote config
                let response = match web_client.get(meet_info.config_xml).send() {
                    Ok(response) => response,
                    Err(why) => {
                        eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
                        continue;
                    }
                };
                let meet_config_decoded = match response.text_with_charset("ISO-8859-1") {
                    Ok(string) => string,
                    Err(why) => {
                        eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
                        continue;
                    }
                };

                // create a file
                let meet_config_file = match File::create(&meet_path) {
                    Ok(file) => file,
                    Err(why) => {
                        eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
                        continue;
                    }
                };
                (meet_config_decoded, meet_config_file)
            }
        };

        // write decoded string into file
        if let Err(why) = write!(&mut meet_config_file, "{}", &meet_config_decoded) {
            eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
            continue;
        };
    }
}
