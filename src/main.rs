use::bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use rand::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_plugins(EguiPlugin)
    .add_systems(Startup, setscene)
    .add_systems(Update, ui)
    .run();
}

fn setscene(mut commands: Commands) { // ADD TO parameter if uncommenting / ( / (mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>> / ) /
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(Circle::new(15.0)),
    //     material: materials.add(Color::WHITE),
    //     transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    //     ..default()
    // });
    commands.spawn(Collider::cuboid(200.0, 0.1, 200.0))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    

    commands.spawn(RigidBody::Dynamic)
            .insert(Collider::ball(0.5))
            .insert(Restitution::coefficient(0.7))
            .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
            .insert(Ccd::enabled());
    
    commands.spawn(RigidBody::KinematicPositionBased)
            .insert(TransformBundle::default())
            .insert(Collider::cuboid(5.0, 10.0, 5.0))
            .insert(KinematicCharacterController::default())
            .insert(GravityScale(0.5))
            .insert(ColliderMassProperties::Density(2.0));


    commands.spawn(Camera3dBundle {
                transform: Transform::from_xyz(-10.0, 25.0, 10.0).looking_at(Vec3::new(0.0, 10.0, 0.0),Vec3::Y),
                ..Default::default()
    });

    bed(&mut commands, Vec3::new(20.0, 0.0, 0.0));

    let block = commands.spawn((
        TransformBundle::from(Transform::from_xyz(-5.0, 5.0, -5.0)),
        RigidBody::Fixed,
        Collider::cuboid(1.0, 1.0, 1.0),
    )).id();
    let x = Vec3::X;
    let joint = RevoluteJointBuilder::new(x)
    .local_anchor1(Vec3::new(0.0, 0.0,  1.0))
    .local_anchor2(Vec3::new(0.0, 0.0, -3.0));

    commands.entity(block).insert(ImpulseJoint::new(block, joint));

}
fn ui(mut contexts: EguiContexts) {
    egui::Window::new("Stats").show(contexts.ctx_mut(), |ui| {
        ui.label("Stat 1");
    });
} 


fn bed(commands: &mut Commands, origin: Vec3) {
    let rad = 0.4;
    let shift = 2.0;
    let mut curr_parent = commands.spawn((
        TransformBundle::from(Transform::from_xyz(origin.x, origin.y, 0.0)),
        RigidBody::Fixed,
        Collider::cuboid(rad * 4.0, rad, rad),
    ))
    .id();
    for i in 0..1 {
        let z = origin.z + i as f32 * shift * 2.0 + shift;
        let positions = [Vec3::new(origin.x, origin.y, z),
                                    Vec3::new(origin.x + shift, origin.y, z),
                                    Vec3::new(origin.x + shift, origin.y, z + shift),
                                    Vec3::new(origin.x, origin.y, z + shift),];
        
        let mut handles = [curr_parent; 4];
        for k in 0..4 {
            handles[k] = commands.spawn((
                TransformBundle::from(Transform::from_translation(positions[k])),
                RigidBody::Dynamic,
                Collider::cylinder(rad, rad),
            )).id();
        }

        let x = Vec3::X;
        let z = Vec3::Z;
        let revs = [
            RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -shift)),
            RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(-shift, 0.0, 0.0)),
            RevoluteJointBuilder::new(z).local_anchor2(Vec3::new(0.0, 0.0, -shift)),
            RevoluteJointBuilder::new(x).local_anchor2(Vec3::new(shift, 0.0, 0.0)),
        ];

        commands.entity(handles[0]).insert(ImpulseJoint::new(curr_parent, revs[0]));
        commands.entity(handles[1]).insert(ImpulseJoint::new(curr_parent, revs[1]));
        commands.entity(handles[2]).insert(ImpulseJoint::new(curr_parent, revs[2]));
        commands.entity(handles[3]).insert(ImpulseJoint::new(curr_parent, revs[3]));
        curr_parent = handles[3];
    }
}


// fn bullet_fire(mut query: Query<(&mut Transform, &mut Laser, Entity)>, time: Res<Time>, mut commands: Commands) {
//     for (mut transform, mut laser, entity) in query.iter_mut() {
//         laser.velocity *= 1.0 + time.delta_seconds() * 6.0;
//         transform.translation += laser.velocity * time.delta_seconds();
//         if transform.translation.distance(Vec3::ZERO) > 50.0 {
//             commands.entity(entity).despawn_recursive();
//         }
//     }
// }
// fn spawn_bullet(query: Query<(Entity, &AwaitingSpawnGun)>,
//     children: Query<&Children>, names: Query<&Name>, mut commands: Commands) {
//         for (entity, spawn_gun) in query.iter() {
//             let mut spawned_guns = false;
//             for child in Children.iter_descendants(entity) {
//                 let Ok(name) = names.get(child) else {
//                     continue;
//                 };
//                 if name.contains("laser") {
//                     spawned_guns = true;
//                     commands.entity(child).insert(Gun {
//                         color: spawn_gun.color,
//                         timer: Timer::from_seconds(
//                             rand::thread_rng().gen_range(0.6..1.3),
//                             TimerMode::Repeating,
//                         ),
//                     })
//                 }
//             }
//         }
//     }

#[derive(Component)]
pub struct Laser {
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct Gun {
    pub color: Color,
}

#[derive(Component)]
pub struct AwaitingSpawnGun {
    pub color: Color,
}


pub fn cast_ray(mut commands: Commands, windows: Query<&Window, With<PrimaryWindow>>,
        rapier_context: Res<RapierContext>, cameras: Query<(&Camera, &GlobalTransform)>,) {
            let window = windows.single();
            let Some(cursor_position) = window.cursor_position() else {
                return;
            };
            for (camera, camera_transform) in &cameras {
                let Some(ray) = camera.viewport_to_world(camera_transform, cursor_position) else {
                    return;
                };
                let hit = rapier_context.cast_ray(ray.origin, ray.direction.into(),
                 f32::MAX, true, QueryFilter::only_dynamic(),
                );
                if let Some((entity, _toi)) = hit {
                    let color = Color::BLUE;
                    commands.entity(entity).insert(ColliderDebugColor(color));
                }
            }
        }

#[derive(Component, Reflect)]
pub struct PrimaryWindow;
