use std::fs;

use crate::particle::Particle;

pub fn read_static_file(path: &str) -> (u32, f64, Vec<f64>) {
    let contents = fs::read_to_string(path).expect("Unable to read static file");
    let mut lines = contents.lines();

    let num_particles = lines.next().unwrap().parse::<u32>().unwrap();
    let simulation_area: f64 = lines.next().unwrap().parse::<f64>().unwrap();
    let particles_radius: Vec<f64> = lines.map(|line| line.parse::<f64>().unwrap()).collect();

    (num_particles, simulation_area, particles_radius)
}

pub fn read_dynamic_file(path: &str) -> Vec<(f64, f64)> {
    let contents = fs::read_to_string(path).expect("Unable to read dynamic file");
    let mut lines = contents.lines();

    // NOTE: Skip to the initial time data
    while lines.next().unwrap() != "t0" {}

    let mut particles: Vec<(f64, f64)> = Vec::new();
    for line in lines {
        if line == "t1" {
            break;
        }
        let mut coords = line.split_whitespace();
        let x = coords.next().unwrap().parse::<f64>().unwrap();
        let y = coords.next().unwrap().parse::<f64>().unwrap();
        particles.push((x, y));
    }
    particles
}
