use crate::{Check, Flags};
use anyhow::Context;
pub use anyhow::Result;
use ignore::{WalkBuilder, WalkParallel, WalkState};
use structopt::StructOpt;

pub fn run() -> Result<()> {
    let flags = Flags::from_args();
    let check = Check::new(&flags).context("Unable to build path check")?;

    build_walk_parallel(&flags).run(|| {
        Box::new(|result| {
            if let Some(entry) = result.ok() {
                let path = entry.path();
                if check.check(&path.to_path_buf(), path.metadata().ok()) {
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

    builder.max_depth(flags.depth);

    builder.threads(20).build_parallel()
}
