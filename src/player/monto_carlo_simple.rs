pub mod monto_carlo_simple {
    use rand;
    use time;

    use crate::{Agentish, GameStatic};
    use log::{debug, error, info, warn};
    use std::cmp::{max, min};

    /// A Monte Carlo search player. This player should only be used for 2 player, constant sum,
    /// turn based games.
    pub struct PlayerMCS {
        name: String,
        depth_limit: u32,
        best_move: Option<String>,
        charge_count: u32,
    }

    impl PlayerMCS {
        /// Returns an PlayerMCS that begins the random terminal state searches at depth `depth`
        pub fn new(game_static: &GameStatic) -> PlayerMCS {
            PlayerMCS {
                name: "MonteCarlo Simple".to_string(),
                depth_limit: 80,
                best_move: None,
                charge_count: 0,
            }
        }

        fn best_move<T: crate::Playable>(&mut self, game: &T) -> Option<String> {
            let player = game.get_active_player();
            let mut moves = game.get_valid_moves();
            assert!(!moves.is_empty(), "No legal moves");

            /*if moves.len() == 1 {
                return Ok(moves.swap_remove(0));
            }*/

            let mut res = moves[0].clone();
            self.best_move = Some(res.clone());

            let mut max = 0;
            self.best_move = Some(res.clone());
            let opponent = opponent(game, player);
            for m in moves {
                let score = match self.min_score(game, opponent, m.clone(), 0, 100, 0) {
                    Ok(score) => score,
                    Err(m) => return Err(m),
                };
                if score == 100 {
                    return Ok(m);
                } else if score > max {
                    max = score;
                    self.best_move = Some(m.clone());
                    res = m
                }
                check_time_result!(self, game);
            }
            Ok(res)
        }

        fn max_score<T: crate::Playable>(
            &mut self,
            game: &T,
            player: &str,
            alpha: u8,
            beta: u8,
            depth: u32,
        ) -> Option<String> {
            if depth >= self.depth_limit {
                return self.monte_carlo(player, game);
            }

            if game.is_terminal() {
                return Ok(game.goal(state, game.player()));
            }

            let moves = game.get_valid_moves();
            assert!(!moves.is_empty(), "No legal moves");

            let opponent = opponent(game, player);
            let mut alpha = alpha;
            for m in moves {
                let res = match self.min_score(game, state, &opponent, m, alpha, beta, depth + 1) {
                    Ok(score) => score,
                    e @ Err(_) => return e,
                };

                alpha = max(res, alpha);
                if alpha >= beta {
                    return Ok(beta);
                }
                check_time_result!(self, game);
            }
            Ok(alpha)
        }

        fn min_score<T: crate::Playable>(
            &mut self,
            game: &T,
            player: &str,
            last_move: String,
            alpha: u8,
            beta: u8,
            depth: u32,
        ) -> Option<String> {
            let moves = game.get_valid_moves();
            assert!(moves.len() >= 1, "No legal moves");

            let mut beta = beta;
            for m in moves {
                let move_vec = if game.game_static.players[0] == *player {
                    vec![m, last_move.clone()]
                } else {
                    vec![last_move.clone(), m]
                };
                let s = game.next_state(state, &*move_vec);
                let opponent = opponent(game, player);
                let res = match self.max_score(game, &s, &opponent, alpha, beta, depth) {
                    Ok(score) => score,
                    e @ Err(_) => return e,
                };
                beta = min(res, beta);
                if beta <= alpha {
                    return Ok(alpha);
                }
                check_time_result!(self, game);
            }
            Ok(beta)
        }

        fn monte_carlo<T: crate::Playable>(&mut self, player: &str, game: &T) -> Option<String> {
            let mut total: u32 = 0;
            for _ in 0..self.charge_count {
                match self.depth_charge(player, game) {
                    Ok(res) => total += res as u32,
                    Err(e) => return Err(e),
                }
            }
            Ok((total / self.charge_count) as u8)
        }

        fn depth_charge<T: crate::Playable>(&mut self, player: &str, game: &T) -> Option<String> {
            let mut moves = Vec::with_capacity(game.players().len());
            while !game.is_terminal() {
                moves.clear();
                for r in game.players().into_iter() {
                    let mut legals = game.get_valid_moves();
                    let r = rand::random::<usize>() % legals.len();
                    moves.push(legals.swap_remove(r));
                }

                new_state = game.next_state(&new_state, &moves);
                check_time_result!(self, game);
            }
            return Ok(game.goal(state, player));
        }
    }

    fn opponent<'a>(game: &'a Game, player: &'a str) -> &'a str {
        let players = game.players();
        assert!(players.len() == 2, "Must be a two player game");
        let res: Vec<_> = players.into_iter().filter(|r| *r != player).collect();
        assert_eq!(res.len(), 1);
        res[0]
    }

    impl Agentish for PlayerMCS {
        fn get_name(&self) -> String {
            "PlayerMCS".to_string()
        }

        fn get_move<T: crate::Playable>(
            &self,
            moves: &Vec<String>,
            player: &str,
            game: &T,
        ) -> Option<String> {
            let m = match self.best_move(&game) {
                Ok(m) => m,
                Err(m) => {
                    warn!("Out of time");
                    m
                }
            };
            info!("Selecting move {}", m.to_string());
            m
        }
    }

    impl PlayerMCS {
        fn out_of_time(&mut self, _: &Game) -> Move {
            self.best_move.take().unwrap()
        }
    }
}
