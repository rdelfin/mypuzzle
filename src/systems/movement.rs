use crate::components::{RotatingObject, Velocity};
use crate::input::{AxisBinding, GameBindingTypes};
use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{prelude::*, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};
use nalgebra::Vector2;

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

            // rate x is equivalent to sideways rotation and y is forwards and backwards
            velocity.v.x = rotation.rate.x / (2. * std::f32::consts::PI);
            velocity.v.z = rotation.rate.y / (2. * std::f32::consts::PI);
            transform.append_rotation_z_axis(rotation.rate.x * frame_delta_s);
            transform.append_rotation_x_axis(rotation.rate.y * frame_delta_s);
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
        let forwards_rot = input.axis_value(&AxisBinding::Forwards).unwrap_or(0.0);
        let sideways_rot = input.axis_value(&AxisBinding::Sideways).unwrap_or(0.0);
        let frame_delta_s = time.fixed_time().as_secs_f32();
        for rotation in (&mut rotating_objects).join() {
            rotation.rate += Vector2::new(
                self.get_rotation(
                    sideways_rot,
                    rotation.max_rate,
                    rotation.acceleration,
                    rotation.rate.x,
                    frame_delta_s,
                ),
                self.get_rotation(
                    forwards_rot,
                    rotation.max_rate,
                    rotation.acceleration,
                    rotation.rate.y,
                    frame_delta_s,
                ),
            );
        }
    }
}

impl RotateInputSystem {
    fn get_rotation(
        &self,
        axis: f32,
        max_rate: f32,
        acceleration: f32,
        current_rate: f32,
        frame_delta_s: f32,
    ) -> f32 {
        let target_rot = axis * max_rate;
        -frame_delta_s * acceleration * (current_rate - target_rot)
    }
}
