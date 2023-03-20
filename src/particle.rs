use neighbors::Particle;

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
