use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
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

    #[arg(
        long,
        value_name = "date of the meet",
        long_help = "date needs this format: YYYY-MM-DD"
    )]
    pub date: Option<chrono::NaiveDate>,

    /// Path to meetsetup file
    #[arg(
        value_name = "meet setup files",
        long_help = "path to meetsetup file. Usually exported as meetsetup.xml"
    )]
    pub meetsetup_path: Option<String>,
}
