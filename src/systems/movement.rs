use crate::components::RotatingObject;
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{prelude::*, Join, ReadStorage, System, WriteStorage},
};

#[derive(SystemDesc)]
pub struct RotateSystem;

impl<'s> System<'s> for RotateSystem {
    type SystemData = (WriteStorage<'s, Transform>, ReadStorage<'s, RotatingObject>);

    fn run(&mut self, (mut transforms, rotating_objects): Self::SystemData) {
        for (transform, rotation) in (&mut transforms, &rotating_objects).join() {
            transform.append_rotation_x_axis(rotation.rate);
        }
    }
}
