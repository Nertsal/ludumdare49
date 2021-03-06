use super::*;

pub struct Asteroid {
    pub rigid_circle: RigidCircle,
    pub break_self: bool,
    pub destroy: bool,
    pub texture: Rc<ugli::Texture>,
}

impl Asteroid {
    pub fn new(
        circle: Circle,
        mass: f32,
        linear_velocity: Vec2<f32>,
        rotation_velocity: f32,
        texture: &Rc<ugli::Texture>,
    ) -> Self {
        Self {
            rigid_circle: RigidCircle {
                circle,
                linear_velocity,
                rotation_velocity,
                mass,
            },
            break_self: false,
            destroy: false,
            texture: texture.clone(),
        }
    }
}
