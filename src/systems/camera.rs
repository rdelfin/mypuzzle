use crate::components::TrackedObject;
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{prelude::*, Join, Read, ReadStorage, System, WriteStorage},
    renderer::{ActiveCamera, Camera},
};
use nalgebra::Vector3;

#[derive(SystemDesc)]
pub struct CameraTrackSystem;

impl<'s> System<'s> for CameraTrackSystem {
    type SystemData = (
        Read<'s, ActiveCamera>,
        WriteStorage<'s, Transform>,
        ReadStorage<'s, TrackedObject>,
    );

    fn run(&mut self, (active_camera_ent, mut transform, tracked_objects): Self::SystemData) {
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
        let camera_transform = transform
            .get_mut(active_camera_ent.entity.unwrap())
            .unwrap();

        camera_transform.face_towards(tracked_position, Vector3::new(0.0, 1.0, 0.0));
    }
}
