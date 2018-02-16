extern crate bytebot_rpn as rpn;
extern crate rand;

use rand::Rng;

use rpn::Cmd;
use rpn::Cmd::*;

use encode::Color;

/// Definitions:
/// A "Bytebeat Unit" is a basic unit of a formula, typically
/// one to three commands in length. These should always end with
/// remaining value on the stack, so that one could place this unit anywhere
/// within a valid bytebeat to get back a valid bytebeat.
/// For example, `t 2 *` is a single unit
/// and the formula `t t 8 >> t >> 4 * consists of three units.
/// In this case, `t`, `t 8 >>`, and `t 4 >>`

/// A "t multiply" type of bytebeat consists of
/// a `t * (expressions)` where `expressions` is
/// any number of `t>>_` or `t` expressions
/// composed with a bitwise operator. For example,
/// `t * ((t>>8) ^ (t>>3) | (t>>2))` is a t multiply bytebeat
/// (In RPN: `t t 8 >> t 3 >> ^ t 2 >> | *`)
/// Grammar:
/// t_multiple := t_like, oscillator, Mul
/// Where t_like and oscillator are defined below
pub fn random_t_multiply(goal_length: usize) -> Vec<Cmd> {
    let t_like = random_t_like(goal_length / 2);
    let oscillator = random_oscillator(goal_length / 2);
    compose(t_like, oscillator, Mul)
}

/// Return a random color
pub fn random_color() -> Color {
    let r = rand::thread_rng().gen_range(0, 255);
    let g = rand::thread_rng().gen_range(0, 255);
    let b = rand::thread_rng().gen_range(0, 255);
    Color([r, g, b])
}

/// Returns a "t like" bytebeat unit with `length` amount of
/// bytebeat units. Length is recommended to be 3 or less.
/// Grammar is below.
/// t_like := unit, {op, unit}
/// unit := Var, num, Shr
/// op := AnyTwoArgOp
/// num := 0...3
fn random_t_like(length: usize) -> Vec<Cmd> {
    let mut vec = random_t_shr(0, 13);
    for _ in 0..length {
        let random_t_like = random_t_shr(0, 3);
        vec = compose(vec, random_t_like, random_two_op());
    }
    vec
}

/// Returns a "oscillator" bytebeat unit with `length` amount of
/// bytebeat units. Length is recommended to be 5 or below.
/// Grammar is below
/// oscillator := unit, {op, unit}
/// unit := Var, num, Shr
/// op := Mul | Mod | And | Orr | Xor
/// num := 0...13
fn random_oscillator(length: usize) -> Vec<Cmd> {
    let mut vec = random_t_shr(0, 13);
    for _ in 0..length {
        let random_t_like = random_t_shr(0, 13);
        vec = compose(vec, random_t_like, choose(vec![Mul, Mod, And, Orr, Xor]));
    }
    vec
}

/// Returns one of [Sub, Mul, Div, Mod, Shl, Shr, And, Orr, Xor]
fn random_two_op() -> Cmd {
    choose(vec![Sub, Mul, Div, Mod, Shl, Shr, And, Orr, Xor])
}

/// Returns `t >> n` (`[Var, Num, Shr]) where `Num` is in range [0, 16]
/// `t >> 0` will be optimised to just `t`
fn random_t_shr(min: i64, max: i64) -> Vec<Cmd> {
    let number = rand::thread_rng().gen_range(min, max);
    match number {
        0 => vec![Var],
        _ => vec![Var, NumI(number), Shr],
    }
}

/// Given bytebeat units `left` and `right`, returns `left right op`
fn compose(mut left: Vec<Cmd>, right: Vec<Cmd>, two_arg_op: Cmd) -> Vec<Cmd> {
    left.extend(right);
    left.push(two_arg_op);
    left
}

#[test]
fn test_compose() {
    let a = vec![Var, NumI(8), Shr];
    let b = vec![Var, NumI(1), Shr];
    let op = And;
    assert_eq!(
        compose(a, b, op),
        vec![Var, NumI(8), Shr, Var, NumI(1), Shr, And]
    );
}

fn choose<T: Clone>(vec: Vec<T>) -> T {
    rand::thread_rng().choose(&vec).unwrap().clone()
}
