#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use ignore::{WalkBuilder, WalkState};
use regex::Regex;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Flags {
    query: String,

    #[structopt(short, long)]
    all: bool,

    #[structopt(short, long)]
    regex: bool,
}

fn main() {
    let opts = Flags::from_args();

    let mut builder = WalkBuilder::new("./");
    if opts.all {
        builder.git_ignore(false);
    }

    let mut re = None;
    if opts.regex {
        re = Regex::new(&opts.query).ok();
    }

    builder.threads(20).build_parallel().run(|| {
        Box::new(|result| {
            if let Some(path) = result.ok().map(|entry| entry.path().to_path_buf()) {
                if let Some(regex) = &re {
                    if regex.is_match(&path.to_string_lossy()) {
                        println!("{}", path.to_string_lossy());
                    }
                } else {
                    if path.to_string_lossy().contains(&opts.query) {
                        println!("{}", path.to_string_lossy());
                    }
                }
            }

            WalkState::Continue
        })
    });
}
