extern crate rltk;

use ad_foramen_cuniculi::entity::Entity;
use ad_foramen_cuniculi::system::System;
use rltk::{GameState, Rltk, RltkBuilder, VirtualKeyCode, RGB};

#[derive(Default)]
struct State {
    ecs: System,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();
        self.ecs.entities.iter().for_each(|x| {
            let position: &Position = x.get_component().unwrap();
            let renderer: &Renderer = x.get_component().unwrap();
            ctx.set(
                position.x,
                position.y,
                renderer.fg,
                renderer.bg,
                renderer.glyph,
            );
        });
        player_input(self, ctx);
    }
}

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50()
        .with_title("Ad Foramen Cuniculi")
        .with_tile_dimensions(15, 15)
        .build()?;
    let mut gs = State::default();

    let mut player = Entity::default();
    player.set_name("criosage");
    player.add_component::<Position>();
    player.add_component_dec(Box::new(Renderer {
        fg: RGB::named(rltk::RED),
        bg: RGB::named(rltk::BLACK),
        glyph: rltk::to_cp437('@'),
    }));
    gs.ecs.entities.push(player);

    rltk::main_loop(context, gs)
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

fn try_move_player(delta_x: i32, delta_y: i32, world: &mut System) {
    let position: &mut Position = world.get_entity_mut(0).get_component_mut().unwrap();

    position.x += delta_x;
    position.y += delta_y;
}
// ----- Components
#[derive(Default)]
struct Position {
    x: i32,
    y: i32,
}

struct Renderer {
    fg: RGB,
    bg: RGB,
    glyph: rltk::FontCharType,
}
