use crate::{NeighborMethod, Particle, ParticleNeighbors};

pub struct BruteForceMethod<'a, T> {
    interaction_range: f64,
    particles: Option<&'a Vec<T>>,
    offsets: Vec<(f64, f64)>,
}

impl<'a, T: Particle> BruteForceMethod<'a, T> {
    pub fn new(interaction_range: f64, length: f64, periodic: bool) -> Self {
        let offsets = if periodic {
            vec![
                (0.0, 0.0),
                (length, 0.0),
                (-length, 0.0),
                (0.0, length),
                (0.0, -length),
                (length, length),
                (-length, length),
                (length, -length),
                (-length, -length),
            ]
        } else {
            vec![(0.0, 0.0)]
        };

        BruteForceMethod {
            interaction_range,
            particles: None,
            offsets,
        }
    }
}

impl<'a, T: Particle> NeighborMethod<'a, T> for BruteForceMethod<'a, T> {
    fn calculate_neighbors(&self) -> Vec<ParticleNeighbors> {
        let particles = match self.particles {
            Some(p) => p,
            None => return vec![],
        };

        let mut neighbors = Vec::with_capacity(particles.len());
        for id in 0..particles.len() {
            neighbors.push(ParticleNeighbors::new(id as u32));
        }

        for particle in particles {
            for other_particle in particles {
                let id = particle.get_id() as usize;
                let other_id = other_particle.get_id() as usize;

                for offset in self.offsets.iter() {
                    if particle.get_id() != other_particle.get_id()
                        && particle.distance_to_neighbor(other_particle, offset)
                            <= self.interaction_range
                    {
                        neighbors[id].insert(other_id as u32);
                        // If A is neighbor to B, B is neighbor to A
                        // We don't check if A is already in B's neighbors as we use a Set
                        neighbors[other_id].insert(id as u32);
                    }
                }
            }
        }

        neighbors
    }

    fn set_particles(&mut self, particles: &'a Vec<T>) {
        self.particles = Some(particles)
    }
}
