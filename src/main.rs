mod args;
mod io;
mod particle;
mod plot;

use std::time::Instant;

use anyhow::{bail, Result};
use clap::Parser;

use args::Cli;
use neighbors::{
    brute_force_method::BruteForceMethod, cell_index_method::CellIndexMethod, NeighborMethod,
};
use particle::CellIndexParticle;

fn main() -> Result<()> {
    let args = Cli::parse();

    println!("Starting io...");

    let start_time = Instant::now();

    let (particles, simulation_area) =
        get_particles(&args.static_input_path, &args.dynamic_input_path)?;

    println!(
        "Finished io... {} µs elapsed",
        start_time.elapsed().as_micros()
    );
    println!("Starting Cell-Index-Method...");
    let start_method_time = Instant::now();

    let end_method_time;

    if args.brute_force {
        println!("Using brute force method");
        let mut method =
            BruteForceMethod::new(args.interaction_range, simulation_area, args.periodic);
        method.set_particles(particles);

        let neighbors = method.calculate_neighbors();

        end_method_time = start_method_time.elapsed().as_micros();

        println!(
            "Finished Brute force method... {} µs elapsed, total {} µs",
            end_method_time,
            start_time.elapsed().as_micros()
        );

        io::output_neighbors(&args.output_path, &neighbors, end_method_time)?;
    } else {
        let mut method = CellIndexMethod::new(
            simulation_area,
            args.m,
            args.interaction_range,
            args.periodic,
        );
        method.set_particles(particles);
        let neighbors = method.calculate_neighbors();

        end_method_time = start_method_time.elapsed().as_micros();

        println!(
            "Finished Cell-Index-Method... {} µs elapsed, total {} µs",
            end_method_time,
            start_time.elapsed().as_micros()
        );

        // Only create graph if an id was requested
        if let Some(id) = args.input_particle {
            plot::plot_cell_index_method(
                &method,
                &neighbors[id as usize],
                &args.output_graph_path,
            )?;
        }
        io::output_neighbors(&args.output_path, &neighbors, end_method_time)?;
    };
    Ok(())
}

pub fn get_particles(
    static_path: &str,
    dynamic_path: &str,
) -> Result<(Vec<CellIndexParticle>, f64)> {
    let (num_particles, simulation_area, particles_radius) = io::read_static_file(static_path)?;
    let particles_coords = io::read_dynamic_file(dynamic_path)?;

    if num_particles as usize != particles_coords.len() {
        panic!(
            "Number of particles in static file does not match number of particles in dynamic file"
        );
    }

    let mut particles: Vec<CellIndexParticle> = Vec::new();
    for i in 0..num_particles {
        let (x, y) = particles_coords[i as usize];
        if x > simulation_area || y > simulation_area {
            bail!(format!(
                "Particle with coordinates ({}, {}) is outside the simulation area of {}",
                x, y, simulation_area
            ));
        }
        let radius = particles_radius[i as usize];
        particles.push(CellIndexParticle::new(i, x, y, radius));
    }
    Ok((particles, simulation_area))
}
