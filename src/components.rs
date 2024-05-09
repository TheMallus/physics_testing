use bevy::ecs::component::Component;


#[derive(Component)]
pub struct Center {
    pub min_size: f32,
    pub max_size: f32,
    pub scale_factor: f32,
}

#[derive(Component)]
pub struct Despawn;

/// Health points.
#[derive(Component)]
pub struct Health {
    pub hp: i32,
    pub max_hp: i32,

}

impl Default for Health {
    fn default() -> Self {
        Self {
            hp: 100,
            max_hp: 100,
        }
    }
}
