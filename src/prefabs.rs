use crate::components::RotatingObject;
use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::Entity,
    renderer::rendy::mesh::{Normal, Position, TexCoord},
    utils::scene::BasicScenePrefab,
    Error,
};
use serde::{Deserialize, Serialize};

pub type PlainPrefabData = BasicScenePrefab<(Vec<Position>, Vec<Normal>, Vec<TexCoord>)>;

#[derive(Debug, Deserialize, Serialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct SpherePrefabData {
    rotating_object: RotatingObject,
    scene: PlainPrefabData,
}
