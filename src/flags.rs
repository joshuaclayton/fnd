use crate::SizeComparison;
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

    #[structopt(short, long, allow_hyphen_values = true, verbatim_doc_comment)]
    /// Match files on very close match (+/- 512 bytes), or above or below a certain threshold.
    ///
    /// To run a close match, provide the number and corresponding designation of bytes:
    ///    k  kilobytes (1024 bytes)
    ///    M  megabytes (1024 kilobytes)
    ///    G  gigabytes (1024 megabytes)
    ///    T  terabytes (1024 gigabytes)
    ///    P  petabytes (1024 terabytes)
    ///
    /// For example, to find all roughly 1gb JPGs:
    ///
    ///   fnd "\.jpg$" -r -s 1G
    ///
    /// To select files above or below a particular threshold, prefix with a '+' or '-'.
    ///
    /// For example, to find all CSVs larger than 500mb:
    ///
    ///   fnd "\.csv$" -r -s +500M
    ///
    /// To find all SVGs under 500kb
    ///
    ///   fnd "\.svg$" -r -s -500k
    pub size: Option<SizeComparison>,

    #[structopt(short, long)]
    /// The maximum directory depth when looking for matches
    pub depth: Option<usize>,
}
