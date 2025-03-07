use bevy::prelude::*;


#[derive(Component)]
pub struct Center {
    pub min_size: f32,
    pub max_size: f32,
    pub scale_factor: f32,
}

impl Center {
    pub fn new(min_size: f32, max_size: f32, scale_factor: f32) -> Self {
        Self { min_size, max_size, scale_factor }
    }
}

#[derive(Component)]
pub struct Despawn;

/// Health points.
#[derive(Component, Debug)]
pub struct Health {
    pub health: f32,

}

impl Health {
    pub fn new(health: f32) -> Self {
        Self {health}
    }
}

#[derive(Component)]
pub struct HealthText;
