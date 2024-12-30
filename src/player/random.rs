pub mod random {

    use log::debug;
    use rand::Rng;
    use boardgame_game::game::game::Playable;

    #[derive(Clone, Default)]
    pub struct PlayerRandom {
        pub name: String,
        pub name_in_game: String,
    }

    impl crate::player::Agentish for PlayerRandom {
        fn get_name(&self) -> String {
            self.name.to_string()
        }
        /*
                fn get_name_in_game(&self) -> String {
                    match self.name_in_game.as_str() {
                        "" => panic!("No name set!"),
                        x => x.to_string(),
                    }
                }
        */
        fn get_move<T: Playable>(
            &self,
            moves: &Vec<String>,
            _active_player: &str,
            _game: &T,
        ) -> Option<String> {
            let mut rng = rand::thread_rng();
            let cmove = &moves[rng.gen_range(0..moves.len())];
            debug!("I({}) move {}", self.name, cmove);
            return Some(cmove.clone());
        }
    }
}
