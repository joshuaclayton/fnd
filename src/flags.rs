use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "fnd",
    about = "A tiny command line tool to find file paths based on substring matching or regular expressions",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
pub struct Flags {
    pub query: Option<String>,

    #[structopt(short, long)]
    pub all: bool,

    #[structopt(short, long)]
    pub regex: bool,

    #[structopt(short, long)]
    pub hidden: bool,

    #[structopt(short = "i", long)]
    pub case_insensitive: bool,
}
