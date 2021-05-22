use crate::{Flags, PathCheck};
use anyhow::Context;
pub use anyhow::Result;
use ignore::{WalkBuilder, WalkParallel, WalkState};
use structopt::StructOpt;

pub fn run() -> Result<()> {
    let flags = Flags::from_args();
    let check = PathCheck::new(&flags).context("Unable to build path check")?;

    build_walk_parallel(&flags).run(|| {
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

fn build_walk_parallel(flags: &Flags) -> WalkParallel {
    let mut builder = WalkBuilder::new("./");

    if flags.all {
        builder.git_ignore(false);
        builder.hidden(false);
    }

    if flags.hidden {
        builder.hidden(false);
    }

    builder.threads(20).build_parallel()
}
