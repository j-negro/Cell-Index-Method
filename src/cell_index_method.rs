use std::collections::HashSet;

use crate::particle::Particle;

#[derive(Debug)]
pub struct CellIndexMethod<'a> {
    length: f64,
    periodic: bool,
    m: usize,
    interaction_range: f64,
    cells: Vec<Vec<&'a Particle>>,
    num_particles: usize,
}

impl<'a> CellIndexMethod<'a> {
    pub fn new(
        length: f64,
        m: usize,
        interaction_range: f64,
        periodic: bool,
        particles: &'a Vec<Particle>,
    ) -> Self {
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

    pub fn get_cells(&self) -> &Vec<Vec<&'a Particle>> {
        &self.cells
    }

    pub fn get_length(&self) -> f64 {
        self.length
    }

    pub fn get_m(&self) -> usize {
        self.m
    }

    fn get_neighboring_cells(&self, cell_idx: usize) -> Vec<usize> {
        let mut neighboring_cells = vec![cell_idx];

        let x = cell_idx % self.m;
        let y = (cell_idx - x) / self.m;

        if x != self.m - 1 {
            // Derecha
            neighboring_cells.push(cell_idx + 1);
            if y != 0 {
                // Abajo derecha
                neighboring_cells.push(cell_idx + 1 - self.m);
            }
        } else if self.periodic {
            // Principio de fila
            neighboring_cells.push(cell_idx + 1 - self.m);
            if y != 0 {
                // Primera columna de la fila de abajo
                neighboring_cells.push(cell_idx + 1 - 2 * self.m);
            } else {
                // Inferior derecha hacia superior izquierda
                neighboring_cells.push(cell_idx + 1);
            }
        }

        if y != self.m - 1 {
            // Arriba
            neighboring_cells.push(cell_idx + self.m);
            if x != self.m - 1 {
                // Arriba derecha
                neighboring_cells.push(cell_idx + self.m + 1);
            }
            if self.periodic && y == 0 {
                if x != self.m - 1 {
                    // Primera fila hacia ultima fila a la derecha
                    neighboring_cells.push(self.m * (self.m - 1) + x + 1);
                } else {
                    // Primera fila hacia ultima fila primera columna
                    neighboring_cells.push(self.m * (self.m - 1));
                }
            }
        } else if self.periodic {
            // Primera fila misma columna
            neighboring_cells.push(x);
            if x != self.m - 1 {
                // Primera fila misma columna a la derecha
                neighboring_cells.push(x + 1);
            } else {
                // Primera celda
                neighboring_cells.push(0);
            }
        }

        neighboring_cells
    }

    pub fn calculate_neighbors(&self) -> Vec<HashSet<u32>> {
        let mut neighbors: Vec<HashSet<u32>> = Vec::with_capacity(self.num_particles);
        for _ in 0..self.num_particles {
            neighbors.push(HashSet::new());
        }

        // For every cell in the simulation area
        for (cell_idx, cell) in self.cells.iter().enumerate() {
            let neighboring_cells = self.get_neighboring_cells(cell_idx);
            // For every particle inside said cell
            for particle in cell {
                let particle_id = particle.get_id() as usize;
                // For neighboring cells, maximum of 5 cells
                for neighbor_idx in neighboring_cells.iter() {
                    // For particles inside the neighboring cells
                    for other_particle in self.cells[*neighbor_idx].iter() {
                        let other_id = other_particle.get_id() as usize;
                        if particle_id == other_id {
                            continue;
                        }
                        if particle.distance_to_neighbor(&other_particle) <= self.interaction_range
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
