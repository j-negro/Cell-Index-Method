pub mod cell_index_method;

use std::{
    collections::HashSet,
    fmt::Display,
    ops::{Deref, DerefMut},
};

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
