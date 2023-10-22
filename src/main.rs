#![warn(clippy::all, clippy::pedantic)]
use bracket_lib::prelude::*;
mod player_mod;
mod state_mod;
mod common;
mod obstacle_mod;

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;

    main_loop(context, state_mod::State::new())
}
