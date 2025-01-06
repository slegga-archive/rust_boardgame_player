pub mod brain;
pub mod human;
//pub mod monte_carlo;
pub mod plgnn_diamond;
pub mod plgnn_diamond_tree_search;
pub mod random;

use crate::player::brain::LogicGatesError;
use boardgame_game::game::GameStatic;
use boardgame_game::game::Playable;
use std::fmt::Display;

pub trait Agentish {
    fn get_move<T: Playable + Display>(
        &self,
        moves: &[String],
        game_player: &str,
        game: &T,
    ) -> Option<String>;
    fn get_name(&self) -> String;

    fn get_ready(
        &mut self,
        game_static: &GameStatic,
        me_color: &str,
    ) -> Result<(), LogicGatesError>;
}
