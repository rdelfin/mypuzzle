use amethyst::{
    assets::{PrefabData, ProgressCounter},
    derive::PrefabData,
    ecs::{Component, Entity, NullStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Component, PrefabData, Serialize, Deserialize)]
#[storage(NullStorage)]
pub struct RotatingObject;
