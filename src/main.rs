mod behaviors;

use std::f32::consts::PI;

use bevy::{
    input::keyboard::KeyboardInput,
    math::{vec2, vec3},
    prelude::*,
    window::WindowResolution,
};
use rand::Rng;

const MAX_SPEED: f32 = 250.0;
const MAX_FORCE: f32 = 150.0;

const VELOCITY_SCALE: f32 = 1.0;
const ACCELERATION_SCALE: f32 = 1.0;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct PhysicsGizmo;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct SteeringGizmo;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct AlignmentGizmo;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct SeperationGizmo;

#[derive(Default, Reflect, GizmoConfigGroup)]
struct CohesionGizmo;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(2000.0, 2000.0),
                ..default()
            }),
            ..default()
        }))
        .init_gizmo_group::<PhysicsGizmo>()
        .init_gizmo_group::<SteeringGizmo>()
        .init_gizmo_group::<AlignmentGizmo>()
        .init_gizmo_group::<SeperationGizmo>()
        .init_gizmo_group::<CohesionGizmo>()
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_systems(Startup, setup)
        .add_systems(Update, update_boids_rotation)
        .add_systems(FixedUpdate, update_boids_physics)
        // .add_systems(FixedUpdate, gravity_system)
        .add_systems(FixedUpdate, force_event_system)
        .add_systems(FixedUpdate, steering_event_system)
        // .add_systems(FixedUpdate, behaviors::seek_mouse)
        .add_systems(FixedUpdate, behaviors::seperate)
        .add_systems(FixedUpdate, behaviors::align)
        .add_systems(FixedUpdate, behaviors::cohesion)
        .add_systems(FixedUpdate, behaviors::avoid_border)
        .add_systems(Update, update_gizmo)
        .add_event::<ForceEvent>()
        .add_event::<SteeringEvent>()
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let mut rng = rand::thread_rng();

    let texture = asset_server.load("textures/boid.png");
    for _ in 0..200 {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: vec3(
                        (rng.gen::<f32>() - 0.5) * 1000.0,
                        (rng.gen::<f32>() - 0.5) * 1000.0,
                        0.0,
                    ),
                    scale: vec3(0.05, 0.05, 0.05),
                    ..default()
                },
                texture: texture.clone(),
                ..Default::default()
            },
            Velocity(vec2(0.0, 0.0)),
            Acceleration(vec2(
                (rng.gen::<f32>() - 0.5) * 10.0,
                (rng.gen::<f32>() - 0.5) * 10.0,
            )),
            Boid,
        ));
    }
}

fn update_gizmo(mut config_store: ResMut<GizmoConfigStore>, keyboard: Res<ButtonInput<KeyCode>>) {
    let (physics, _) = config_store.config_mut::<PhysicsGizmo>();
    if keyboard.pressed(KeyCode::Digit1) {
        physics.enabled = true;
    } else {
        physics.enabled = false;
    }
    println!("Physics: {}", physics.enabled);

    let (steering, _) = config_store.config_mut::<SteeringGizmo>();
    if keyboard.pressed(KeyCode::Digit2) {
        steering.enabled = true;
    } else {
        steering.enabled = false;
    }
    println!("Steering: {}", steering.enabled);

    let (seperation, _) = config_store.config_mut::<SeperationGizmo>();
    if keyboard.pressed(KeyCode::Digit3) {
        seperation.enabled = true;
    } else {
        seperation.enabled = false;
    }
    println!("Seperation: {}", seperation.enabled);

    let (cohesion, _) = config_store.config_mut::<CohesionGizmo>();
    if keyboard.pressed(KeyCode::Digit4) {
        cohesion.enabled = true;
    } else {
        cohesion.enabled = false;
    }
    println!("Cohesion: {}", cohesion.enabled);

    let (alignment, _) = config_store.config_mut::<AlignmentGizmo>();
    if keyboard.pressed(KeyCode::Digit5) {
        alignment.enabled = true;
    } else {
        alignment.enabled = false;
    }
    println!("Alignment: {}", alignment.enabled);
}

fn update_boids_rotation(mut boids_query: Query<(&mut Transform, &Velocity), With<Boid>>) {
    for (mut transform, velocity) in &mut boids_query {
        transform.rotation = Quat::from_rotation_z(velocity.to_angle() - PI * 0.5);
    }
}

fn update_boids_physics(
    mut boids_query: Query<(&mut Velocity, &mut Acceleration, &mut Transform), With<Boid>>,
    mut gizmo: Gizmos<PhysicsGizmo>,
    time: Res<Time>,
) {
    for (mut velocity, mut acceleration, mut transform) in &mut boids_query {
        gizmo.arrow_2d(
            transform.translation.truncate(),
            transform.translation.truncate() + **velocity * VELOCITY_SCALE,
            Color::GREEN,
        );
        gizmo.arrow_2d(
            transform.translation.truncate(),
            transform.translation.truncate() + **acceleration * ACCELERATION_SCALE,
            Color::BLUE,
        );
        **velocity += **acceleration * time.delta_seconds();
        **acceleration = vec2(0.0, 0.0);
        transform.translation += velocity.extend(0.0) * time.delta_seconds();
        transform.rotation = Quat::from_rotation_z(velocity.to_angle() - PI * 0.5);
    }
}

fn gravity_system(
    mut boids_query: Query<Entity, With<Boid>>,
    mut force_writer: EventWriter<ForceEvent>,
) {
    for entity in &mut boids_query {
        force_writer.send(ForceEvent {
            entity,
            force: vec2(0.0, -0.01),
        });
    }
}

fn steering_event_system(
    mut steering_reader: EventReader<SteeringEvent>,
    mut boids_query: Query<(&Transform, &Velocity, Entity), With<Boid>>,
    mut force_writer: EventWriter<ForceEvent>,
    mut gizmo: Gizmos<SteeringGizmo>,
) {
    for event in steering_reader.read() {
        if let Ok((transform, velocity, entity)) = boids_query.get_mut(event.entity) {
            gizmo.arrow_2d(transform.translation.truncate(), event.target, Color::RED);
            let desired = (event.target - transform.translation.truncate()).normalize() * MAX_SPEED;
            let steering = desired - **velocity;
            let weighted = steering * event.weight;
            force_writer.send(ForceEvent {
                entity,
                force: weighted,
            });
        }
    }
}

fn force_event_system(
    mut _commands: Commands,
    mut force_reader: EventReader<ForceEvent>,
    mut boids_query: Query<&mut Acceleration, With<Boid>>,
) {
    for event in force_reader.read() {
        if let Ok(mut acceleration) = boids_query.get_mut(event.entity) {
            **acceleration += event.force.clamp_length_max(MAX_FORCE);
        }
    }
}

#[derive(Event)]
struct ForceEvent {
    entity: Entity,
    force: Vec2,
}

#[derive(Event)]
struct SteeringEvent {
    entity: Entity,
    target: Vec2,
    weight: f32,
}

#[derive(Component)]
struct Boid;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Component, Deref, DerefMut)]
struct Acceleration(Vec2);
