// use crate::player::player::*;
// use crate::player::Agentish;

use crate::player::*;
use boardgame_game::game::*;
use std::fmt::Debug;
use std::fmt::Display;
use std::io;
use std::io::stdout;
use std::io::BufRead;
use std::io::Write;

#[derive(Clone, Default)]
pub struct PlayerHuman {
    pub name: String,
}
impl Agentish for PlayerHuman {
    fn get_name(&self) -> String {
        self.name.to_string()
    }
    fn get_move<T: Playable + Display>(
        &self,
        moves: &Vec<String>,
        _active_player: &str,
        game: &T,
    ) -> Option<String> {
        loop {
            game.pretty_print();
            print!("Ditt(B) flytt. Du kan velge{:?}: ", moves);
            let _ = stdout().flush();
            let mut line = String::new();
            let stdin = io::stdin();
            stdin
                .lock()
                .read_line(&mut line)
                .expect("Problemer med å lese fra shell");
            let line_s = line.trim_end();
            if moves.iter().find(|&x| x.eq(&line_s)) != None {
                return Some(line_s.to_string());
            }
            println!(
                "Feil valg! Du gjore '{}'. Gyldige valg er {:?}",
                line, moves
            );
        }

        // return hmove;
    }

    fn get_ready(
        &mut self,
        _game_static: &GameStatic,
        _me_color: &str,
    ) -> Result<(), crate::player::brain::brain::LogicGatesError> {
        Ok(())
    }
}
impl Debug for PlayerHuman {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
