extern crate chrono;
extern crate encoding;
extern crate serde_xml_rs;

use self::chrono::{DateTime, Local};
use self::encoding::all::ISO_8859_1;
use self::encoding::Encoding;
use self::serde_xml_rs::from_str;
use super::{meet_info::MeetInfo, Entries};
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

    let response = reqwest::blocking::get(meets_url)?.text_with_charset("ISO-8859-1")?;

    let tmp: Entries = from_str(&response)?;

    Ok(tmp.meet_setup_entries)
}

/// Download all `meetsetup.xml` files into specified directory.
/// Skips meet on error and prints the error message to `stderr`.
/// # Panics
/// will panic when unable to create a cache directory
pub fn download_meets(meets_directory: &Path, meet_infos: Vec<MeetInfo>) {
    if !meets_directory.exists() {
        fs::create_dir_all(meets_directory).expect("could not create requested directory");
    }
    let web_client = reqwest::blocking::Client::new();
    for meet_info in meet_infos {
        let meet_path = meets_directory.join(&meet_info.get_filename());

        match fs::try_exists(&meet_path) {
            Err(why) => {
                eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
                continue;
            }
            Ok(true) => {
                println!(
                    "[DEBUG]: skipping {} beacuse it already exists in the cache directory",
                    meet_info.name
                );
                continue;
            }
            Ok(false) => {
                const XML_ENCODING_HEADER_ISO_5589_1: &str =
                    r#"<?xml version="1.0" encoding="ISO-8859-1" ?>"#;
                const XML_ENCODING_HEADER_UTF8: &str = r#"<?xml version="1.0" encoding="UTF-8" ?>"#;

                // fetch the remote meet_setup.xml
                let response = match web_client.get(meet_info.meet_setup).send() {
                    Ok(response) => response,
                    Err(why) => {
                        eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
                        continue;
                    }
                };

                // TODO: we receive last modification date on the downloaded file.
                // Can we get this information without downloading the full file?
                // dbg!(&response.headers().get("last-modified"));

                let content = response.bytes();
                let content = match &content {
                    Ok(content) => content,
                    Err(why) => {
                        eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
                        continue;
                    }
                };

                let content = match ISO_8859_1.decode(content, encoding::DecoderTrap::Strict) {
                    Ok(content) => content,
                    Err(why) => {
                        eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
                        continue;
                    }
                };

                let content: String =
                    content.replace(XML_ENCODING_HEADER_ISO_5589_1, XML_ENCODING_HEADER_UTF8);

                // write to file
                let mut meet_config_file = match File::create(&meet_path) {
                    Ok(file) => file,
                    Err(why) => {
                        eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
                        continue;
                    }
                };

                if let Err(why) = meet_config_file.write(content.as_bytes()) {
                    eprintln!("[ERROR]: [{}] {}", &meet_info.name, why);
                    continue;
                };
            }
        };
    }
}
