extern crate chrono;
extern crate clap;
extern crate colored;
extern crate directories;
extern crate jechsoft;
extern crate tabled;

use chrono::Local;
use clap::Parser;
use colored::Colorize;
use directories::BaseDirs;
use jechsoft::meet_setup::{
    meet::Meet,
    utils::{download_meets, get_meet_list},
};
use std::fs;
use std::{io, path::PathBuf};
use tabled::{builder::Builder, settings::Style};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "path to `meetsetup.xml` to parse")]
    pub meetsetup_path: Option<String>,
    #[arg(
        short,
        long,
        value_name = "download new meets from server and cache them",
        default_value_t = false
    )]
    pub download: bool,

    #[arg(
        short,
        long,
        value_name = "print short info about each event in the meet",
        default_value_t = false
    )]
    pub info: bool,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let base_dir = BaseDirs::new();
    let meets_dir = match base_dir {
        Some(base_dir) => base_dir.cache_dir().join("moisty/meets"),
        None => todo!("cannot deal with system without configured cache directory"),
    };

    fs::create_dir_all(&meets_dir)?;

    if cli.download {
        // how do we know if there has been any updates? Do they need to be redownloaded each time?
        // invalidate the files if they are older than X hours?
        let date = Local::now();
        match get_meet_list(date) {
            Ok(meets_to_download) => download_meets(&meets_dir, meets_to_download),
            Err(why) => eprintln!("{}", why),
        };
    }

    let meets = match cli.meetsetup_path {
        Some(path_meetsetup_file) => {
            vec![PathBuf::from(path_meetsetup_file)]
        }
        None => fs::read_dir(meets_dir)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path())
            .collect::<Vec<_>>(),
    };

    let mut error_count = 0;
    for meet_setup_file in &meets {
        let meet = match Meet::try_from(meet_setup_file) {
            Ok(meet) => meet,
            Err(why) => {
                eprintln!(
                    "[{}][{}]: {why}",
                    "ERROR".red(),
                    &meet_setup_file
                        .file_name()
                        .unwrap()
                        .to_string_lossy()
                        .green()
                );
                error_count += 1;
                continue;
            }
        };

        if cli.info {
            let mut builder = Builder::default();
            builder.push_record([
                "id",
                "distance",
                "style",
                "gender_group",
                "date",
                //"description",
            ]);
            for event in meet.events {
                let row = [
                    event.id.to_string(),
                    event.distance.to_string(),
                    event.style.to_string(),
                    event.gender_group.to_string(),
                    event.date.to_string(),
                    //event.description.to_string(),
                ];
                builder.push_record(row);
            }
            let table = builder.build().with(Style::rounded()).to_string();
            println!("{table}");
        }
    }
    println!(
        "[{}]: {}/{} successfully meet files parsed",
        "INFO".green(),
        meets.len() - error_count,
        meets.len()
    );
    Ok(())
}
