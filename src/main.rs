#[cfg(all(unix, not(target_env = "musl")))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use fnd::cli;

fn main() -> cli::Result<()> {
    cli::run()
}
