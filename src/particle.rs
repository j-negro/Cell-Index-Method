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
}

pub struct Area {
    periodic: bool,
    interaction_range: f64,
    particles: Vec<Vec<Particle>>,
}

impl Area {
    pub fn new(length: usize, interaction_range: f64, periodic: bool) -> Self {
        let mut particles = Vec::with_capacity(length * length);
        for _ in 0..length * length {
            particles.push(vec![]);
        }

        Area {
            periodic,
            interaction_range,
            particles,
        }
    }

    pub fn add_particle(x: f64, y: f64) {
        todo!()
    }
}
