/*
 * LOGIC GATE NEURAL NETWORK!
 */
pub mod plgnn_diamond {
    use crate::player::*;
    //use get_terminal_state_from_bit_state;
    use crate::player::brain::brain::*;
    use crate::player::brain::lg_diamond::lg_diamond::*;
    use Agentish;
    use boardgame_game::game::game::*;
    use log::{debug,warn};
    use std::collections::HashMap;

    #[derive(Clone)]
    pub struct PlayerNNDiamond {
        pub name: String,
        pub is_loaded: bool,
        pub brain: BrainDiamond,
        me_color: String,
        opponent_color: String,
    }

    impl Default for PlayerNNDiamond {
        /// remember to set game name after this is used.
        fn default() -> PlayerNNDiamond {
            //            let mut brain = BrainDiamond::default();
            //            brain.game_name = game_static.name.clone();
            PlayerNNDiamond {
                name: "LG Diamond".to_string(),
                is_loaded: false,
                brain: BrainDiamond::default(),
                me_color: "".to_string(),
                opponent_color: "".to_string(),
            }
        }
    }

    impl Agentish for PlayerNNDiamond {
        fn get_name(&self) -> String {
            self.name.to_string()
        }

        fn get_move<T: Playable>(
            &self,
            moves: &Vec<String>,
            player: &str,
            game: &T,
        ) -> Option<String> {
            // temporary until ai is working
            let bit_state = game.get_bit_state(player);
            let current_grade = self.brain.evaluate_bit_state(&bit_state);
            let mut move_alternatives: HashMap<String, usize> = HashMap::new();
            debug!("current state: {:?}", bit_state);
            for mov in moves.iter() {
                // let bit_state_clone = bit_state.clone();
                let tmp_state =
                    game.get_bit_state_from_bit_state_and_move(&player, &bit_state, &mov);
                let mut tmp_grade = self.brain.evaluate_bit_state(&tmp_state);
                //debug!("state: {:?}", tmp_state);
                // if temp game is terminal set hardcoded values
                match get_terminal_state_from_bit_state(&tmp_state) {
                    Some(x) => {
                        debug!("GAME IS Termialstate: {:?} \n{:?}", x, tmp_state);

                        tmp_grade = match x {
                            TerminalState::Me => CELL_SIZE,
                            TerminalState::Opponent => 0,
                            TerminalState::Draw => CELL_SIZE / 2,
                        }
                    }
                    None => {}
                }
                move_alternatives
                    .entry(mov.to_string())
                    .or_insert(tmp_grade);
            }
            let cmove = move_alternatives
                .iter()
                .max_by(|a, b| a.1.cmp(&b.1))
                .map_or("MISSING MOVE ALTERNATIVES", |(k, _v)| k);

            debug!("state:{}.  ", current_grade);
            debug!("alternatives: {:?}", move_alternatives);
            debug!("I({}) move {} ", self.name, cmove);
            return Some(cmove.to_string());
        }

        fn get_ready(&mut self, game_static: &GameStatic,_me_color: &str) -> Result<(), LogicGatesError> {
            // _me_color is ignored. Expect always to only think one move a head.


            if !self.is_loaded {
                let mut brain = BrainDiamond::default();
                brain.game_name = game_static.name.clone();
                //todo!("Look for file or generate a random brain");
                match brain.from_file() {
                    Ok(_value) => (),
                    Err(e) => match e {
                        LogicGatesError::Io { source: x } => {
                            if x.kind() == std::io::ErrorKind::NotFound {
                                warn!("File not found loading brain. Generate a new one");
                                brain = generate_random_brain(&game_static);
                                brain.save_to_file()?;
                                ()
                            };
                        }
                        other_error => panic!("Problem creating the file: {other_error:?}"),
                    },
                }
                // warn!("BrainDiamond is {:?}", brain);
                self.brain = brain;
            }
             Ok(())
        }
    }
}
