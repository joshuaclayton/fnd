#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use ignore::{WalkBuilder, WalkState};
use regex::Regex;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Flags {
    query: Option<String>,

    #[structopt(short, long)]
    all: bool,

    #[structopt(short, long)]
    regex: bool,
}

enum PathCheck<'a> {
    Allow,
    CheckRegex(Regex),
    CheckStr(&'a str),
}

impl<'a> PathCheck<'a> {
    fn check(&self, path: &Path) -> bool {
        match self {
            PathCheck::Allow => true,
            PathCheck::CheckStr(query) => path.to_string_lossy().contains(query),
            PathCheck::CheckRegex(regex) => regex.is_match(&path.to_string_lossy()),
        }
    }

    fn new(flags: &'a Flags) -> Self {
        if let Some(ref query) = flags.query {
            if flags.regex {
                if let Some(re) = Regex::new(&query).ok() {
                    PathCheck::CheckRegex(re)
                } else {
                    PathCheck::CheckStr(&query)
                }
            } else {
                if query == "." {
                    PathCheck::Allow
                } else {
                    PathCheck::CheckStr(&query)
                }
            }
        } else {
            PathCheck::Allow
        }
    }
}

fn main() {
    let flags = Flags::from_args();
    let mut builder = WalkBuilder::new("./");

    if flags.all {
        builder.git_ignore(false);
    }

    let check = PathCheck::new(&flags);

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
}
