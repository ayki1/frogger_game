use bevy::prelude::*;

#[derive(Component)]
pub struct Frog {
    pub score: u32,
    pub lives: i32,
}

#[derive(Component)]
pub struct Vehicle {
    pub speed: f32,
}

#[derive(Component)]
pub struct RiverObject {
    pub speed: f32,
}

#[derive(Component)]
pub struct Road;

#[derive(Component)]
pub struct River;

#[derive(Component)]
pub struct Log;

#[derive(Component)]
pub struct Turtle;
