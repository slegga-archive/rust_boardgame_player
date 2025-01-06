use crate::player::LogicGatesError;
use boardgame_game::game::Playable;
use log::debug;
use rand::Rng;

#[derive(Clone, Default)]
pub struct PlayerRandom {
    pub name: String,
    pub name_in_game: String,
}

impl crate::player::Agentish for PlayerRandom {
    fn get_name(&self) -> String {
        self.name.to_string()
    }

    fn get_move<T: Playable>(
        &self,
        moves: &Vec<String>,
        _active_player: &str,
        _game: &T,
    ) -> Option<String> {
        let mut rng = rand::thread_rng();
        let cmove = &moves[rng.gen_range(0..moves.len())];
        debug!("I({}) move {}", self.name, cmove);

        // return value
        Some(cmove.clone())
    }

    fn get_ready(
        &mut self,
        _game_static: &boardgame_game::game::GameStatic,
        _my_color: &str,
    ) -> Result<(), LogicGatesError> {
        // Trender ikke å gjøre mer
        Ok(())
    }
}
