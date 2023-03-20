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
pub struct CellIndexParticle {
    id: u32,
    x: f64,
    y: f64,
    radius: f64,
}

impl CellIndexParticle {
    pub fn new(id: u32, x: f64, y: f64, radius: f64) -> CellIndexParticle {
        CellIndexParticle { id, x, y, radius }
    }
}

impl Particle for CellIndexParticle {
    fn get_coordinates(&self) -> (f64, f64) {
        (self.x, self.y)
    }

    fn get_id(&self) -> u32 {
        self.id
    }

    fn get_radius(&self) -> f64 {
        self.radius
    }
}
