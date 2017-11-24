pub mod encode;

use encode::Color;


#[derive(Debug, PartialEq)]
pub enum Cmd {
    Var,
    Num(f64),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Shl,
    Shr,
    And,
    Orr,
    Xor,
    Sin,
    Cos,
    Tan,
    Pow,
    AddF,
    SubF,
    MulF,
    DivF,
    ModF,
    Lt,
    Gt,
    Leq,
    Geq,
    Eq,
    Neq,
    Cond,
    Arr(usize),
    Fg(Color),
    Bg(Color),
    Khz(u8),
}

pub struct Program {
    code: Vec<Cmd>,
    bg: Option<Color>,
    fg: Option<Color>,
    khz: Option<u8>,
}

impl Program {
    pub fn bg(&self) -> Option<Color> {
        self.bg
    }

    pub fn fg(&self) -> Option<Color> {
        self.fg
    }

    pub fn hz(&self) -> Option<u32> {
        self.khz.map(|x| x as u32 * 1000)
    }
}

pub fn compile(cmds: Vec<Cmd>) -> Result<Program, ()> {
    use Cmd::*;
    let (mut bg, mut fg, mut khz) = (None, None, None);
    for cmd in &cmds {
        match *cmd {
            Bg(col) => bg = Some(col),
            Fg(col) => fg = Some(col),
            Khz(k) => khz = Some(k),
            _ => (),
        }
    }

    Ok(Program {
        code: cmds,
        bg,
        fg,
        khz,
    })
}

impl std::fmt::Display for Program {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", format_beat(&self.code))
    }
}

pub fn eval_beat(program: &Program, t: f64) -> Result<f64, ()> {
    use Cmd::*;
    let mut stack = Vec::new();
    for cmd in &program.code {
        match *cmd {
            Var => stack.push(t),
            Num(y) => stack.push(y),
            Add => {
                let b = stack.pop().ok_or(())? as i64;
                let a = stack.pop().ok_or(())? as i64;
                stack.push(a.wrapping_add(b) as f64);
            }
            Sub => {
                let b = stack.pop().ok_or(())? as i64;
                let a = stack.pop().ok_or(())? as i64;
                stack.push(a.wrapping_sub(b) as f64);
            }
            Mul => {
                let b = stack.pop().ok_or(())? as i64;
                let a = stack.pop().ok_or(())? as i64;
                stack.push(a.wrapping_mul(b) as f64);
            }
            Div => {
                let b = stack.pop().ok_or(())? as i64;
                let a = stack.pop().ok_or(())? as i64;
                if b == 0 {
                    stack.push(0.0);
                } else {
                    stack.push(a.wrapping_div(b) as f64);
                }
            }
            Mod => {
                let b = stack.pop().ok_or(())? as i64;
                let a = stack.pop().ok_or(())? as i64;
                if b == 0 {
                    stack.push(0.0);
                } else {
                    stack.push(a.wrapping_rem(b) as f64);
                }
            }
            Shl => {
                let mut b = stack.pop().ok_or(())? as i64 % 64;
                let a = stack.pop().ok_or(())? as i64;
                if b < 0 {
                    b += 64;
                }
                stack.push((a << b) as f64);
            }
            Shr => {
                let mut b = stack.pop().ok_or(())? as i64 % 64;
                let a = stack.pop().ok_or(())? as i64;
                if b < 0 {
                    b += 64;
                }
                stack.push((a >> b) as f64);
            }
            And => {
                let b = stack.pop().ok_or(())? as i64;
                let a = stack.pop().ok_or(())? as i64;
                stack.push((a & b) as f64);
            }
            Orr => {
                let b = stack.pop().ok_or(())? as i64;
                let a = stack.pop().ok_or(())? as i64;
                stack.push((a | b) as f64);
            }
            Xor => {
                let b = stack.pop().ok_or(())? as i64;
                let a = stack.pop().ok_or(())? as i64;
                stack.push((a ^ b) as f64);
            }
            Sin => {
                let a = stack.pop().ok_or(())?;
                stack.push(a.sin());
            }
            Cos => {
                let a = stack.pop().ok_or(())?;
                stack.push(a.cos());
            }
            Tan => {
                let a = stack.pop().ok_or(())?;
                stack.push(a.tan());
            }
            Pow => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push(a.powf(b));
            }
            AddF => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push(a + b);
            }
            SubF => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push(a - b);
            }
            MulF => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push(a * b);
            }
            DivF => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                if b == 0.0 {
                    stack.push(0.0);
                } else {
                    stack.push(a / b);
                }
            }
            ModF => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                if b == 0.0 {
                    stack.push(0.0);
                } else {
                    stack.push(a % b);
                }
            }
            Lt => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push((a < b) as i64 as f64);
            }
            Gt => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push((a > b) as i64 as f64);
            }
            Leq => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push((a <= b) as i64 as f64);
            }
            Geq => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push((a >= b) as i64 as f64);
            }
            Eq => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push((a == b) as i64 as f64);
            }
            Neq => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push((a != b) as i64 as f64);
            }
            Cond => {
                let cond = stack.pop().ok_or(())?;
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                if cond != 0.0 {
                    stack.push(a);
                } else {
                    stack.push(b);
                }

            }
            Arr(size) => {
                let index = stack.pop().ok_or(())? as i64;
                // We need to pop `size` values, so our stack needs to be
                // atleast size elements long. Note that split_off panics if we
                // exceed the length of the vector, so we need this if guard.
                if size > stack.len() {
                    return Err(());
                } else if size == 0 {
                    stack.push(0.0);
                } else {
                    // We want to split off from the end, so we must subtract here.
                    let split_index = stack.len() - size;
                    let mut vec = stack.split_off(split_index);
                    let size = size as i64;
                    // Calculate the positive modulus (% gives remainder, which
                    // is slightly different than mod for negative values)
                    let index = ((index % size) + size) % size;
                    stack.push(vec[index as usize]);
                }
            }
            // These have no runtime effect
            Fg(..) | Bg(..) | Khz(..) => (),
        }
    }
    stack.pop().ok_or(())
}

pub fn parse_beat(text: &str) -> Result<Vec<Cmd>, &str> {
    use Cmd::*;
    text.split_whitespace()
        .map(|x| match x {
            "t" => Ok(Var),
            "+" => Ok(Add),
            "-" => Ok(Sub),
            "*" => Ok(Mul),
            "/" => Ok(Div),
            "%" => Ok(Mod),
            "<<" => Ok(Shl),
            ">>" => Ok(Shr),
            "&" => Ok(And),
            "|" => Ok(Orr),
            "^" => Ok(Xor),
            "sin" => Ok(Sin),
            "cos" => Ok(Cos),
            "tan" => Ok(Tan),
            "pow" => Ok(Pow),
            "+." => Ok(AddF),
            "-." => Ok(SubF),
            "*." => Ok(MulF),
            "/." => Ok(DivF),
            "%." => Ok(ModF),
            "<" => Ok(Lt),
            ">" => Ok(Gt),
            "<=" => Ok(Leq),
            ">=" => Ok(Geq),
            "==" => Ok(Eq),
            "!=" => Ok(Neq),
            "?" => Ok(Cond),
            x if x.starts_with('[') => x[1..].parse().map(Arr).map_err(|_| x),
            x if x.starts_with("!fg:") => {
                let raw = u16::from_str_radix(&x[4..], 16).map_err(|_| x)?;
                let r = (raw >> 8 & 0xF) as u8;
                let g = (raw >> 4 & 0xF) as u8;
                let b = (raw & 0xF) as u8;
                Ok(Fg(Color([r << 4 | r, g << 4 | g, b << 4 | b])))
            }
            x if x.starts_with("!bg:") => {
                let raw = u16::from_str_radix(&x[4..], 16).map_err(|_| x)?;
                let r = (raw >> 8 & 0xF) as u8;
                let g = (raw >> 4 & 0xF) as u8;
                let b = (raw & 0xF) as u8;
                Ok(Bg(Color([r << 4 | r, g << 4 | g, b << 4 | b])))
            }
            x if x.starts_with("!khz:") => x[5..].parse().map(Khz).map_err(|_| x),
            x => x.parse().map(Num).map_err(|_| x),
        })
        .collect()
}

impl std::fmt::Display for Cmd {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Cmd::*;
        match *self {
            Var => write!(fmt, "t"),
            Num(y) => write!(fmt, "{}", y),
            Add => write!(fmt, "+"),
            Sub => write!(fmt, "-"),
            Mul => write!(fmt, "*"),
            Div => write!(fmt, "/"),
            Mod => write!(fmt, "%"),
            Shl => write!(fmt, "<<"),
            Shr => write!(fmt, ">>"),
            And => write!(fmt, "&"),
            Orr => write!(fmt, "|"),
            Xor => write!(fmt, "^"),
            Sin => write!(fmt, "sin"),
            Cos => write!(fmt, "cos"),
            Tan => write!(fmt, "tan"),
            Pow => write!(fmt, "pow"),
            AddF => write!(fmt, "+."),
            SubF => write!(fmt, "-."),
            MulF => write!(fmt, "*."),
            DivF => write!(fmt, "/."),
            ModF => write!(fmt, "%."),
            Lt => write!(fmt, "<"),
            Gt => write!(fmt, ">"),
            Leq => write!(fmt, "<="),
            Geq => write!(fmt, ">="),
            Eq => write!(fmt, "=="),
            Neq => write!(fmt, "!="),
            Cond => write!(fmt, "?"),
            Arr(size) => write!(fmt, "[{}", size),
            Fg(col) => {
                write!(
                    fmt,
                    "!fg:{:X}{:X}{:X}",
                    col.0[0] & 0xF,
                    col.0[1] & 0xF,
                    col.0[2] & 0xF
                )
            }
            Bg(col) => {
                write!(
                    fmt,
                    "!bg:{:X}{:X}{:X}",
                    col.0[0] & 0xF,
                    col.0[1] & 0xF,
                    col.0[2] & 0xF
                )
            }
            Khz(khz) => write!(fmt, "!khz:{}", khz),
        }
    }
}

pub fn format_beat(cmds: &[Cmd]) -> String {
    cmds.iter()
        .map(|cmd| format!("{}", cmd))
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_beat {
        (
            name: $name:ident,
            text: $text:expr,
            code: [$($cmd:expr),* $(,)*],
            eval: { $($src:expr => $res:expr),* $(,)* } $(,)*
        ) => {
            mod $name {
                use super::*;

                #[test]
                fn test_eval() {
                    use Cmd::*;
                    let cmd = vec![$($cmd),*];
                    let cmd = compile(cmd).unwrap();
                    $(
                        assert_eq!(eval_beat(&cmd, $src), Ok($res), "t = {}, cmd: {}", $src, $text);
                    )*
                }

                #[test]
                fn test_format() {
                    use Cmd::*;
                    let cmd = [$($cmd),*];
                    assert_eq!(format_beat(&cmd), $text);
                }

                #[test]
                fn test_parse() {
                    use Cmd::*;
                    let cmd = vec![$($cmd),*];
                    assert_eq!(parse_beat($text), Ok(cmd));
                }
            }
        }
    }

    test_beat! {
        name: var,
        text: "t",
        code: [Var],
        eval: {
            0.0 => 0.0,
            0.5 => 0.5,
        }
    }

    test_beat! {
        name: num,
        text: "42.69",
        code: [Num(42.69)],
        eval: {
            0.0 => 42.69,
            13.0 => 42.69,
        }
    }

    test_beat! {
        name: add,
        text: "t t +",
        code: [Var, Var, Add],
        eval: {
            0.5 => 0.0,
            1.0 => 2.0,
        }
    }

    test_beat! {
        name: sub,
        text: "t 1 -",
        code: [Var, Num(1.0), Sub],
        eval: {
            0.0 => -1.0,
            0.6 => -1.0,
            1.5 => 0.0,
        }
    }

    test_beat! {
        name: mul,
        text: "t t *",
        code: [Var, Var, Mul],
        eval: {
            0.5 => 0.0,
            2.5 => 4.0,
            3.0 => 9.0,
        }
    }

    test_beat! {
        name: div,
        text: "t 2 /",
        code: [Var, Num(2.0), Div],
        eval: {
            0.0 => 0.0,
            1.0 => 0.0,
            2.0 => 1.0,
            3.0 => 1.0,
            4.0 => 2.0,
        }
    }

    test_beat! {
        name: div_0,
        text: "1 0 /",
        code: [Num(1.0), Num(0.0), Div],
        eval: { 1.0 => 0.0 },
    }

    test_beat! {
        name: rem,
        text: "t 7 %",
        code: [Var, Num(7.0), Mod],
        eval: {
            -8.0 => -1.0,
            -1.0 => -1.0,
            0.0 => 0.0,
            5.0 => 5.0,
            8.0 => 1.0,
        }
    }

    test_beat! {
        name: rem_0,
        text: "7 0 %",
        code: [Num(7.0), Num(0.0), Mod],
        eval: { 1.0 => 0.0 },
    }

    test_beat! {
        name: shl,
        text: "1.1 t <<",
        code: [Num(1.1), Var, Shl],
        eval: {
            -60.0 => 16.0,
            0.0 => 1.0,
            0.6 => 1.0,
            1.0 => 2.0,
            8.3 => 256.0,
            65.0 => 2.0,
        }
    }

    test_beat! {
        name: shr,
        text: "1024.1 t >>",
        code: [Num(1024.1), Var, Shr],
        eval: {
            -60.0 => 64.0,
            0.0 => 1024.0,
            0.2 => 1024.0,
            1.0 => 512.0,
            4.0 => 64.0,
            4.7 => 64.0,
            11.0 => 0.0,
        }
    }

    test_beat! {
        name: and,
        text: "1 2 &",
        code: [Num(1.0), Num(2.0), And],
        eval: { 1.0 => 0.0 },
    }

    test_beat! {
        name: orr,
        text: "1 2 |",
        code: [Num(1.0), Num(2.0), Orr],
        eval: { 1.0 => 3.0 },
    }

    test_beat! {
        name: xor,
        text: "t -1 ^",
        code: [Var, Num(-1.0), Xor],
        eval: {
            0.0 => -1.0,
            3.2 => (!3i64) as f64,
        }
    }

    test_beat! {
        name: addf,
        text: "0.1 t +.",
        code: [Num(0.1), Var, AddF],
        eval: {
            0.0 => 0.1,
            -2.0 => -1.9,
            1.0 => 1.1,
        }
    }

    test_beat! {
        name: subf,
        text: "0 t -.",
        code: [Num(0.0), Var, SubF],
        eval: {
            -6.9 => 6.9,
            0.0 => 0.0,
            4.2 => -4.2,
        }
    }

    test_beat! {
        name: mulf,
        text: "7.5 t *.",
        code: [Num(7.5), Var, MulF],
        eval: {
            4.2 => 31.5,
            0.0 => 0.0,
            -6.9 => -51.75,
        }
    }

    test_beat! {
        name: divf,
        text: "7.5 t /.",
        code: [Num(7.5), Var, DivF],
        eval: {
            4.2 => 1.7857142857142856,
            0.0 => 0.0,
            -6.9 => -1.0869565217391304,
        }
    }

    test_beat! {
        name: modf,
        text: "7.5 t %.",
        code: [Num(7.5), Var, ModF],
        eval: {
            4.2 => 3.3,
            0.0 => 0.0,
            -6.9 => 0.5999999999999996,
        }
    }

    test_beat! {
        name: lt,
        text: "t 64 <",
        code: [Var, Num(64.0), Lt],
        eval: {
            0.0 => 1.0,
            64.0 => 0.0,
            128.0 => 0.0,
        }
    }

    test_beat! {
        name: gt,
        text: "t 64 >",
        code: [Var, Num(64.0), Gt],
        eval: {
            0.0 => 0.0,
            64.0 => 0.0,
            128.0 => 1.0,
        }
    }

    test_beat! {
        name: leq,
        text: "t 64 <=",
        code: [Var, Num(64.0), Leq],
        eval: {
            0.0 => 1.0,
            64.0 => 1.0,
            128.0 => 0.0,
        }
    }

    test_beat! {
        name: geq,
        text: "t 64 >=",
        code: [Var, Num(64.0), Geq],
        eval: {
            0.0 => 0.0,
            64.0 => 1.0,
            128.0 => 1.0,
        }
    }

    test_beat! {
        name: eq,
        text: "t 64 ==",
        code: [Var, Num(64.0), Eq],
        eval: {
            0.0 => 0.0,
            64.0 => 1.0,
            128.0 => 0.0,
        }
    }

    test_beat! {
        name: neq,
        text: "t 64 !=",
        code: [Var, Num(64.0), Neq],
        eval: {
            0.0 => 1.0,
            64.0 => 0.0,
            128.0 => 1.0,
        }
    }

    test_beat! {
        name: cond,
        text: "3 2 t ?",
        code: [Num(3.0), Num(2.0), Var, Cond],
        eval: {
            0.0 => 2.0,
            1.0 => 3.0,
            2.0 => 3.0,
        }
    }

    test_beat! {
        name: color,
        text: "!fg:F00 !bg:00F 0",
        code: [Fg(Color([0xFF, 0x00, 0x00])), Bg(Color([0x00, 0x00, 0xFF])), Num(0.0)],
        eval: { 0.0 => 0.0 },
    }

    test_beat! {
        name: khz,
        text: "!khz:8 8000",
        code: [Khz(8), Num(8000.0)],
        eval: { 0.0 => 8000.0 },
    }

    #[test]
    fn test_metadata() {
        use Cmd::*;
        let prog = compile(vec![
            Fg(Color([0, 0, 0])),
            Bg(Color([0, 0, 0])),
            Khz(8),
            Khz(11),
            Fg(Color([1, 0, 0])),
            Bg(Color([0, 1, 1])),
        ]).unwrap();
        assert_eq!(prog.hz(), Some(11_000));
        assert_eq!(prog.fg(), Some(Color([1, 0, 0])));
        assert_eq!(prog.bg(), Some(Color([0, 1, 1])));
    }

    test_beat! {
        name: even,
        text: "3 2 t 2 % 0 == ?",
        code: [Num(3.0), Num(2.0), Var, Num(2.0), Mod, Num(0.0), Eq, Cond],
        eval: {
            0.0 => 3.0,
            1.0 => 2.0,
            2.0 => 3.0,
            3.0 => 2.0,
            4.0 => 3.0,
            5.0 => 2.0,
        }
    }

    test_beat! {
        name: circle,
        text: "t sin t sin *. t cos t cos *. +.",
        code: [Var, Sin, Var, Sin, MulF, Var, Cos, Var, Cos, MulF, AddF],
        eval: {
            0.0 => 1.0,
            0.5 => 1.0,
            13.0 => 1.0,
        }
    }

    test_beat! {
        name: tan_ratio,
        text: "t sin t cos /. t tan -.",
        code: [Var, Sin, Var, Cos, DivF, Var, Tan, SubF],
        eval: {
            0.0 => 0.0,
            0.5 => 0.0,
            13.0 => 0.0,
        }
    }

    test_beat! {
        name: pow,
        text: "t 3.5 pow",
        code: [Var, Num(3.5), Pow],
        eval: {
            0.0 => 0.0,
            1.0 => 1.0,
            2.0 => 11.313708498984761,
            2.7 => 32.34246929812256,
        }
    }

    test_beat! {
        name: arr,
        text: "1 2 3 t [3",
        code: [Num(1.0), Num(2.0), Num(3.0), Var, Arr(3)],
        eval: {
            -4.0 => 3.0,
            -3.0 => 1.0,
            -2.0 => 2.0,
            -1.0 => 3.0,
            0.0 => 1.0,
            1.0 => 2.0,
            2.0 => 3.0,
            3.0 => 1.0,
            4.0 => 2.0,
            5.0 => 3.0,
            6.0 => 1.0,
        }
    }

    test_beat! {
        name: arr_pops,
        text: "10 1 2 3 t [3 +",
        code: [Num(10.0), Num(1.0), Num(2.0), Num(3.0), Var, Arr(3), Add],
        eval: {
            0.0 => 11.0,
            1.0 => 12.0,
            2.0 => 13.0,
        }
    }

    test_beat! {
        name: arr_pushes_zero,
        text: "1 2 3 [0",
        code: [Num(1.0), Num(2.0), Num(3.0), Arr(0)],
        eval: {
            0.0 => 0.0,
            1.0 => 0.0,
            2.0 => 0.0,
            3.0 => 0.0,
            4.0 => 0.0,
        }
    }

    #[test]
    fn test_arr_stack_too_small() {
        use Cmd::*;
        let cmd = compile(vec![Num(1.0), Num(2.0), Num(3.0), Arr(3)]).unwrap();
        assert_eq!(eval_beat(&cmd, 0.0), Err(()), "t = {}, cmd: {}", 0.0, cmd);
    }

    #[test]
    fn test_eval_empty_arr_is_err() {
        use Cmd::*;
        let cmd = compile(vec![Arr(0)]).unwrap();
        assert_eq!(eval_beat(&cmd, 0.0), Err(()), "t = {}, cmd: {}", 0.0, cmd);
        assert_eq!(eval_beat(&cmd, 1.0), Err(()), "t = {}, cmd: {}", 1.0, cmd);
        assert_eq!(eval_beat(&cmd, 2.0), Err(()), "t = {}, cmd: {}", 2.0, cmd);
    }

    test_beat! {
        name: example1,
        text: "t 1 >> t | tan 128 +",
        code: [Var, Num(1.0), Shr, Var, Orr, Tan, Num(128.0), Add],
        eval: { 1.0 => 129.0 },
    }

    test_beat! {
        name: example2,
        text: "t 1 >> t | tan 128 +.",
        code: [Var, Num(1.0), Shr, Var, Orr, Tan, Num(128.0), AddF],
        eval: { 1.0 => 129.5574077246549 },
    }

    test_beat! {
        name: example3,
        text: "2.5 1.2 %.",
        code: [Num(2.5), Num(1.2), ModF],
        eval: { 0.0 => 0.10000000000000009 },
    }

    test_beat! {
        name: example4,
        text: "t cos t cos *. t sin t sin *. +.",
        code: [Var, Cos, Var, Cos, MulF, Var, Sin, Var, Sin, MulF, AddF],
        eval: { 0.4 => 1.0 },
    }

    test_beat! {
        name: example5,
        text: "t 10 / t 2 t 10 >> pow * sin + sin 64 * 128 +",
        code: [
            Var,
            Num(10.0),
            Div,
            Var,
            Num(2.0),
            Var,
            Num(10.0),
            Shr,
            Pow,
            Mul,
            Sin,
            Add,
            Sin,
            Num(64.0),
            Mul,
            Num(128.0),
            Add,
        ],
        eval: { 3.0 => 128.0 },
    }

    test_beat! {
        name: example6,
        text: "t 10 /. t 2 t 10 >> pow *. sin +. sin 64 *. 128 +.",
        code: [
            Var,
            Num(10.0),
            DivF,
            Var,
            Num(2.0),
            Var,
            Num(10.0),
            Shr,
            Pow,
            MulF,
            Sin,
            AddF,
            Sin,
            Num(64.0),
            MulF,
            Num(128.0),
            AddF,
        ],
        eval: { 3.0 => 155.324961718789 },
    }
}
