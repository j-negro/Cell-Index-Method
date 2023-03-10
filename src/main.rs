mod args;
mod particle;

use args::Cli;
use clap::Parser;
use particle::Particle;

fn main() {
    let args = Cli::parse();

    dbg!(args);

    Particle::new(1, 2.0, 3.0, 4.0);
}
