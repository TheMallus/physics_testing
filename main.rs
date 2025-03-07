use bevy::{prelude::*, render::mesh::shape::Cube};
use bevy_rapier3d::{prelude::*, rapier::{control::EffectiveCharacterMovement, dynamics::RigidBodyHandle}};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::time::Duration;
use blank::setup::SetupPlugin;

fn main() {
    App::new() 
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(SetupPlugin)
        .add_systems(Update, (update_system, read_result_system))
        // .add_systems(Update, cast_ray)
        .run();
    
}


fn update_system(mut controllers: Query<&mut KinematicCharacterController>, keys: Res<ButtonInput<KeyCode>>,
    mut centers: Query<(&mut Transform, &Center)>, mut commands: Commands, query: Query<Entity, With<Despawn>>,
    mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    for mut controller in controllers.iter_mut() {
            if keys.pressed(KeyCode::KeyD) {
                controller.translation = Some(Vec3::new(0.1, 0.0, -0.1));
            }
            if keys.pressed(KeyCode::KeyA) {
                controller.translation = Some(Vec3::new(-0.1, 0.0, 0.1));
            }
            if keys.pressed(KeyCode::KeyW) {
                controller.translation = Some(Vec3::new(-0.1, 0.0, -0.1));
            }
            if keys.pressed(KeyCode::KeyS) {
                controller.translation = Some(Vec3::new(0.1, 0.0, 0.1));
            }
            // if keys.pressed(KeyCode::Space) {
            //     controller.translation = Some(Vec3::new(0.0, 1.0, 0.0));
            // }
            // gravity control, WIP might be more efficient to use rust instead of bevy rapier
            if keys.pressed(KeyCode::KeyH) {
                commands.spawn((
                    RigidBody::Dynamic,
                    PbrBundle {
                        transform: Transform::from_xyz(0.0,0.5,0.0),
                        mesh: meshes.add(Mesh::from(Sphere::new(1.0))),
                        material: materials.add(Color::RED).into(),
                        ..default()
                    },
                    Collider::ball(1.0),
                    Ccd::enabled(),
                    Velocity {
                        linvel: Vec3::new(1.0,-1.0,1.0),
                        angvel: Vec3::new(0.0,0.0,0.0)
                    },
                    ColliderMassProperties::Density(2.0),
                ));
            }

            if keys.pressed(KeyCode::KeyJ) {
                for (mut transform, center) in &mut centers {
                    transform.scale = Vec3::splat(center.scale_factor);
                }
            }
            if keys.pressed(KeyCode::KeyK) {
                controller.translation = Some(Vec3::new(0.0, -0.01, 0.0));
            }
            //may or may not move block down, whether it falls below plane is subject to trial testing

    }
    
}



fn read_result_system(controllers: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in controllers.iter() {
        println!("Entity {:?} moved by {:?} and touches the ground: {:?}",
                  entity, output.effective_translation, output.grounded);
    }
}

fn cast_ray(rapier_context: Res<RapierContext>) {
    let ray_pos = Vec3::new(1.0,2.0,1.0);
    let ray_dir = Vec3::new(0.0, 0.01,0.0);
    let max_toi = 10.0;
    let solid = true; // this must remain true if laser wants to be visibly seen in debug, outside of debug
    let filter = QueryFilter::default(); // will require texture mesh and material etc.
    if let Some((entity, toi)) = rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, filter) {
        let hit_point = ray_pos + ray_dir * toi;
        println!("Entity {:?} hit at point {}", entity, hit_point);
    }
    if let Some((entity, intersection)) = rapier_context.cast_ray_and_get_normal(ray_pos, ray_dir, max_toi, solid, filter) {
        let hit_point = intersection.point;
        let hit_normal = intersection.normal;
        println!("Entity {:?} hit at point {} with normal {}", entity, hit_point, hit_normal);
    }
    rapier_context.intersections_with_ray(ray_pos, ray_dir, max_toi, solid, filter,
    |entity, intersection| {
        let hit_point = intersection.point;
        let hit_normal = intersection.normal;
        println!("Entity {:?} hit at point {} with normal {}", entity, hit_point, hit_normal);
        true
    },
    );
}

// fn update_health(mut query: Query<(&mut Health, &RigidBodyVelocity)>, quer: Query<Entity, With<Despawn>>, mut commands: Commands) {
//     for (mut health, velocity) in &mut query {
//         if velocity > 1.0 {
//             health.current -= 10.0;
//         }
//         if health.current < 0.0 {
//             for entity in &query {
//                     println!("Ded");
//                     commands.entity(entity).despawn();
//                 }
//         }
//     }
// }

// FIX/ME

#[derive(Component)]
struct Center {
    min_size: f32,
    max_size: f32,
    scale_factor: f32,
}

#[derive(Component)]
struct Despawn;

#[derive(Component)]
struct Health {
    health: f32,
}

#[derive(Component)]
struct HealthText;