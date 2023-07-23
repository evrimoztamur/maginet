#[derive(Copy, Clone)]
pub enum ParticleSort {
    Missile,
    Diagonals,
    RedWin,
    BlueWin,
}

pub struct Particle {
    pub position: (f64, f64),
    velocity: (f64, f64),
    pub lifetime: u64,
    pub sort: ParticleSort,
}

impl Particle {
    pub fn new(
        position: (f64, f64),
        velocity: (f64, f64),
        lifetime: u64,
        sort: ParticleSort,
    ) -> Particle {
        Particle {
            position,
            velocity,
            lifetime,
            sort,
        }
    }

    pub fn tick(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        self.velocity.0 -= self.velocity.0 * 0.1;
        self.velocity.1 -= self.velocity.1 * 0.1;
        self.lifetime = self.lifetime.saturating_sub(1);
    }

    pub fn is_alive(&self) -> bool {
        self.lifetime > 1
    }
}
