use crate::prelude::*;

use self::end_turn::end_turn;

mod player_input;
mod map_renderer;
mod entity_renderer;
mod collision;
mod random_move;
mod end_turn;

pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(player_input::player_input_system())
    .flush()
    .add_system(map_renderer::map_render_system())
    .add_system(entity_renderer::entity_render_system())
    .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(collision::collision_system())
    .flush()
    .add_system(map_renderer::map_render_system())
    .add_system(entity_renderer::entity_render_system())
    .add_system(end_turn::end_turn_system())
    .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
    .add_system(random_move::random_move_system())
    .flush()
    .add_system(collision::collision_system())
    .flush()
    .add_system(map_renderer::map_render_system())
    .add_system(entity_renderer::entity_render_system())
    .add_system(end_turn::end_turn_system())
    .build()
}

