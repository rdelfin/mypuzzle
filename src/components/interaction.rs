use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{
        storage::{DenseVecStorage, NullStorage},
        Component, Entity, WriteStorage,
    },
    Error,
};
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct RotatingObject {
    pub max_rate: f32,
    pub rate: Vector2<f32>,
    pub acceleration: f32,
}

#[derive(Clone, Component, Default, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(NullStorage)]
#[serde(deny_unknown_fields)]
pub struct TrackedObject;
