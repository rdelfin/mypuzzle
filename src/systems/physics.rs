use crate::components::{Position, Velocity};
use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{prelude::*, Read, ReadStorage, System, WriteStorage},
};

#[derive(SystemDesc)]
pub struct PhysicsSystem;

impl<'s> System<'s> for PhysicsSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Position>,
        ReadStorage<'s, Velocity>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut positions, velocities, time): Self::SystemData) {
        for (transform, position, velocity) in (&mut transforms, &mut positions, &velocities).join()
        {
            let frame_delta_s = time.fixed_time().as_secs_f32();
            position.p += frame_delta_s * velocity.v;
            transform.set_translation_xyz(position.p.x, position.p.y, position.p.z);
        }
    }
}
