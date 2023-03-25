pub mod brute_force_method;
pub mod cell_index_method;

use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Deref, DerefMut},
};

pub trait Particle {
    fn distance_to_neighbor(&self, other: &dyn Particle, offset: &(f64, f64)) -> f64 {
        let (x, y) = self.get_coordinates();
        let (other_x, other_y) = other.get_coordinates();
        let dx = x - other_x - offset.0;
        let dy = y - other_y - offset.1;
        (dx * dx + dy * dy).sqrt() - self.get_radius() - other.get_radius()
    }
    fn get_coordinates(&self) -> (f64, f64);
    fn get_id(&self) -> u32;
    fn get_radius(&self) -> f64;
}

#[derive(Debug)]
pub struct ParticleNeighbors(u32, HashSet<u32>);

impl ParticleNeighbors {
    pub fn new(id: u32) -> Self {
        ParticleNeighbors(id, HashSet::new())
    }

    pub fn get_particle_id(&self) -> u32 {
        self.0
    }
}

impl Display for ParticleNeighbors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\t\t", self.0)?;
        for id in self.1.iter() {
            write!(f, "{id} ")?
        }
        Ok(())
    }
}

impl Deref for ParticleNeighbors {
    type Target = HashSet<u32>;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl DerefMut for ParticleNeighbors {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.1
    }
}

trait NeighborMethod<'a, T: Particle> {
    fn calculate_neighbors(&self) -> Vec<ParticleNeighbors>;
    fn set_particles(&mut self, particles: &'a Vec<T>);
}
