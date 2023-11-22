extern crate rltk;
use rltk::{Rltk, GameState, Console, RltkBuilder};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello RLTK World");
    }
}

fn main() -> rltk::BError {
    let context = RltkBuilder::simple80x50()
        .with_title("Ad Foramen Cuniculi")
        .build()?;
    let gs = State{ };
    rltk::main_loop(context, gs)
}