#[derive(Debug)]
pub struct Particle {
    id: u32,
    x: f64,
    y: f64,
    radius: f64,
}

impl Particle {
    pub fn new(id: u32, x: f64, y: f64, radius: f64) -> Particle {
        Particle { id, x, y, radius }
    }

    pub fn distance_to_neighbor(&self, other: &Particle) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt() - self.radius - other.radius
    }

    pub fn get_coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}

#[derive(Debug)]
pub struct CellIndexMethod<'a> {
    length: f64,
    periodic: bool,
    m: usize,
    interaction_range: f64,
    cells: Vec<Vec<&'a Particle>>,
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
        }
    }
}
