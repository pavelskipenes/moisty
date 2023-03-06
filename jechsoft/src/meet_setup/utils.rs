use super::{meet_info::MeetInfo, Entries};
use chrono::{DateTime, Local};
use serde_xml_rs::from_str;
use std::error::Error;
use std::fs::File;
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

/// Download all `meetsetup.xml` files into `meets/` directory.
/// Skips meet on error and prints the error message to `stderr`.
pub fn download_meets(meet_infos: Vec<MeetInfo>) {
    let web_client = reqwest::blocking::Client::new();
    for meet_info in meet_infos {
        // calculate the path
        let mut meet_config_path_string = String::new();
        meet_config_path_string.push_str("meets/");
        meet_config_path_string.push_str(&meet_info.name);
        let mut meet_config_path_string = meet_config_path_string.replace(' ', &'_'.to_string());
        meet_config_path_string.push_str(".xml");
        let meet_config_path_string = meet_config_path_string.to_lowercase();

        let meet_path = Path::new(&meet_config_path_string);

        if !meet_path.exists() {
            // file needs to be downloaded

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
            let mut meet_config_file = match File::create(meet_path) {
                Ok(file) => file,
                Err(why) => {
                    eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
                    continue;
                }
            };

            // write decoded string into file
            if let Err(why) = write!(&mut meet_config_file, "{}", &meet_config_decoded) {
                eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
                continue;
            };
        }
    }
}
