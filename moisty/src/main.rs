#![feature(error_iter)]

extern crate chrono;
extern crate clap;
extern crate colored;
extern crate directories;
extern crate jechsoft;
extern crate tabled;
extern crate url;

mod cli;
// mod validators;

use crate::clap::Parser;
use crate::cli::Cli;
use chrono::Local;
use directories::BaseDirs;
use jechsoft::medley::utils::{download_meets, get_meet_list};
use jechsoft::meet_setup::meet::Meet;
use std::fs;
use std::{io, path::PathBuf};
use tabled::{builder::Builder, settings::Style};

// TODO: download meet files into one directory and move them if parsing is successful.
// TODO: auto complete on command line the parsed meets?
fn main() -> io::Result<()> {
    colog::init();
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
        let search_date_start = cli.date.unwrap_or(Local::now().naive_local().date());
        match get_meet_list(search_date_start) {
            Ok(meets_to_download) => download_meets(&download_dir, meets_to_download),
            Err(why) => panic!("{why}"),
        };
    }

    let meet_setup_paths = match cli.meetsetup_path {
        Some(path_meetsetup_file) => {
            vec![PathBuf::from(path_meetsetup_file)]
        }
        None => fs::read_dir(&download_dir)?
            .filter_map(Result::ok)
            .filter(|entry| entry.path().is_file())
            .map(|entry| entry.path())
            .collect::<Vec<_>>(),
    };

    let (meets, failed): (Vec<_>, Vec<_>) = meet_setup_paths
        .into_iter()
        .map(|meet_setup_file| Meet::try_from(&meet_setup_file))
        .partition(Result::is_ok);

    let meets: Vec<_> = meets.into_iter().map(Result::unwrap).collect();
    let failed: Vec<_> = failed.into_iter().map(Result::unwrap_err).collect();

    let meets_count = meets.len();
    let failed_count = failed.len();

    log::info!(
        "parsed {} out of {} meets",
        meets_count,
        meets_count + failed_count
    );

    for fail in failed {
        log::error!("{fail}");
    }

    for meet in meets {
        if cli.list {
            let mut header_builder = Builder::default();
            header_builder.push_record::<&[String; 3]>(&[
                "NSF meet id".into(),
                "Date".into(),
                "Name".into(),
            ]);

            if meet.date_start.is_some() && meet.date_end.is_some() {
                println!(
                    "[{:0>10}] [{} {}] {}",
                    meet.nsf_meet_id.unwrap_or(0),
                    meet.date_start.unwrap(),
                    meet.date_end.unwrap(),
                    meet.name
                )
            } else {
                println!(
                    "[{:0>10}] {}, {}",
                    meet.nsf_meet_id.unwrap_or(0),
                    meet.date,
                    meet.name
                )
            }
        }

        if cli.table {
            let mut header_builder = Builder::default();
            header_builder.push_record::<[&str; 5]>([
                "Meet name",
                "Date",
                "Sessions",
                "Location",
                "NSF id",
            ]);
            header_builder.push_record(&[
                meet.name,
                meet.date,
                meet.sessions.len().to_string(),
                meet.location,
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
