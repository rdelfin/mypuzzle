use crate::components::{RotatingObject, Velocity};
use crate::input::{AxisBinding, GameBindingTypes};
use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{prelude::*, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};

#[derive(SystemDesc)]
pub struct RotateSystem;

impl<'s> System<'s> for RotateSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Velocity>,
        ReadStorage<'s, RotatingObject>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut transforms, mut velocity, rotating_objects, time): Self::SystemData) {
        for (transform, velocity, rotation) in
            (&mut transforms, &mut velocity, &rotating_objects).join()
        {
            let frame_delta_s = time.fixed_time().as_secs_f32();
            velocity.v.z = rotation.rate / (2. * std::f32::consts::PI);
            transform.append_rotation_x_axis(rotation.rate * frame_delta_s);
        }
    }
}

#[derive(SystemDesc)]
pub struct RotateInputSystem;

impl<'s> System<'s> for RotateInputSystem {
    type SystemData = (
        WriteStorage<'s, RotatingObject>,
        Read<'s, InputHandler<GameBindingTypes>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut rotating_objects, input, time): Self::SystemData) {
        let axis_rot = input.axis_value(&AxisBinding::Rotation).unwrap_or(0.0);
        let frame_delta_s = time.fixed_time().as_secs_f32();
        for rotation in (&mut rotating_objects).join() {
            let target_rot = axis_rot * rotation.max_rate;
            rotation.rate -= frame_delta_s * rotation.acceleration * (rotation.rate - target_rot);
        }
    }
}
