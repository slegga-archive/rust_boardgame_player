use player::Agentish;

pub mod brain;
pub mod human;
//pub mod monte_carlo;
pub mod plgnn_diamond;
pub mod plgnn_diamond_tree_search;
pub mod random;


pub mod player {
    //use crate::{GameStatic, Playable};
    use std::fmt::Display;
    //use boardgame_game::game::GameStatic;
    use boardgame_game::game::game::GameStatic;
    use crate::player::brain::brain::LogicGatesError;

    pub trait Agentish {
        fn get_move<T: boardgame_game::game::game::Playable + Display>(
            &self,
            moves: &Vec<String>,
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
}
