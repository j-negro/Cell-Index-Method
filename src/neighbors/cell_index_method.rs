use super::ParticleNeighbors;
use crate::particle::Particle;

#[derive(Debug)]
pub struct CellIndexMethod<'a, T: Particle> {
    length: f64,
    periodic: bool,
    m: usize,
    interaction_range: f64,
    cells: Vec<Vec<&'a T>>,
    num_particles: usize,
}

impl<'a, T: Particle> CellIndexMethod<'a, T> {
    pub fn new(
        length: f64,
        m: Option<usize>,
        interaction_range: f64,
        periodic: bool,
        particles: &'a Vec<T>,
    ) -> Self {
        // TODO: calculate m with algoritm
        let m = m.unwrap_or((length / interaction_range) as usize);
        let mut cells = Vec::with_capacity(m * m);
        for _ in 0..m * m {
            cells.push(vec![]);
        }

        for particle in particles {
            let (x, y) = particle.get_coordinates();
            // NOTE: normalize x and y by m
            let x = (x * m as f64 / length).floor() as usize;
            let y = (y * m as f64 / length).floor() as usize;
            let index: usize = y * m + x;
            if index >= m * m {
                panic!("Particle coordinates out of bounds with simulation area");
            }
            cells[index].push(particle);
        }

        CellIndexMethod {
            length,
            periodic,
            m,
            interaction_range,
            cells,
            num_particles: particles.len(),
        }
    }

    pub fn get_cells(&self) -> &Vec<Vec<&'a T>> {
        &self.cells
    }

    pub fn get_length(&self) -> f64 {
        self.length
    }

    pub fn get_m(&self) -> usize {
        self.m
    }

    fn get_neighboring_cells(&self, cell_idx: usize) -> Vec<((f64, f64), usize)> {
        if self.cells[cell_idx].is_empty() {
            return vec![];
        }

        let x = cell_idx % self.m;
        let y = (cell_idx - x) / self.m;

        let default_offset = (0.0, 0.0);
        let mut neighboring_cells: Vec<((f64, f64), usize)> = vec![(default_offset, cell_idx)];

        if x != self.m - 1 {
            // Derecha
            neighboring_cells.push((default_offset, cell_idx + 1));
            if y != 0 {
                // Abajo derecha
                neighboring_cells.push((default_offset, cell_idx + 1 - self.m));
            }
        } else if self.periodic {
            // Principio de fila
            neighboring_cells.push(((self.length, 0.0), cell_idx + 1 - self.m));
            if y != 0 {
                // Primera columna de la fila de abajo
                neighboring_cells.push(((self.length, 0.0), cell_idx + 1 - 2 * self.m));
            } else {
                // Inferior derecha hacia superior izquierda
                neighboring_cells.push(((self.length, 0.0), cell_idx + 1));
            }
        }

        if y != self.m - 1 {
            // Arriba
            neighboring_cells.push((default_offset, cell_idx + self.m));
            if x != self.m - 1 {
                // Arriba derecha
                neighboring_cells.push((default_offset, cell_idx + self.m + 1));
            }
            if self.periodic && y == 0 {
                if x != self.m - 1 {
                    // Primera fila hacia ultima fila a la derecha
                    neighboring_cells.push(((0.0, -self.length), self.m * (self.m - 1) + x + 1));
                } else {
                    // Primera fila hacia ultima fila primera columna
                    neighboring_cells.push(((self.length, -self.length), self.m * (self.m - 1)));
                }
            }
        } else if self.periodic {
            // Primera fila misma columna
            neighboring_cells.push(((0.0, self.length), x));
            if x != self.m - 1 {
                // Primera fila misma columna a la derecha
                neighboring_cells.push(((0.0, self.length), x + 1));
            } else {
                // Primera celda
                neighboring_cells.push(((self.length, self.length), 0));
            }
        }

        neighboring_cells
    }

    pub fn calculate_neighbors(&self) -> Vec<ParticleNeighbors> {
        let mut neighbors = Vec::with_capacity(self.num_particles);
        for id in 0..self.num_particles {
            neighbors.push(ParticleNeighbors::new(id as u32));
        }

        // For every cell in the simulation area
        for (cell_idx, cell) in self.cells.iter().enumerate() {
            let neighboring_cells = self.get_neighboring_cells(cell_idx);
            // For every particle inside said cell
            for particle in cell {
                let particle_id = particle.get_id() as usize;
                // For neighboring cells, maximum of 5 cells
                for (offset, neighbor_idx) in &neighboring_cells {
                    // For particles inside the neighboring cells
                    for other_particle in &self.cells[*neighbor_idx] {
                        let other_id = other_particle.get_id() as usize;
                        if particle_id == other_id
                            || neighbors[particle_id].contains(&(other_id as u32))
                        {
                            continue;
                        }
                        if particle.distance_to_neighbor(*other_particle, offset)
                            <= self.interaction_range
                        {
                            neighbors[particle_id].insert(other_id as u32);
                            // If A is neighbor to B, B is neighbor to A
                            // We don't check if A is already in B's neighbors as we use a Set
                            neighbors[other_id].insert(particle_id as u32);
                        }
                    }
                }
            }
        }

        neighbors
    }
}
