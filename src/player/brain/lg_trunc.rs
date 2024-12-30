/// "Layer" har kun en "Cell" i denne versionen av logiske port nettverket.


pub mod lg_trunc {
    #![allow(dead_code)]
    // use env_logger;
    use log::{debug, trace};
    use rand::Rng;
    //use std::char;
    use crate::player::brain::brain::*;
    //use crate::brain::brain::LogicGatesError;
    //use crate::brain::brain::{Cell, Gate};
    use boardgame_game::game::game::GameStatic;
    use std::fs::File;
    use std::io::Read;
    use std::io::Write;
    //use thiserror::Error;

    /// Layer representerer et lag med nevroner
    #[derive(Clone, Debug)]
    pub struct Brain {
        pub game_name: String,
        pub filepath: String,
        pub layers: [Cell; NO_LAYERS],
    }

    impl Brain {
        /// main function. Evaluate state
        pub fn evaluate_bit_state(&self, current_player: &str, state: &Vec<bool>) -> usize {
            let is_current_active_player = state[0] ^ (current_player == "A");
            let mut current_state = vec![false, true, is_current_active_player];
            current_state.extend(state.iter());
            // println!("in:{:?}", current_state);
            for layer_no in 0..NO_LAYERS {
                let mut next_state: Vec<bool> = vec![true; CELL_SIZE];
                let layer = &self.layers[layer_no];
                for gate in 0..CELL_SIZE {
                    let bit_a = current_state[layer.address_a[gate]];
                    let bit_b = current_state[layer.address_b[gate]];
                    next_state[gate] = match layer.operator[gate] {
                        BitOp::FALSE => false, //         0 False 0                       0  0  0  0
                        BitOp::AND => bit_a & bit_b, // 1 A ∧ B A · B                   0  0  0  1
                        BitOp::ANDANB => bit_a & !bit_b, // 2 ¬(A ⇒ B) A − AB               0  0  1  0
                        BitOp::A => bit_a, // 3 A A                           0  0  1  1
                        BitOp::ANDNAB => !bit_a & bit_b, // 4 ¬(A ⇐ B) B − AB               0  1  0  0
                        BitOp::B => bit_b, // 5 B B                           0  1  0  1
                        BitOp::XOR => bit_a ^ bit_b, // 6 A ⊕ B A + B − 2AB             0  1  1  0
                        BitOp::OR => bit_a | bit_b, // 7 A ∨ B A + B − AB              0  1  1  1
                        BitOp::NOR => !(bit_a | bit_b), // 8 ¬(A ∨ B) 1 − (A + B − AB)     1  0  0  0
                        BitOp::NXOR => !(bit_a ^ bit_b), // 9 ¬(A ⊕ B) 1 − (A + B − 2AB)    1  0  0  1
                        BitOp::NB => !bit_b, // 10 ¬B 1 − B                     1  0  1  0
                        BitOp::ORANB => bit_a | !bit_b, //11 A ⇐ B 1 − B + AB             1  0  1  1
                        BitOp::NA => !bit_a, //12 ¬A 1 − A                     1  1  0  0
                        BitOp::ORNAB => !bit_a | bit_b, //13 A ⇒ B 1 − A + AB             1  1  0  1
                        BitOp::NAND => !(bit_a & bit_b), //14 ¬(A ∧ B) 1 − AB              1  1  1  0
                        BitOp::TRUE => true, // 15 True 1                       1  1  1  1
                    };
                }
                // println!("{}: {:?}", layer_no, next_state);
                current_state = next_state;
            }
            // println!("{:?}", current_state);
            current_state.iter().filter(|&x| *x).count()
        }

        #[allow(dead_code)]
        pub fn from_file(&mut self) -> Result<(), LogicGatesError> {
            if self.filepath == "" {
                self.filepath = format!(
                    "data/{}-logic_gate_nn-{}-{}.txt",
                    self.game_name, NO_LAYERS, CELL_SIZE
                );
            }
            // warn!("Filename: {}", &self.filepath);
            let mut logic_gates_nn: File = File::open(&self.filepath)?;
            let mut logic_gates_nn_string = String::new(); // This String will hold it
            logic_gates_nn.read_to_string(&mut logic_gates_nn_string)?; // Read the file into it

            //logic_gates_nn_string.split_ split_whitespace().for_each(|word| println!("{} ", word.to_uppercase())); // Do things with the String now

            // For å få kjørt kjør: rm data/logic_gate_nn.txt
            let mut layer_no = 0;
            let mut gate_no: usize = 0;
            for line in logic_gates_nn_string.lines() {
                let line_string = line.to_string();
                if line_string.starts_with("Layer:") {
                    gate_no = 0;
                    let no_str = line.rsplit_once(" ").unwrap_or_else(|| ("", "")).1;
                    layer_no = no_str
                        .to_string()
                        .parse::<usize>()
                        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
                } else if line_string.eq("Data:") {
                    // do nothing
                } else {
                    match line_string.chars().next() {
                        Some(char) => {
                            if char.is_numeric() {
                                let mut gate_iter = line_string.split(",");
                                let adr_a: usize = gate_iter
                                    .next()
                                    .unwrap_or_else(|| "")
                                    .parse::<usize>()
                                    .map_err(|e| {
                                        std::io::Error::new(std::io::ErrorKind::Other, e)
                                    })?;
                                let adr_b: usize = gate_iter
                                    .next()
                                    .unwrap_or_else(|| "")
                                    .parse::<usize>()
                                    .map_err(|e| {
                                        std::io::Error::new(std::io::ErrorKind::Other, e)
                                    })?;
                                let operator = gate_iter
                                    .next()
                                    .unwrap_or_else(|| "XOR")
                                    .parse::<BitOp>()
                                    .unwrap(); // forstår ikke hvordan bli kvitt unwrap her. (Fikk feil med expect også.)

                                self.layers[layer_no].address_a[gate_no] = adr_a;
                                self.layers[layer_no].address_b[gate_no] = adr_b;
                                self.layers[layer_no].operator[gate_no] = operator;
                            } else {
                                return Err(LogicGatesError::InvalidChar {
                                    char: char,
                                    line_string: line_string,
                                    file: self.filepath.clone(),
                                });
                            }
                        }
                        None => {
                            panic!("Nothing Handle {layer_no}:{gate_no} for {line_string}")
                        }
                    }
                    trace!("Handle {layer_no}:{gate_no} for {line_string}");
                    gate_no = gate_no + 1;
                }
            }
            return Ok(());
        }

        #[allow(dead_code)]
        pub fn save_to_file(&self) -> Result<(), LogicGatesError> {
            {
                // truncate file
                let _ = File::options()
                    .create(true)
                    .write(true)
                    .truncate(true)
                    .open(&self.filepath)?;
                {
                    //  writeln!(&mut f, "")?;
                }
                //
            }
            let mut f = File::options()
                .create(true)
                .write(true)
                .append(true)
                .open(&self.filepath)?;
            debug!("INSIDE SAVE_TO_FILE");
            for layer in 0..NO_LAYERS {
                writeln!(&mut f, "Layer: {}", layer)?;
                writeln!(&mut f, "Data:")?;
                for i in 0..CELL_SIZE {
                    writeln!(
                        &mut f,
                        "{},{},{:?}",
                        self.layers[layer].address_a[i],
                        self.layers[layer].address_b[i],
                        self.layers[layer].operator[i]
                    )?;
                }
            }
            Ok(())
        }

        /// Change random gates. Both type and input addresses.
        /// Do this 1% of gates.
        pub fn do_mutate(&mut self, max_address: &usize) -> () {
            let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
            let total_gates = NO_LAYERS * CELL_SIZE;
            for _i in 0..total_gates / 50 {
                let layer = rng.gen_range(0..NO_LAYERS);
                let gate_address_a = match layer {
                    0 => rng.gen_range(0..*max_address),
                    _ => rng.gen_range(0..CELL_SIZE),
                };
                let gate_address_b = match layer {
                    0 => rng.gen_range(0..*max_address),
                    _ => rng.gen_range(0..CELL_SIZE),
                };
                let gate_id = rng.gen_range(0..CELL_SIZE);
                self.layers[layer].address_a[gate_id] = gate_address_a;
                self.layers[layer].address_b[gate_id] = gate_address_b;
                self.layers[layer].operator[gate_id] = match rng.gen_range(1..15) {
                    0 => BitOp::FALSE,  //         0 False 0                       0  0  0  0
                    1 => BitOp::AND,    // 1 A ∧ B A · B                   0  0  0  1
                    2 => BitOp::ANDANB, // 2 ¬(A ⇒ B) A − AB               0  0  1  0
                    3 => BitOp::A,      // 3 A A                           0  0  1  1
                    4 => BitOp::ANDNAB, // 4 ¬(A ⇐ B) B − AB               0  1  0  0
                    5 => BitOp::B,      // 5 B B                           0  1  0  1
                    6 => BitOp::XOR,    // 6 A ⊕ B A + B − 2AB             0  1  1  0
                    7 => BitOp::OR,     // 7 A ∨ B A + B − AB              0  1  1  1
                    8 => BitOp::NOR,    // 8 ¬(A ∨ B) 1 − (A + B − AB)     1  0  0  0
                    9 => BitOp::NXOR,   // 9 ¬(A ⊕ B) 1 − (A + B − 2AB)    1  0  0  1
                    10 => BitOp::NB,    // 10 ¬B 1 − B                     1  0  1  0
                    11 => BitOp::ORANB, //11 A ⇐ B 1 − B + AB             1  0  1  1
                    12 => BitOp::NA,    //12 ¬A 1 − A                     1  1  0  0
                    13 => BitOp::ORNAB, //13 A ⇒ B 1 − A + AB             1  1  0  1
                    14 => BitOp::NAND,  //14 ¬(A ∧ B) 1 − AB              1  1  1  0
                    15 => BitOp::TRUE,  // 15 True 1                       1  1  1  1

                    x => panic!("gen_range out of range: 0..6 {x}"),
                };
            }
            ()
        }
    }

    /// Layer representerer et lag med nevroner
    /*
        #[derive(Copy, Clone, Debug)]
        pub struct Layer {
            address_a: [usize; CELL_SIZE],
            address_b: [usize; CELL_SIZE],
            operator: [BitOp; CELL_SIZE],
        }
    */
    pub fn get_default_cell() -> Cell {
        return Cell {
            address_a: [0; CELL_SIZE],
            address_b: [0; CELL_SIZE],
            operator: [BitOp::AND; CELL_SIZE],
        };
    }

    #[allow(dead_code)]
    fn generate_random_layer(max_address: &usize) -> Cell {
        let mut retur: Cell = get_default_cell();
        let mut rng: rand::prelude::ThreadRng = rand::thread_rng();
        for i in 0..CELL_SIZE {
            retur.address_a[i] = rng.gen_range(0..*max_address);
            retur.address_b[i] = rng.gen_range(0..*max_address);
            retur.operator[i] = match rng.gen_range(1..15) {
                // rand 0.8
                0 => BitOp::FALSE, //         0 False 0                       0  0  0  0
                1 => BitOp::AND,   // 1 A ∧ B A · B                   0  0  0  1
                2 => BitOp::ANDANB, // 2 ¬(A ⇒ B) A − AB               0  0  1  0
                3 => BitOp::A,     // 3 A A                           0  0  1  1
                4 => BitOp::ANDNAB, // 4 ¬(A ⇐ B) B − AB               0  1  0  0
                5 => BitOp::B,     // 5 B B                           0  1  0  1
                6 => BitOp::XOR,   // 6 A ⊕ B A + B − 2AB             0  1  1  0
                7 => BitOp::OR,    // 7 A ∨ B A + B − AB              0  1  1  1
                8 => BitOp::NOR,   // 8 ¬(A ∨ B) 1 − (A + B − AB)     1  0  0  0
                9 => BitOp::NXOR,  // 9 ¬(A ⊕ B) 1 − (A + B − 2AB)    1  0  0  1
                10 => BitOp::NB,   // 10 ¬B 1 − B                     1  0  1  0
                11 => BitOp::ORANB, //11 A ⇐ B 1 − B + AB             1  0  1  1
                12 => BitOp::NA,   //12 ¬A 1 − A                     1  1  0  0
                13 => BitOp::ORNAB, //13 A ⇒ B 1 − A + AB             1  1  0  1
                14 => BitOp::NAND, //14 ¬(A ∧ B) 1 − AB              1  1  1  0
                15 => BitOp::TRUE, // 15 True 1                       1  1  1  1
                x => panic!("gen_range out of range: 0..6 {x}"),
            };
        }
        //println!("{:?}", retur);
        return retur;
    }

    #[allow(dead_code)]
    pub fn generate_random_brain(game: &GameStatic) -> Brain {
        //game: crate::game::game::GameStatic
        let mut brain = Brain {
            game_name: game.name.clone(),
            filepath: format!(
                "data/{}-logic_gate_nn-{}-{}.txt",
                game.name, NO_LAYERS, CELL_SIZE
            ),
            layers: [get_default_cell(); NO_LAYERS],
        };
        for i in 0..NO_LAYERS {
            if i == 0 {
                let state_size = game.get_state_size();
                brain.layers[i] = generate_random_layer(&state_size);
            } else {
                brain.layers[i] = generate_random_layer(&CELL_SIZE);
            }
        }

        return brain;
    }
}
