pub mod lg_diamond;
pub mod lg_trunc;

pub mod brain {
    pub const NO_LAYERS: usize = 6; //1++
    pub const CELL_SIZE: usize = 256;
    use std::io;
    use std::str::FromStr;
    use thiserror::Error;

    #[derive(Debug, Error)]
    pub enum LogicGatesError {
        #[error("Unknown Bit Operator {found}")]
        InvalidBitOperator { found: String },
        #[error("IO error")]
        Io {
            #[from]
            source: io::Error,
        },
        #[error("Expected digit got char {char}, at line {line_string} in file: {file}")]
        InvalidChar {
            char: char,
            line_string: String,
            file: String,
        },
        #[error("General error: {message}")]
        General {
            message: String,
        }
    }

    #[derive(Copy, Clone, Debug)] //Need copy because of setting default gate
    pub enum BitOp {
        //                                        00 01 10 11
        FALSE,  // 0,False,0                       0  0  0  0
        AND,    // 1,A ∧ B,A · B                   0  0  0  1
        ANDANB, // 2,¬(A ⇒ B),A − AB               0  0  1  0
        A,      // 3,A,A                           0  0  1  1
        ANDNAB, // 4,¬(A ⇐ B),B − AB               0  1  0  0
        B,      // 5,B,B                           0  1  0  1
        XOR,    // 6,A ⊕ B,A + B − 2AB             0  1  1  0
        OR,     // 7,A ∨ B,A + B − AB              0  1  1  1
        NOR,    // 8,¬(A ∨ B),1 − (A + B − AB)     1  0  0  0
        NXOR,   // 9,¬(A ⊕ B),1 − (A + B − 2AB)    1  0  0  1
        NB,     //10,¬B,1 − B                      1  0  1  0
        ORANB,  //11,A ⇐ B,1 − B + AB              1  0  1  1
        NA,     //12,¬A,1 − A                      1  1  0  0
        ORNAB,  //13,A ⇒ B,1 − A + AB              1  1  0  1
        NAND,   //14,¬(A ∧ B),1 − AB               1  1  1  0
        TRUE,   //15,True,1                        1  1  1  1
    }

    impl FromStr for BitOp {
        type Err = LogicGatesError;

        fn from_str(input: &str) -> Result<BitOp, LogicGatesError> {
            match input {
                "FALSE" => Ok(BitOp::FALSE),
                "AND" => Ok(BitOp::AND),
                "ANDANB" => Ok(BitOp::ANDANB),
                "A" => Ok(BitOp::A),
                "ANDNAB" => Ok(BitOp::ANDNAB),
                "B" => Ok(BitOp::B),
                "XOR" => Ok(BitOp::XOR),
                "OR" => Ok(BitOp::OR),
                "NOR" => Ok(BitOp::NOR),
                "NXOR" => Ok(BitOp::NXOR),
                "NB" => Ok(BitOp::NB),
                "ORANB" => Ok(BitOp::ORANB),
                "NA" => Ok(BitOp::NA),
                "ORNAB" => Ok(BitOp::ORNAB),
                "NAND" => Ok(BitOp::NAND),
                "TRUE" => Ok(BitOp::TRUE),
                x => Err(LogicGatesError::InvalidBitOperator {
                    found: x.to_string(),
                }),
            }
        }
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Cell {
        pub address_a: [usize; CELL_SIZE],
        pub address_b: [usize; CELL_SIZE],
        pub operator: [BitOp; CELL_SIZE],
    }
}
