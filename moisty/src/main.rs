use chrono::Local;
use clap::Parser;
use colored::Colorize;
use jechsoft::meet_setup::{
    meet::Meet,
    utils::{download_meets, get_meet_list},
};
use std::{fs, path::Path};
use std::{io, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "path to meetsetup.xml to parse")]
    pub meetsetup_path: Option<String>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let meets_dir = Path::new("assets/meets");
    fs::create_dir_all(meets_dir)?;

    match cli.meetsetup_path.as_deref() {
        Some(path) => {
            // load spesified meet
            let meetsetup = PathBuf::from(path);
            match Meet::from(&meetsetup) {
                Ok(meet) => {
                    dbg!(&meet);
                }
                Err(why) => eprintln!("[ERROR]: [{}] {why}", &meetsetup.as_path().display()),
            };
        }
        None => {
            // download meets
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
                        eprintln!(
                            "[{}][{}]: {why}",
                            "ERROR".red(),
                            &path.path().to_str().unwrap().green()
                        );
                        continue;
                    }
                };
                // dbg!(meet);
            }
        }
    }

    Ok(())
}
