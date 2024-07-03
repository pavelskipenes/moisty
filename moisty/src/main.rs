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
use jechsoft::medley::utils::{download_meets, get_meet_list};
use jechsoft::meet_setup::meet::Meet;
use std::fs;
use std::{io, path::PathBuf};
use tabled::{builder::Builder, settings::Style};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Download latest meets from medley.no
    #[arg(
        short,
        long,
        value_name = "download new meets from server and cache them",
        default_value_t = false
    )]
    pub download: bool,

    /// Output table of events from meets
    #[arg(
        short,
        long,
        value_name = "print short info about each event in the meet",
        default_value_t = false
    )]
    pub info: bool,

    /// clear out cache directory of saved meets
    #[arg(
        short,
        long,
        value_name = "clear cached meets",
        default_value_t = false
    )]
    pub clear_cache: bool,

    #[arg(
        short,
        long,
        value_name = "list available meets",
        long_help = "lists all the meets that are avaialble locally",
        default_value_t = false
    )]
    pub list: bool,

    /// Path to meetsetup file
    #[arg(
        value_name = "meet setup files",
        long_help = "path to meetsetup file. Usually exported as meetsetup.xml"
    )]
    pub meetsetup_path: Option<String>,
}

// TODO: download meet files into one directory and move them if parsing is successful.
// TODO: auto complete on command line the parsed meets?
fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let base_dir = BaseDirs::new();
    let cache_dir = match base_dir {
        None => unimplemented!("cannot deal with system without configured cache directory"),
        Some(base_dir) => base_dir.cache_dir().join("moisty/meets"),
    };
    let download_dir = cache_dir.join("downloads");
    let parsed_dir = cache_dir.join("parsed");

    if cli.clear_cache {
        fs::remove_dir_all(&cache_dir)?;
    }
    fs::create_dir_all(&cache_dir).unwrap();
    fs::create_dir_all(&download_dir).unwrap();
    fs::create_dir_all(parsed_dir).unwrap();

    if cli.download {
        let date = Local::now();
        match get_meet_list(date) {
            Ok(meets_to_download) => download_meets(&download_dir, meets_to_download),
            Err(why) => eprintln!("{}", why),
        };
    }

    let meets = match cli.meetsetup_path {
        Some(path_meetsetup_file) => {
            vec![PathBuf::from(path_meetsetup_file)]
        }
        None => fs::read_dir(&download_dir)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path())
            .collect::<Vec<_>>(),
    };

    for meet_setup_file in &meets {
        // if parse downloded meets
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
                continue;
            }
        };

        if cli.list {
            match meet.nsf_meet_id {
                Some(nsf_meet_id) => {
                    println!("[{:0>10}] {}, {}", nsf_meet_id, meet.name, meet.date)
                }
                None => println!("[MISSING] {}, {}", meet.name, meet.date),
            };
        }

        if cli.info {
            let mut header_builder = Builder::default();
            header_builder.push_record::<&[String; 4]>(&[
                "Meet name".into(),
                "Date".into(),
                "Sessions".into(),
                "NSF id".into(),
            ]);
            header_builder.push_record(&[
                meet.name,
                meet.date,
                meet.sessions.len().to_string(),
                match meet.nsf_meet_id {
                    Some(nsf_meet_id) => format!("{:0>10}", nsf_meet_id),
                    None => "".to_string(),
                },
            ]);
            let mut table = header_builder.build();
            table.with(Style::rounded());
            println!("{}", table);

            let mut builder = Builder::default();
            builder.push_record([
                "Event", "Distance", "Style", "Gender", "Date", "Sorting", //"Description",
            ]);
            for event in meet.events {
                let row = [
                    event.id.to_string(),
                    event.distance.to_string(),
                    event.style.to_string(),
                    event.gender_group.to_string(),
                    event.date.to_string(),
                    event.sorting.to_string(),
                    //event.description.to_string(),
                ];
                builder.push_record(row);
            }
            let table = builder.build().with(Style::rounded()).to_string();
            println!("{table}");
        }
    }
    Ok(())
}
