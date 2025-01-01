/*
 * LOGIC GATE NEURAL NETWORK!
 */
//use crate::Playable;
pub mod plgnn_trunc {
    #![allow(dead_code)]
    use crate::player::brain::brain::*;
    //use crate::player::brain::lg_trunc::*;
    //use crate::player::plgnn_trunc::plgnn_trunc::lg_trunc::Brain;
    //use crate::{GameStatic, Playable};
    use boardgame_game::game::game::GameStatic;
    use boardgame_game::game::game::Playable;
    use boardgame_game::game::game::get_terminal_state_from_bit_state;
    use crate::player::brain::lg_trunc::lg_trunc::*;
    use crate::player::Agentish;
    use boardgame_game::game::game::TerminalState;
    use log::{debug, warn};
    use std::collections::HashMap;

    #[derive(Clone)]
    pub struct PlayerNeuralNetwork {
        pub name: String,
        pub is_loaded: bool,
        pub brain: Brain,
    }

    impl Default for PlayerNeuralNetwork {
        fn default() -> PlayerNeuralNetwork {
            PlayerNeuralNetwork {
                name: "Logic gates".to_string(),
                is_loaded: false,
                brain: Brain {
                    game_name: "Not set".to_string(),
                    filepath: "".to_string(),
                    layers: [get_default_cell(); NO_LAYERS],
                },
            }
        }
    }

    impl Agentish for PlayerNeuralNetwork {
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
            let current_grade = self.brain.evaluate_bit_state(&player, &bit_state);
            let mut move_alternatives: HashMap<String, usize> = HashMap::new();
            debug!("current state: {:?}", bit_state);
            for mov in moves.iter() {
                // let bit_state_clone = bit_state.clone();
                let tmp_state =
                    game.get_bit_state_from_bit_state_and_move(&player, &bit_state, &mov);
                let mut tmp_grade = self.brain.evaluate_bit_state(&player, &tmp_state);
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

        fn get_ready(&mut self, game_static: &GameStatic, my_color: &str) -> Result<(), LogicGatesError> {

            // TODO: Implementer bruk av my_color.
            if !self.is_loaded {
                let layer = get_default_cell();
                let mut brain = Brain {
                    game_name: game_static.name.clone(),
                    filepath: "".to_string(),
                    layers: [layer; NO_LAYERS],
                };
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
                warn!("Brain is {:?}", brain);
                self.brain = brain;
            }
            Ok(())
        }
    }

}
