use crate::recs::Component;
use recs::System;
use std::any::TypeId;

mod recs;
mod rterm;

fn main() {
    let mut world = System::default();
    let mut player = world
        .create_entity()
        .set_id(30)
        .set_name("Player")
        .add_component::<Transform>(Transform { x: 1, y: 1 })
        .build();
    let mut transform = player.get_component::<Transform>();
}

#[derive(Debug)]
struct Transform {
    x: i32,
    y: i32,
}

impl Component for Transform {}
