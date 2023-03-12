pub mod cell_index_method;

use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Deref, DerefMut},
};

use crate::particle::Particle;

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
        write!(f, "[ {}\t\t", self.0)?;
        for id in self.1.iter() {
            write!(f, "{id} ")?
        }
        writeln!(f, "]")?;
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

pub fn brute_force_method(
    interaction_range: f64,
    particles: &Vec<Particle>,
) -> Vec<ParticleNeighbors> {
    let mut neighbors = Vec::with_capacity(particles.len());
    for id in 0..particles.len() {
        neighbors.push(ParticleNeighbors::new(id as u32));
    }

    for particle in particles {
        for other_particle in particles {
            let id = particle.get_id() as usize;
            let other_id = other_particle.get_id() as usize;
            if particle.get_id() != other_particle.get_id()
                && particle.distance_to_neighbor(other_particle) <= interaction_range
            {
                neighbors[id].insert(other_id as u32);
                // If A is neighbor to B, B is neighbor to A
                // We don't check if A is already in B's neighbors as we use a Set
                neighbors[other_id].insert(id as u32);
            }
        }
    }

    neighbors
}
