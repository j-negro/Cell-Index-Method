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
        (dx * dx + dy * dy).sqrt()
    }
}
