use bevy::{prelude::*, render::mesh::shape::Cube};
use bevy_rapier3d::{prelude::*, rapier::dynamics::RigidBodyHandle};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use std::time::Duration;

fn main() {
    App::new() 
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_graphics)
        .add_systems(Update, (update_system, read_result_system))
        .add_systems(Update, cast_ray)
        .run();
    
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    let x_size = 1.0;
    commands.spawn((
        RigidBody::KinematicPositionBased,
        PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Cube::new(2.0))),
            material: materials.add(Color::NAVY).into(),
            ..default()
        },
        Collider::cuboid(x_size, x_size, x_size),
        ColliderMassProperties::Density(2.0),
        KinematicCharacterController {
            up: Vec3::Y,
            ..default()
        },
        GravityScale(0.5),
        Ccd::enabled(),
        Sleeping::disabled(),
        Center {
            max_size: 2.0,
            min_size: 1.0,
            scale_factor: 8.0,
        },
        Despawn,
    ));
    commands.spawn((
        RigidBody::Fixed,
        PbrBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            mesh: meshes.add(Mesh::from(shape::Cube::new(100.0))),
            material: materials.add(Color::WHITE).into(),
            ..default()
        },
        Collider::cuboid(100.0, 0.1, 100.0),
        ColliderMassProperties::Density(10.0),
        Ccd::disabled(),

    ));

    let num = 6;
    let rad = 1.0;

    let shift = rad * 2.0 + rad;
    let centerx = shift + 2.0 * (num / 2) as f32;
    let centery = shift / 2.0;
    let centerz = shift + 2.0 * (num / 2) as f32;

    let mut offset = -(num as f32) * (rad * 2.0 + rad) * 0.5;
    let mut color = 0;
    let colors = [
        Color::hsl(220.0, 1.0, 0.3),
        Color::hsl(180.0, 1.0, 0.3),
        Color::hsl(260.0, 1.0, 0.7),
    ];

    for j in 0usize..10 {
        for i in 0..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx + offset;
                let y = j as f32 * shift + centery + 3.0;
                let z = k as f32 * shift - centerz + offset;
                color += 1;

                commands
                    .spawn(TransformBundle::from(Transform::from_rotation(
                        Quat::from_rotation_x(0.2),
                    )))
                    .with_children(|child| {
                        child.spawn((
                            PbrBundle {
                                transform: Transform::from_xyz(x,y,z),
                                mesh: meshes.add(Mesh::from(shape::Cube::new(2.0))),
                                material: materials.add(Color::WHITE).into(),
                                ..default()
                            },
                            RigidBody::Dynamic,
                            Collider::cuboid(rad, rad, rad),
                            ColliderDebugColor(colors[color % 3]),
                            Ccd::enabled(),
                        ));
                    });
            }
        }

    //     offset -= 0.05 * rad * (num as f32 - 1.0);
     }
}

fn setup_graphics(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(44.0, 30.0, 40.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
fn update_system(mut controllers: Query<&mut KinematicCharacterController>, keys: Res<ButtonInput<KeyCode>>,
    mut centers: Query<(&mut Transform, &Center)>, mut commands: Commands, query: Query<Entity, With<Despawn>>,
) {
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
            if keys.pressed(KeyCode::KeyH) {
                for (mut transform, center) in &mut centers {
                    transform.scale = Vec3::splat(center.scale_factor);
                }
            }
            if keys.pressed(KeyCode::KeyJ) {
                for entity in &query {
                    println!("Despawning ground entity");
                    commands.entity(entity).despawn();
                }
            }

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
    let solid = true;
    let filter = QueryFilter::default();
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

#[derive(Component)]
struct Center {
    min_size: f32,
    max_size: f32,
    scale_factor: f32,
}

#[derive(Component)]
struct Despawn;