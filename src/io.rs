use std::fs::{self, File};
use std::io::Write;

use crate::neighbors::ParticleNeighbors;
use anyhow::Result;
use simple_error::SimpleError;

pub fn read_static_file(path: &str) -> Result<(u32, f64, Vec<f64>)> {
    let contents = fs::read_to_string(path)?;
    let mut lines = contents.lines();

    let num_particles = lines
        .next()
        .ok_or(SimpleError::new(format!(
            "Invalid file format for file {}",
            path
        )))?
        .trim()
        .parse()?;
    let simulation_area: f64 = lines
        .next()
        .ok_or(SimpleError::new(format!(
            "Invalid file format for file {}",
            path
        )))?
        .trim()
        .parse()?;
    let particles_radius: Vec<f64> = lines
        .filter_map(|line| line.split_whitespace().next()?.parse::<f64>().ok())
        .collect();

    Ok((num_particles, simulation_area, particles_radius))
}

pub fn read_dynamic_file(path: &str) -> Result<Vec<(f64, f64)>> {
    let contents = fs::read_to_string(path)?;
    let mut lines = contents.lines();

    // NOTE: Skip the initial time data
    lines.next();

    let mut particles: Vec<(f64, f64)> = Vec::new();
    for line in lines {
        if line == "t1" {
            break;
        }
        let mut coords = line.split_whitespace();
        let x = coords
            .next()
            .ok_or(SimpleError::new(format!(
                "Invalid file format for file {}",
                path
            )))?
            .parse()?;
        let y = coords
            .next()
            .ok_or(SimpleError::new(format!(
                "Invalid file format for file {}",
                path
            )))?
            .parse()?;
        particles.push((x, y));
    }
    Ok(particles)
}

pub fn output_neighbors(path: &str, neighbors: &Vec<ParticleNeighbors>) -> Result<()> {
    let mut output = File::create(path)?;
    for particle_neighbors in neighbors {
        writeln!(output, "{}", particle_neighbors)?;
    }
    Ok(())
}
