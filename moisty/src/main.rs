use chrono::Local;
use jechsoft::meet_setup::{
    meet::Meet,
    utils::{download, get_meet_list},
};
use std::{fs, path::Path};

fn main() {
    let meets_dir = Path::new("meets");

    if let Err(why) = fs::create_dir_all(meets_dir) {
        panic!("{}", why);
    };

    let date = Local::now();
    match get_meet_list(date) {
        Ok(meets) => download(meets),
        Err(why) => panic!("{}", why.to_string()),
    };

    let paths = fs::read_dir(meets_dir);
    let paths = match paths {
        Ok(paths) => paths,
        Err(why) => panic!("{why}"),
    };
    for path in paths {
        let path = match path {
            Ok(path) => path,
            Err(why) => panic!("{why}"),
        };

        let _meet = match Meet::from(&path.path()) {
            Ok(meet) => meet,
            Err(why) => {
                eprintln!("[ERROR]: [{}] {}", &path.path().display(), why);
                continue;
            }
        };
    }
}
