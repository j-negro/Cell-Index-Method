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

    pub fn distance_to_neighbor(&self, other: &Particle, offset: &(f64, f64)) -> f64 {
        let dx = self.x - other.x - offset.0;
        let dy = self.y - other.y - offset.1;
        (dx * dx + dy * dy).sqrt() - self.radius - other.radius
    }

    pub fn get_coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }

    pub fn get_radius(&self) -> f64 {
        self.radius
    }
}
