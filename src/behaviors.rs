use bevy::{math::vec2, prelude::*, window::PrimaryWindow};

use crate::{
    AlignmentGizmo, Behaviors, Boid, CohesionGizmo, SeperationGizmo, SteeringEvent, SteeringGizmo,
    Velocity,
};


pub fn seek_mouse(
    mut boids_query: Query<Entity, With<Boid>>,
    mut steering_writer: EventWriter<SteeringEvent>,
    mut gizmo: Gizmos<SteeringGizmo>,
    windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera>>,
    behaviors: Res<Behaviors>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_q.single();

    if let Some(mouse) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        gizmo.circle_2d(mouse, 10.0, Color::RED);
        for entity in &mut boids_query {
            steering_writer.send(SteeringEvent {
                entity,
                weight: behaviors.seek_mouse_strength,
                target: mouse,
            });
        }
    }
}


pub fn seperate(
    boids_query: Query<(&Transform, Entity), With<Boid>>,
    mut steering_writer: EventWriter<SteeringEvent>,
    mut gizmo: Gizmos<SeperationGizmo>,
    behaviors: Res<Behaviors>,
) {
    for (transform, entity) in &boids_query {
        let position = transform.translation.truncate();

        let mut sum = vec2(0.0, 0.0);
        let mut count = 0;
        for (other_transform, other_entity) in &boids_query {
            if entity == other_entity {
                continue;
            }
            let distance = position.distance(other_transform.translation.truncate());
            if distance < behaviors.seperation_radius {
                let difference = position - other_transform.translation.truncate();
                sum += difference / distance;
                count += 1;
            }
        }

        if count > 0 {
            sum /= count as f32;
            gizmo.arrow_2d(position, position + sum * behaviors.separation_strength, Color::PURPLE);
            gizmo.circle_2d(position, behaviors.seperation_radius, Color::PURPLE);

            steering_writer.send(SteeringEvent {
                entity,
                weight: behaviors.separation_strength,
                target: position + sum,
            });
        }
    }
}


pub fn align(
    boids_query: Query<(&Transform, &Velocity, Entity), With<Boid>>,
    mut steering_writer: EventWriter<SteeringEvent>,
    mut gizmo: Gizmos<AlignmentGizmo>,
    behaviors: Res<Behaviors>,
) {
    for (transform, _, entity) in &boids_query {
        let mut sum = vec2(0.0, 0.0);
        let mut count = 0;

        for (other_transform, other_velocity, other_entity) in &boids_query {
            if entity == other_entity {
                continue;
            }
            let difference =
                other_transform.translation.truncate() - transform.translation.truncate();
            let distance = difference.length();

            if distance < behaviors.alignment_radius {
                sum += **other_velocity;
                count += 1;
            }
        }

        if count > 0 {
            sum /= count as f32;
            gizmo.arrow_2d(
                transform.translation.truncate(),
                transform.translation.truncate() + sum * behaviors.alignment_strength,
                Color::ORANGE,
            );
            gizmo.circle_2d(
                transform.translation.truncate(),
                behaviors.alignment_radius,
                Color::ORANGE,
            );

            steering_writer.send(SteeringEvent {
                entity,
                weight: behaviors.alignment_strength,
                target: transform.translation.truncate() + sum,
            });
        }
    }
}


pub fn cohesion(
    boids_query: Query<(&Transform, Entity), With<Boid>>,
    mut steering_writer: EventWriter<SteeringEvent>,
    mut gizmo: Gizmos<CohesionGizmo>,
    behaviors: Res<Behaviors>,
) {
    for (transform, entity) in &boids_query {
        let mut sum = vec2(0.0, 0.0);
        let mut count = 0;

        for (other_transform, other_entity) in &boids_query {
            if entity == other_entity {
                continue;
            }
            let difference =
                other_transform.translation.truncate() - transform.translation.truncate();
            let distance = difference.length();
            if distance < behaviors.cohesion_radius {
                sum += other_transform.translation.truncate();
                count += 1;
            }
        }

        if count > 0 {
            sum /= count as f32;
            gizmo.arrow_2d(transform.translation.truncate(), sum, Color::OLIVE);
            gizmo.circle_2d(
                transform.translation.truncate(),
                behaviors.cohesion_radius,
                Color::OLIVE,
            );

            steering_writer.send(SteeringEvent {
                entity,
                weight: behaviors.cohesion_strength,
                target: sum,
            });
        }
    }
}

const SIZE: Vec2 = Vec2 {
    x: 1500.0,
    y: 1500.0,
};

pub fn avoid_border(
    boids_query: Query<(&Transform, Entity), With<Boid>>,
    mut steering_writer: EventWriter<SteeringEvent>,
    mut gizmo: Gizmos<SteeringGizmo>,
    behaviors: Res<Behaviors>
) {
    for (transform, entity) in &boids_query {
        let position = transform.translation.truncate();

        if position.x < -SIZE.x / 2.0 {
            gizmo.line_2d(position, vec2(-SIZE.x / 2.0, position.y), Color::RED);
            steering_writer.send(SteeringEvent {
                entity,
                weight: behaviors.border_strength,
                target: vec2(-SIZE.x / 2.0, position.y),
            });
        } else if position.x > SIZE.x / 2.0 {
            gizmo.line_2d(position, vec2(SIZE.x / 2.0, position.y), Color::RED);
            steering_writer.send(SteeringEvent {
                entity,
                weight: behaviors.border_strength,
                target: vec2(SIZE.x / 2.0, position.y),
            });
        }

        if position.y < -SIZE.y / 2.0 {
            gizmo.line_2d(position, vec2(position.x, -SIZE.y / 2.0), Color::RED);
            steering_writer.send(SteeringEvent {
                entity,
                weight: behaviors.border_strength,
                target: vec2(position.x, -SIZE.y / 2.0),
            });
        } else if position.y > SIZE.y / 2.0 {
            gizmo.line_2d(position, vec2(position.x, SIZE.y / 2.0), Color::RED);
            steering_writer.send(SteeringEvent {
                entity,
                weight: behaviors.border_strength,
                target: vec2(position.x, SIZE.y / 2.0),
            });
        }
    }
}
