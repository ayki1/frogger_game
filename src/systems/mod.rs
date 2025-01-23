use bevy::prelude::*;
use rand::Rng;
use std::collections::HashMap;

use crate::components::*;
use crate::GameState;
use crate::{WINDOW_WIDTH, WINDOW_HEIGHT};

// Hız sabitleri
const FROG_STEP: f32 = 40.0;
const VEHICLE_SPEED: f32 = 80.0;
const RIVER_OBJECT_SPEED: f32 = 60.0;

// Şerit yükseklikleri
const ROAD_LANES: [f32; 3] = [-180.0, -120.0, -60.0];
const RIVER_LANES: [f32; 3] = [60.0, 120.0, 180.0];

// Spawn oranları (ne kadar büyük o kadar az spawn olur)
const VEHICLE_SPAWN_RATE: f32 = 0.05;
const RIVER_SPAWN_RATE: f32 = 0.05;

#[derive(Resource)]
pub struct LaneObjects {
    vehicles: HashMap<i32, Vec<Entity>>,
    river_objects: HashMap<i32, Vec<Entity>>,
}

impl Default for LaneObjects {
    fn default() -> Self {
        Self {
            vehicles: HashMap::new(),
            river_objects: HashMap::new(),
        }
    }
}

pub fn movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: ParamSet<(
        Query<(&mut Transform, &mut Frog)>,
        Query<(&mut Transform, &Vehicle)>,
        Query<(&mut Transform, &RiverObject)>,
    )>,
    time: Res<Time>,
) {
    // Move frog
    if let Ok((mut transform, mut frog)) = query.p0().get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.just_pressed(KeyCode::Left) || keyboard_input.just_pressed(KeyCode::A) {
            direction.x -= 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::Right) || keyboard_input.just_pressed(KeyCode::D) {
            direction.x += 1.0;
        }
        if keyboard_input.just_pressed(KeyCode::Up) || keyboard_input.just_pressed(KeyCode::W) {
            direction.y += 1.0;
            frog.score += 10; // Her yukarı adımda 10 puan
        }
        if keyboard_input.just_pressed(KeyCode::Down) || keyboard_input.just_pressed(KeyCode::S) {
            direction.y -= 1.0;
        }

        if direction != Vec3::ZERO {
            transform.translation += direction * FROG_STEP;
            transform.translation.x = transform.translation.x.clamp(-WINDOW_WIDTH/2.0 + 15.0, WINDOW_WIDTH/2.0 - 15.0);
            transform.translation.y = transform.translation.y.clamp(-WINDOW_HEIGHT/2.0 + 15.0, WINDOW_HEIGHT/2.0 - 15.0);
        }
    }

    // Move vehicles
    for (mut transform, vehicle) in query.p1().iter_mut() {
        transform.translation.x += vehicle.speed * time.delta_seconds();
        if transform.translation.x > WINDOW_WIDTH/2.0 + 25.0 {
            transform.translation.x = -WINDOW_WIDTH/2.0 - 25.0;
        }
        if transform.translation.x < -WINDOW_WIDTH/2.0 - 25.0 {
            transform.translation.x = WINDOW_WIDTH/2.0 + 25.0;
        }
    }

    // Move river objects
    for (mut transform, river_object) in query.p2().iter_mut() {
        transform.translation.x += river_object.speed * time.delta_seconds();
        if transform.translation.x > WINDOW_WIDTH/2.0 + 50.0 {
            transform.translation.x = -WINDOW_WIDTH/2.0 - 50.0;
        }
        if transform.translation.x < -WINDOW_WIDTH/2.0 - 50.0 {
            transform.translation.x = WINDOW_WIDTH/2.0 + 50.0;
        }
    }
}

pub fn spawn_vehicles(
    mut commands: Commands,
    mut lane_objects: ResMut<LaneObjects>,
    vehicle_query: Query<(Entity, &Transform), With<Vehicle>>,
) {
    let mut rng = rand::thread_rng();
    
    if rng.gen::<f32>() < VEHICLE_SPAWN_RATE {
        // Rastgele bir şerit seç
        let lane_idx = rng.gen_range(0..ROAD_LANES.len());
        let lane = ROAD_LANES[lane_idx];
        
        // Şeritteki araçları kontrol et
        let lane_vehicles = lane_objects.vehicles.entry(lane_idx as i32).or_default();
        lane_vehicles.retain(|entity| vehicle_query.get(*entity).is_ok());
        
        // Eğer şeritte çok fazla araç varsa spawn etme
        if lane_vehicles.len() >= 3 {
            return;
        }
        
        let x = if rng.gen_bool(0.5) { -WINDOW_WIDTH/2.0 - 25.0 } else { WINDOW_WIDTH/2.0 + 25.0 };
        let vehicle_speed = if x < 0.0 { VEHICLE_SPEED } else { -VEHICLE_SPEED };
        
        // Yeni araç oluştur
        let vehicle = commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.8, 0.2, 0.2),
                    custom_size: Some(Vec2::new(50.0, 30.0)),
                    ..default()
                },
                transform: Transform::from_xyz(x, lane, 1.0),
                ..default()
            },
            Vehicle { speed: vehicle_speed },
        )).id();
        
        lane_vehicles.push(vehicle);
    }
}

pub fn spawn_river_objects(
    mut commands: Commands,
    mut lane_objects: ResMut<LaneObjects>,
    river_object_query: Query<(Entity, &Transform), With<RiverObject>>,
) {
    let mut rng = rand::thread_rng();
    
    if rng.gen::<f32>() < RIVER_SPAWN_RATE {
        let lane_idx = rng.gen_range(0..RIVER_LANES.len());
        let lane = RIVER_LANES[lane_idx];
        
        // Şeritteki nesneleri kontrol et
        let lane_objects_vec = lane_objects.river_objects.entry(lane_idx as i32).or_default();
        lane_objects_vec.retain(|entity| river_object_query.get(*entity).is_ok());
        
        // Eğer şeritte çok fazla nesne varsa spawn etme
        if lane_objects_vec.len() >= 2 {
            return;
        }
        
        let x = if rng.gen_bool(0.5) { -WINDOW_WIDTH/2.0 - 50.0 } else { WINDOW_WIDTH/2.0 + 50.0 };
        let object_speed = if x < 0.0 { RIVER_OBJECT_SPEED } else { -RIVER_OBJECT_SPEED };
        
        let river_object = if rng.gen_bool(0.5) {
            // Spawn log
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.6, 0.3, 0.1),
                        custom_size: Some(Vec2::new(100.0, 40.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, lane, 1.0),
                    ..default()
                },
                Log,
                RiverObject { speed: object_speed },
            )).id()
        } else {
            // Spawn turtle
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::rgb(0.0, 0.8, 0.0),
                        custom_size: Some(Vec2::new(40.0, 40.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x, lane, 1.0),
                    ..default()
                },
                Turtle,
                RiverObject { speed: object_speed },
            )).id()
        };
        
        lane_objects_vec.push(river_object);
    }
}

pub fn move_river_objects(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Transform, &RiverObject), With<RiverObject>>,
) {
    for (entity, mut transform, river_object) in query.iter_mut() {
        transform.translation.x += river_object.speed * time.delta_seconds();
        
        if transform.translation.x > WINDOW_WIDTH/2.0 + 50.0 || transform.translation.x < -WINDOW_WIDTH/2.0 - 50.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_victory(
    frog_query: Query<(&Transform, &Frog), With<Frog>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((transform, frog)) = frog_query.get_single() {
        if transform.translation.y > WINDOW_HEIGHT/2.0 - 30.0 {
            println!("Victory! Final Score: {}", frog.score);
            next_state.set(GameState::Victory);
        }
    }
}

fn spawn_frog(commands: &mut Commands) {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-WINDOW_WIDTH/2.0 + 15.0..WINDOW_WIDTH/2.0 - 15.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.8, 0.0),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(x, -WINDOW_HEIGHT/2.0 + 50.0, 1.0),
            ..default()
        },
        Frog {
            score: 0,
            lives: 3,
        },
    ));
}

fn check_collision(transform1: &Transform, transform2: &Transform) -> bool {
    let distance = transform1.translation.distance(transform2.translation);
    distance < 30.0
}

pub fn collision_system(
    mut commands: Commands,
    mut frog_query: Query<(Entity, &Transform, &mut Frog)>,
    vehicle_query: Query<(&Transform, &Vehicle)>,
    river_query: Query<(&Transform, &River)>,
    river_object_query: Query<(&Transform, &RiverObject)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok((frog_entity, frog_transform, mut frog)) = frog_query.get_single_mut() {
        // Check vehicle collisions
        for (vehicle_transform, _) in vehicle_query.iter() {
            if check_collision(frog_transform, vehicle_transform) {
                frog.lives -= 1;
                if frog.lives <= 0 {
                    next_state.set(GameState::GameOver);
                } else {
                    commands.entity(frog_entity).despawn();
                    spawn_frog(&mut commands);
                }
                return;
            }
        }

        // Check if frog is in river
        for (river_transform, _) in river_query.iter() {
            if check_collision(frog_transform, river_transform) {
                let mut on_safe_object = false;
                
                // Check if frog is on a river object
                for (river_object_transform, _) in river_object_query.iter() {
                    if check_collision(frog_transform, river_object_transform) {
                        on_safe_object = true;
                        break;
                    }
                }

                if !on_safe_object {
                    frog.lives -= 1;
                    if frog.lives <= 0 {
                        next_state.set(GameState::GameOver);
                    } else {
                        commands.entity(frog_entity).despawn();
                        spawn_frog(&mut commands);
                    }
                    return;
                }
            }
        }
    }
}

pub fn vehicle_river_collision_system(
    mut commands: Commands,
    vehicle_query: Query<(Entity, &Transform, &Vehicle)>,
    river_object_query: Query<(&Transform, &RiverObject)>,
) {
    for (vehicle_entity, vehicle_transform, _) in vehicle_query.iter() {
        for (river_object_transform, _) in river_object_query.iter() {
            if check_collision(vehicle_transform, river_object_transform) {
                commands.entity(vehicle_entity).despawn();
            }
        }
    }
}
