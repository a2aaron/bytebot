use rand::{self, Rng};
use super::Cmd;
use Cmd::*;

static OPERATORS: [Cmd; 15] = [
    Var, // Var is used as a stand in for Var or Num
    Var,
    Var,
    Var,
    Var,
    Var,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    And,
    Orr,
    Xor,
];
static OPERATORS2: [Cmd; 9] = [Sub, Mul, Div, Mod, Shl, Shr, And, Orr, Xor];

pub fn random_beat(goal_length: usize) -> Vec<Cmd> {
    let mut vec = Vec::new();
    let mut num_args = 0;
    while vec.len() < goal_length || num_args > 1 {
        let mut random_cmd = match num_args {
            0 | 1 => Var,
            _ => {
                if vec.len() > goal_length {
                    *rand::thread_rng().choose(&OPERATORS2).unwrap()
                } else {
                    *rand::thread_rng().choose(&OPERATORS).unwrap()
                }
            }
        };

        if let Var = random_cmd {
            random_cmd = random_value();
        }

        num_args += match random_cmd {
            Var | Num(_) => 1,
            _ => -1,
        };

        vec.push(random_cmd);
    }
    vec
}

/// Returns either a Var or a Num in the range [0, 256)
fn random_value() -> Cmd {
    let number = rand::thread_rng().gen_range::<i32>(0, 256);
    *rand::thread_rng().choose(&[Cmd::Var, Cmd::Num(number as f64)]).unwrap()
}