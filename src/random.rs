use rand::{self, Rng};
use super::Cmd;
use Cmd::*;

#[derive(Clone, Copy, PartialEq)]
enum Template {
    TRShift,
    Operation,
}

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

/// A "t multiply" type of bytebeat consists of 
/// a `t * (expressions)` where `expressions` is
/// any number of `t>>_` or `t` expressions 
/// composed with a bitwise operator. For example,
/// `t * ((t>>8) ^ (t>>3) | (t>>2))` is a t multiply bytebeat
/// (In RPN: `t t 8 >> t 3 >> ^ t 2 >> | *`)
pub fn random_t_multiply(goal_length: usize) -> Vec<Cmd> {
    let mut vec = Vec::new();
    vec.push(Var);
    let mut num_args = 1;
    while vec.len() < goal_length || num_args > 2 {
        let template = match num_args {
            x if x >= 3 => {
                // Stop making the stack longer once
                // we hit our goal length.
                if vec.len() > goal_length {
                    Template::Operation
                } else {
                    *choose(&[Template::TRShift, Template::Operation])
                }
            },
            // We need to have atleast 3 avaliable values on the stack
            // (1 for the inital t, and 2 for an operation)
             _ => Template::TRShift,
        };

        let random_cmds = match template {
            Template::TRShift => random_t_shr(),
            Template::Operation => vec!(random_op()),
        };

        num_args += match template {
            Template::TRShift => 1,
            Template::Operation => -1,
        };

        vec.extend(random_cmds);
    }
    vec.push(Mul);
    vec
}

pub fn random_beat(goal_length: usize) -> Vec<Cmd> {
    let mut vec = Vec::new();
    let mut num_args = 0;
    while vec.len() < goal_length || num_args > 1 {
        let mut random_cmd = match num_args {
            0 | 1 => Var,
            _ => {
                if vec.len() > goal_length {
                    random_op()
                } else {
                    *choose(&OPERATORS)
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

/// Returns one of [Sub, Mul, Div, Mod, Shl, Shr, And, Orr, Xor]
fn random_op() -> Cmd {
    *choose(&[Sub, Mul, Div, Mod, Shl, Shr, And, Orr, Xor])
}

/// Returns `t >> n` (`[Var, Num, Shr]) where `Num` is in range [0, 16]
/// `t >> 0` will be optimised to just `t`
fn random_t_shr() -> Vec<Cmd> {
    let number = rand::thread_rng().gen_range::<i32>(0, 17);
    match number {

        0 => vec!(Var),
        _ => vec!(Var, Num(number as f64), Shr),
    }
}

/// Returns either a Var or a Num in the range [0, 256)
fn random_value() -> Cmd {
    let number = rand::thread_rng().gen_range::<i32>(0, 256);
    *choose(&[Cmd::Var, Cmd::Num(number as f64)])
}

fn choose<T>(vec: &[T]) -> &T {
    rand::thread_rng().choose(vec).unwrap()
}