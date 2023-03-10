mod args;
mod io;
mod particle;
mod plot;

use args::Cli;
use clap::Parser;
use particle::{CellIndexMethod, Particle};

fn main() {
    let args = Cli::parse();

    let (particles, simulation_area) =
        get_particles(&args.static_input_path, &args.dynamic_input_path);

    Particle::new(1, 2.0, 3.0, 4.0);

    let area = CellIndexMethod::new(simulation_area, 5, 1.0, false, &particles);

    dbg!(area);
}

pub fn get_particles(static_path: &str, dynamic_path: &str) -> (Vec<Particle>, f64) {
    let (num_particles, simulation_area, particles_radius) = io::read_static_file(static_path);
    let particles_coords = io::read_dynamic_file(dynamic_path);

    if num_particles as usize != particles_coords.len() {
        panic!(
            "Number of particles in static file does not match number of particles in dynamic file"
        );
    }

    let mut particles: Vec<Particle> = Vec::new();
    for i in 0..num_particles {
        let (x, y) = particles_coords[i as usize];
        let radius = particles_radius[i as usize];
        particles.push(Particle::new(i, x, y, radius));
    }
    (particles, simulation_area)
}
