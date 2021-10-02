use super::*;

pub struct Particle {
    pub rigid_circle: RigidCircle,
}

impl Particle {
    pub fn new(circle: Circle, mass: f32, velocity: Vec2<f32>) -> Self {
        Self {
            rigid_circle: RigidCircle {
                circle,
                velocity,
                mass,
            },
        }
    }
}

pub struct ParticleGroup {
    pub position: Vec2<f32>,
    pub amount_range: Range<usize>,
    pub radius_range: Range<f32>,
    pub initial_velocity: Vec2<f32>,
    pub velocity_offset_x_range: Range<f32>,
    pub velocity_offset_y_range: Range<f32>,
    pub color_reference: Color<f32>,
    pub color_alpha: f32,
    // color_r_range: impl SampleRange<f32> + Copy,
    // color_g_range: impl SampleRange<f32> + Copy,
    // color_b_range: impl SampleRange<f32> + Copy,
}

impl GameState {
    pub fn spawn_particle_group(&mut self, particle_group: ParticleGroup) {
        let mut rng = global_rng();
        let amount = rng.gen_range(particle_group.amount_range);

        for _ in 0..amount {
            let velocity_offset_x = rng.gen_range(particle_group.velocity_offset_x_range.clone());
            let velocity_offset_y = rng.gen_range(particle_group.velocity_offset_y_range.clone());
            let velocity =
                particle_group.initial_velocity + vec2(velocity_offset_x, velocity_offset_y);

            let radius = rng.gen_range(particle_group.radius_range.clone());

            // let color_r = rng.gen_range(color_r_range);
            // let color_g = rng.gen_range(color_g_range);
            // let color_b = rng.gen_range(color_b_range);
            // let color = Color::rgba(color_r, color_g, color_b, color_alpha);
            let mut color = particle_group.color_reference;
            color.a = particle_group.color_alpha;

            let circle = Circle::new(particle_group.position, radius, color);

            let particle = Particle::new(circle, 1.0, velocity);
            let circle = self.particles.push(particle);
        }
    }
}
