use super::*;

pub struct Asteroid {
    pub rigid_circle: RigidCircle,
}

impl Asteroid {
    pub fn new(rigid_circle: RigidCircle) -> Self {
        Self { rigid_circle }
    }
}
