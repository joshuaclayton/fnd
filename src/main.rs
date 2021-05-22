#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use anyhow::{Context, Result};
use ignore::{WalkBuilder, WalkState};
use regex::{Regex, RegexBuilder};
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "fnd",
    about = "A tiny command line tool to find file paths based on substring matching or regular expressions",
    setting = structopt::clap::AppSettings::ColoredHelp
)]
struct Flags {
    query: Option<String>,

    #[structopt(short, long)]
    all: bool,

    #[structopt(short, long)]
    regex: bool,

    #[structopt(short, long)]
    hidden: bool,

    #[structopt(short = "i", long)]
    case_insensitive: bool,
}

enum PathCheck<'a> {
    Allow,
    CheckRegex(Regex),
    CheckStr(&'a str),
    CheckCaseInsensitiveStr(&'a str),
}

impl<'a> PathCheck<'a> {
    fn check(&self, path: &Path) -> bool {
        match self {
            PathCheck::Allow => true,
            PathCheck::CheckStr(query) => path.to_string_lossy().contains(query),
            PathCheck::CheckCaseInsensitiveStr(query) => path
                .to_string_lossy()
                .to_lowercase()
                .contains(&query.to_lowercase()),
            PathCheck::CheckRegex(regex) => regex.is_match(&path.to_string_lossy()),
        }
    }

    fn new(flags: &'a Flags) -> Result<Self, regex::Error> {
        if let Some(ref query) = flags.query {
            if flags.regex {
                RegexBuilder::new(&query)
                    .case_insensitive(flags.case_insensitive)
                    .build()
                    .map(|v| PathCheck::CheckRegex(v))
            } else {
                if query == "." {
                    Ok(PathCheck::Allow)
                } else {
                    if flags.case_insensitive {
                        Ok(PathCheck::CheckCaseInsensitiveStr(&query))
                    } else {
                        Ok(PathCheck::CheckStr(&query))
                    }
                }
            }
        } else {
            Ok(PathCheck::Allow)
        }
    }
}

fn main() -> Result<()> {
    let flags = Flags::from_args();
    let check = PathCheck::new(&flags).context("Unable to build path check")?;

    let mut builder = WalkBuilder::new("./");

    if flags.all {
        builder.git_ignore(false);
        builder.hidden(false);
    }

    if flags.hidden {
        builder.hidden(false);
    }

    builder.threads(20).build_parallel().run(|| {
        Box::new(|result| {
            if let Some(path) = result.ok().map(|entry| entry.path().to_path_buf()) {
                if check.check(&path) {
                    println!("{}", path.to_string_lossy());
                }
            }

            WalkState::Continue
        })
    });

    Ok(())
}
