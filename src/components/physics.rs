use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{storage::DenseVecStorage, Component, Entity, WriteStorage},
    Error,
};
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct Position {
    pub p: Vector3<f32>,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct Velocity {
    pub v: Vector3<f32>,
}

#[derive(Clone, Component, Debug, Deserialize, Serialize, PrefabData)]
#[prefab(Component)]
#[storage(DenseVecStorage)]
#[serde(deny_unknown_fields)]
pub struct Weight {
    pub g: f32,
}
