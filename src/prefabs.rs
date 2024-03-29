use crate::components::{Force, Movable, Position, RotatingObject, TrackedObject, Velocity};
use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::Entity,
    renderer::rendy::mesh::{Normal, Position as RendPosition, TexCoord},
    utils::scene::BasicScenePrefab,
    Error,
};
use serde::{Deserialize, Serialize};

pub type PlainPrefabData = BasicScenePrefab<(Vec<RendPosition>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Debug, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct PlayerPrefab {
    rotating_object: RotatingObject,
    tracked_object: TrackedObject,
    position: Position,
    velocity: Velocity,
    force: Force,
    movable: Movable,
    render: PlainPrefabData,
}
