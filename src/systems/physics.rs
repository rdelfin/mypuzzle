use crate::components::{Movable, Position, Velocity};
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
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, Movable>,
        Read<'s, Time>,
    );

    fn run(
        &mut self,
        (mut transforms, mut positions, mut velocities, movables, time): Self::SystemData,
    ) {
        for (transform, position, velocity, movable) in (
            &mut transforms,
            &mut positions,
            (&mut velocities).maybe(),
            (&movables).maybe(),
        )
            .join()
        {
            let frame_delta_s = time.fixed_time().as_secs_f32();
            if let Some(velocity) = velocity {
                position.p += frame_delta_s * velocity.v;

                if let Some(movable) = movable {
                    if position.p.y > 1.0 {
                        velocity.v.y -= movable.g * frame_delta_s;
                    } else {
                        velocity.v.y = 0.0;
                        position.p.y = 1.0;
                    }
                }
            }
            transform.set_translation_xyz(position.p.x, position.p.y, position.p.z);
        }
    }
}
