mod args;
mod io;
mod neighbors;
mod particle;
mod plot;

use std::time::Instant;

use clap::Parser;

use args::Cli;
use neighbors::cell_index_method::CellIndexMethod;
use particle::Particle;

fn main() {
    let args = Cli::parse();

    println!("Starting io...");

    let start_time = Instant::now();

    let (particles, simulation_area) =
        get_particles(&args.static_input_path, &args.dynamic_input_path);

    println!(
        "Finished io... {} µs elapsed",
        start_time.elapsed().as_micros()
    );
    println!("Starting Cell-Index-Method...");
    let start_method_time = Instant::now();

    let area = CellIndexMethod::new(simulation_area, 5, 1.0, false, &particles);
    let neighbors = area.calculate_neighbors();

    println!(
        "Finished Cell-Index-Method... {} µs elapsed, total {} µs",
        start_method_time.elapsed().as_micros(),
        start_time.elapsed().as_micros()
    );

    io::output_neighbors(&args.output_path, &neighbors);
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
