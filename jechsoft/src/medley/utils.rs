extern crate chrono;
extern crate encoding;
extern crate serde_xml_rs;
extern crate log;

use super::{meet_info::MeetInfo, Entries};

use self::chrono::NaiveDate;
use self::encoding::all::ISO_8859_1;
use self::encoding::Encoding;
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

pub fn get_meet_list(search_date_start: NaiveDate) -> Result<Vec<MeetInfo>, Box<dyn Error>> {
    let meets_url = "http://medley.no/tidsjekk/stevneoppsett.asmx/VisStevneoppsett?FraNr=1&FraDato="
        .to_owned() + &search_date_start.format("%Y%m%d").to_string();

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
        let meet_path = meets_directory.join(meet_info.get_filename());

        match fs::exists(&meet_path) {
            Err(why) => {
                panic!("{why}");
            }
            Ok(true) => {
                log::debug!(
                    "skipping {} {} beacuse it already exists in the cache directory",
                    meet_info.name, meet_info.id
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
                        log::error!("[{}] {}", &meet_info.name, why);
                        continue;
                    }
                };

                let content = response.bytes();
                let content = match &content {
                    Ok(content) => content,
                    Err(why) => {
                        log::error!("[{}] {}", &meet_info.name, why);
                        continue;
                    }
                };

                let content = match ISO_8859_1.decode(content, encoding::DecoderTrap::Strict) {
                    Ok(content) => content,
                    Err(why) => {
                        log::error!("[{}] {}", &meet_info.name, why);
                        continue;
                    }
                };

                let content: String =
                    content.replace(XML_ENCODING_HEADER_ISO_5589_1, XML_ENCODING_HEADER_UTF8);

                // write to file
                let mut meet_config_file = match File::create(&meet_path) {
                    Ok(file) => file,
                    Err(why) => {
                        log::error!("[{}] {} {}", &meet_info.name, why, &meet_path.display());
                        continue;
                    }
                };

                if let Err(why) = meet_config_file.write(content.as_bytes()) {
                    log::error!("[{}] {}", &meet_info.name, why);
                    continue;
                };
            }
        };
    }
}
