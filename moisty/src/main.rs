use chrono::Local;
use clap::Parser;
use colored::Colorize;
use jechsoft::meet_setup::{
    meet::Meet,
    utils::{download_meets, get_meet_list},
};
use std::{fs, iter::once, path::Path};
use std::{io, path::PathBuf};

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
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    // create meets directory
    let meets_dir = Path::new("assets/meets");
    fs::create_dir_all(meets_dir)?;

    // download only those we don't have
    if cli.download {
        let date = Local::now();
        match get_meet_list(date) {
            Ok(meets_to_download) => download_meets(meets_to_download),
            Err(why) => eprintln!("{}", why),
        };
    }

    // this mess tries to return an iterator over files in a directory. If the user passes a file explicitly this should return an iterator also. That would make the next block simpler as it can take in an iterator of files, no matter if there are one or many entries.
    let meets = match cli.meetsetup_path.as_deref() {
        Some(path_meetsetup_file) => {
            // return file wrapped inside an iterator to match the signature of the next return block
            Box::new(once(PathBuf::from(path_meetsetup_file))) as Box<dyn Iterator<Item = PathBuf>>
        }
        None => {
            let files = fs::read_dir(meets_dir)?
                .filter_map(Result::ok)
                .filter(|entry| entry.path().is_file())
                .map(|entry| entry.path())
                .collect::<Vec<_>>();
            Box::new(files.into_iter()) as Box<dyn Iterator<Item = PathBuf>>
        }
    };

    for file in meets {
        let _meet = match Meet::try_from(&file) {
            Ok(meet) => meet,
            Err(why) => {
                eprintln!(
                    "[{}][{}]: {why}",
                    "ERROR".red(),
                    &file.file_name().unwrap().to_string_lossy().green()
                );
                continue;
            }
        };
        // dbg!(_meet);
    }
    Ok(())
}
