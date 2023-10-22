use crate::player_mod;
use crate::common;
use crate::obstacle_mod;

use bracket_lib::prelude::*;
pub struct State {
    player: player_mod::Player,
    frame_time: f32,
    mode: common::GameMode,
    obstacle: obstacle_mod::Obstacle,
    score: i32,
}

impl State{
    pub fn new() -> Self{
        Self{
            player: player_mod::Player::new(5, 25),
            frame_time: 0.0,
            mode: common::GameMode::Menu,
            obstacle: obstacle_mod::Obstacle::new(common::SCREEN_WIDTH, 0),
            score: 0,
        }
    }
    fn play(&mut self, ctx: &mut BTerm){
        ctx.cls_bg(NAVY);
        self.frame_time += ctx.frame_time_ms;
        if self.frame_time > common::FRAME_DURATION{
            self.frame_time = 0.0;
            self.player.gravity_and_move();
        }
        if let Some(VirtualKeyCode::Space) = ctx.key{
            self.player.flap();
        }
        self.player.render(ctx);
        ctx.print(0, 0, "Press SPACE to flap.");
        ctx.print(0, 1, &format!("Score: {}", self.score));

        self.obstacle.render(ctx, self.player.x);
        if self.player.x > self.obstacle.x{
            self.score += 1;
            self.obstacle = obstacle_mod::Obstacle::new(self.player.x + common::SCREEN_WIDTH, self.score);
        }
        if self.player.y > common::SCREEN_HEIGHT || self.obstacle.hit_obstacle(&self.player){
            self.mode = common::GameMode::End;
        }    }
    fn restart(&mut self){
        self.player = player_mod::Player::new(5, 25);
        self.frame_time = 0.0;
        self.obstacle = obstacle_mod::Obstacle::new(common::SCREEN_WIDTH, 0);
        self.mode = common::GameMode::Playing;
        self.score = 0;
    }
    fn main_menu(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "Welcome to Flappy Dragon");
        ctx.print_centered(8, "(P) Play Game");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key{
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
    fn dead(&mut self, ctx: &mut BTerm){
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(6, &format!("You earned {} points", self.score));
        ctx.print_centered(8, "(P) Play Again");
        ctx.print_centered(9, "(Q) Quit Game");
        if let Some(key) = ctx.key {
            match key{
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode{
            common::GameMode::Menu => self.main_menu(ctx),
            common::GameMode::End => self.dead(ctx),
            common::GameMode::Playing => self.play(ctx),
        }
    }
}