use chrono::Local;
use jechsoft::meet_setup::{
    meet::Meet,
    utils::{download_meets, get_meet_list},
};
use std::io;
use std::{fs, path::Path};

fn main() -> io::Result<()> {
    let meets_dir = Path::new("assets/meets");

    fs::create_dir_all(meets_dir)?;

    let date = Local::now();
    match get_meet_list(date) {
        Ok(meets_to_download) => download_meets(meets_to_download),
        Err(why) => panic!("{}", why.to_string()),
    };

    let dir_entries = fs::read_dir(meets_dir)?;
    for meetsetup_path in dir_entries {
        let path = meetsetup_path?;
        let _meet = match Meet::from(&path.path()) {
            Ok(meet) => meet,
            Err(why) => {
                eprintln!("[ERROR]: [{}] {}", &path.path().display(), why);
                continue;
            }
        };
    }
    Ok(())
}
