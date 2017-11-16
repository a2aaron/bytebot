use rand::{self, Rng};
use super::Cmd;
use Cmd::*;

static OPERATORS: [Cmd; 15] = [
    Var,
    Var,
    Var,
    Var,
    Num(-1.0),
    Num(-1.0),
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
static VALUES: [Cmd; 1] = [Var];

pub fn random_beat(goal_length: usize) -> Vec<Cmd> {
    let mut vec = Vec::new();
    let mut num_args = 0;
    while vec.len() < goal_length || num_args > 1 {
        let mut random_cmd = *match num_args {
            0 | 1 => rand::thread_rng().choose(&VALUES).unwrap(),
            _ => {
                if vec.len() > goal_length {
                    rand::thread_rng().choose(&OPERATORS2).unwrap()
                } else {
                    rand::thread_rng().choose(&OPERATORS).unwrap()
                }
            }
        };

        num_args += match random_cmd {
            Var | Num(_) => 1,
            _ => -1,
        };

        if let Num(_) = random_cmd {
            let random_number = (rand::thread_rng().gen::<i32>() % 256).abs();
            random_cmd = Num(random_number as f64);
        }

        vec.push(random_cmd);
    }
    vec
}
