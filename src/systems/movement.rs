use crate::components::{RotatingObject, Velocity};
use crate::input::{ActionBinding, AxisBinding, GameBindingTypes};
use amethyst::{
    core::{Time, Transform},
    derive::SystemDesc,
    ecs::{prelude::*, Join, Read, ReadStorage, System, WriteStorage},
    input::InputHandler,
};
use nalgebra::{Unit, UnitQuaternion, Vector2, Vector3};

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

            // Append axis angle to rotation based on direction of movement
            if rotation.rate.norm() != 0. {
                let axis = Unit::new_normalize(Vector3::new(rotation.rate.y, 0., -rotation.rate.x));
                let angle = rotation.rate.norm() * frame_delta_s;
                transform.set_rotation(
                    UnitQuaternion::from_axis_angle(&axis, angle) * transform.rotation(),
                );
            }
        }
    }
}

#[derive(SystemDesc, Default)]
pub struct RotateInputSystem {
    jump_was_pressed: bool,
}

impl<'s> System<'s> for RotateInputSystem {
    type SystemData = (
        WriteStorage<'s, RotatingObject>,
        WriteStorage<'s, Velocity>,
        Read<'s, InputHandler<GameBindingTypes>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut rotating_objects, mut velocities, input, time): Self::SystemData) {
        let forwards_rot = input.axis_value(&AxisBinding::Forwards).unwrap_or(0.0);
        let sideways_rot = input.axis_value(&AxisBinding::Sideways).unwrap_or(0.0);
        let jump_pressed = input.action_is_down(&ActionBinding::Jump).unwrap_or(false);
        let frame_delta_s = time.fixed_time().as_secs_f32();

        for (rotation, velocity) in (&mut rotating_objects, &mut velocities).join() {
            if self.jump_was_pressed && !jump_pressed {
                velocity.v.y += 10.;
            }
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

        self.jump_was_pressed = jump_pressed;
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
