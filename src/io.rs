use std::fs;

use crate::particle::Particle;

const STATIC_FILE_PATH: &str = "static.txt";
const DINAMIC_FILE_PATH: &str = "dinamic.txt";

fn read_static_file() -> (u32, f64, Vec<f64>) {
    let contents = fs::read_to_string(STATIC_FILE_PATH).expect("Unable to read static file");
    let mut lines = contents.lines();

    let num_particles = lines.next().unwrap().parse::<u32>().unwrap();
    let simulation_area: f64 = lines.next().unwrap().parse::<f64>().unwrap();
    let particles_radius: Vec<f64> = lines.map(|line| line.parse::<f64>().unwrap()).collect();

    (num_particles, simulation_area, particles_radius)
}

fn read_dinamic_file() -> Vec<(f64, f64)> {
    let contents = fs::read_to_string(DINAMIC_FILE_PATH).expect("Unable to read dinamic file");
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

pub fn get_particles() -> (Vec<Particle>, f64) {
    let (num_particles, simulation_area, particles_radius) = read_static_file();
    let particles_coords = read_dinamic_file();

    if num_particles as usize != particles_coords.len() {
        panic!(
            "Number of particles in static file does not match number of particles in dinamic file"
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
