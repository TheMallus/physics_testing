use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::components::{Center, Health, Despawn};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup)
        .add_systems(PostStartup, setup_graphics);
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>, mut meshes: ResMut<Assets<Mesh>>) {
    let x_size = 1.0;
    commands.spawn((
        RigidBody::KinematicPositionBased,
        PbrBundle {
            transform: Transform::from_xyz(1.1, 0.0, -0.8),
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
        Center::new(2.0, 1.0, 8.0),
        Despawn,
        Health::new(250.0),
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

    let num = 10;
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

    // making the pile of cubes (May cause lag, run in development mode)
    // to adjust amount of cubes:
    // for size, adjust variable 'num' above on line 62
    // for height, adjust variable j in for loop below

    for j in 0usize..15 {
        for i in 0..num {
            for k in 0usize..num {
                let x = i as f32 * shift - centerx + offset + -10.0;
                let y = j as f32 * shift + centery + 3.0;
                let z = k as f32 * shift - centerz + offset + 2.0;
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
                            Health::new(100.0)
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
