use crate::components::TrackedObject;
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{prelude::*, Join, ReadStorage, System, WriteStorage},
    renderer::Camera,
};
use nalgebra::Vector3;

#[derive(SystemDesc)]
pub struct CameraTrackSystem;

impl<'s> System<'s> for CameraTrackSystem {
    type SystemData = (
        ReadStorage<'s, Camera>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, TrackedObject>,
    );

    fn run(&mut self, (cameras, mut transform, tracked_objects): Self::SystemData) {
        let tracked_position = {
            let tracked_transform =
                match (&transform, &tracked_objects).join().map(|(t, _)| t).nth(0) {
                    Some(t) => t,
                    None => {
                        return;
                    }
                };
            tracked_transform.translation().clone()
        };
        let camera_transform = match (&mut transform, &cameras).join().map(|(t, _)| t).nth(0) {
            Some(t) => t,
            None => {
                return;
            }
        };

        camera_transform.set_translation_x(tracked_position.x);
        camera_transform.set_translation_z(tracked_position.z - 10.);
        camera_transform.face_towards(tracked_position, Vector3::new(0.0, 1.0, 0.0));
    }
}
