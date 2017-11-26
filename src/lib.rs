pub mod encode;

use encode::Color;


#[derive(Clone, Debug, PartialEq)]
pub enum Cmd {
    Var,
    NumF(f64),
    NumI(i64),
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
    Comment(String),
}

pub struct Program {
    cmds: Vec<Cmd>,
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

pub fn compile(cmds: Vec<Cmd>) -> Result<Program, CompileError> {
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
    // Validate the bytebeat by checking that the stack does not get popped when empty
    let mut stack_size = 0 as isize;
    for (i, cmd) in cmds.iter().enumerate() {
        stack_size += match *cmd {
            Var | NumF(_) | NumI(_) => 1,
            Fg(_) | Bg(_) | Khz(_) | Comment(_) => continue,
            // These all pop 1 value off the stack and push 1
            // value back on, so the net effect is no stack change
            Sin | Cos | Tan => 0,
            // Arr(x) pops a value off the stack (called the index)
            // then pops x more values off the stack. Finally, it
            // pushes one value back onto the stack based on the index
            // Thus the net effect of Arr is to reduce the stack size by x.
            Arr(x) => -(x as isize),
            Cond => -2,
            // Split these into multiple branches to make rustfmt stop complaining
            Add | Sub | Mul | Div | Mod => -1,
            Shl | Shr | And | Orr | Xor => -1,
            Pow | AddF | SubF | MulF | DivF | ModF => -1,
            Lt | Gt | Leq | Geq | Eq | Neq => -1,
        };
        if stack_size <= 0 {
            // Hand back the Vec<Cmd> since we probably shouldn't drop it.
            return Err(CompileError {
                cmds: cmds.clone(),
                index: i,
                stack_size,
            });
        }
    }
    Ok(Program { cmds, bg, fg, khz })
}

impl std::fmt::Display for Program {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", format_beat(&self.cmds))
    }
}

#[derive(Debug)]
pub struct CompileError {
    cmds: Vec<Cmd>,
    index: usize,
    stack_size: isize,
}

impl<'a> std::fmt::Display for CompileError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            fmt,
            "Attempt to pop beyond stack size. instruction: {} index: {}, size of stack {}",
            self.cmds[self.index],
            self.index,
            self.stack_size
        )
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Val {
    F(f64),
    I(i64),
}

impl From<bool> for Val {
    fn from(b: bool) -> Val {
        if b { Val::I(1) } else { Val::I(0) }
    }
}

impl From<i64> for Val {
    fn from(i: i64) -> Val {
        Val::I(i)
    }
}

impl From<f64> for Val {
    fn from(f: f64) -> Val {
        Val::F(f)
    }
}

impl Into<bool> for Val {
    fn into(self) -> bool {
        match self {
            Val::F(x) if x == 0.0 => false,
            Val::I(0) => false,
            _ => true,
        }
    }
}

impl Into<i64> for Val {
    fn into(self) -> i64 {
        match self {
            Val::F(x) => x as i64,
            Val::I(x) => x,
        }
    }
}

impl Into<f64> for Val {
    fn into(self) -> f64 {
        match self {
            Val::F(x) => x,
            Val::I(x) => x as f64,
        }
    }
}

impl Into<u8> for Val {
    fn into(self) -> u8 {
        let x: i64 = self.into();
        x as u8
    }
}

// @Todo: How should this ordering work?? Should we compare intervals?
impl PartialOrd for Val {
    fn partial_cmp(&self, rhs: &Val) -> Option<std::cmp::Ordering> {
        match (*self, *rhs) {
            (Val::I(l), Val::I(r)) => l.partial_cmp(&r),
            (Val::F(l), Val::F(r)) => l.partial_cmp(&r),
            (Val::I(l), Val::F(r)) => (l as f64).partial_cmp(&r),
            (Val::F(l), Val::I(r)) => l.partial_cmp(&(r as f64)),
        }
    }
}

// @Todo: How should this work? Should we do some smarter interval comparison?
// Is that equivalent to this?
impl PartialEq for Val {
    fn eq(&self, rhs: &Val) -> bool {
        match (*self, *rhs) {
            (Val::I(l), Val::I(r)) => l == r,
            (Val::F(l), Val::F(r)) => l == r,
            (Val::I(l), Val::F(r)) => l as f64 == r && l == r as i64,
            (Val::F(l), Val::I(r)) => l == r as f64 && l as i64 == r,
        }
    }
}

/// This allows you to write each expression in terms of consuming the top of the
/// stack, and then generating the new value to be pushed on.
///
/// # Example:
/// ```rust
/// stack_op!(stack { a: Val, b: Val, c: bool } => if c { a } else { b })
/// ```
/// will pop the top three elements off the stack (with `c` being the topmost),
/// and then push on either `a` or `b`.
macro_rules! stack_op {
    ($stack:ident { $($var:ident : $t:ty),* } => $res:expr) => {{
        // Pop the variables
        stack_op!($stack { $($var : $t),* });
        // Evaluate the expression and push it onto the stack
        $stack.push($res.into());
    }};
    // Pop the variables in reverse order
    ($stack:ident { }) => {};
    ($stack:ident { $var:ident : $t:ty $(, $rvar:ident : $rt:ty)* }) => {
        stack_op!($stack { $($rvar : $rt),* });
        let $var: $t = $stack.pop().unwrap().into();
    }
}

pub fn eval_beat<T: Into<Val>>(program: &Program, t: T) -> Val {
    use Cmd::*;
    let t = t.into();
    let mut stack: Vec<Val> = Vec::new();
    for cmd in &program.cmds {
        match *cmd {
            Var => stack_op!(stack { } => t),
            NumF(y) => stack_op!( stack { } => y),
            NumI(y) => stack_op!( stack { } => y),
            Add => stack_op!(stack { a: i64, b: i64 } => a.wrapping_add(b)),
            Sub => stack_op!(stack { a: i64, b: i64 } => a.wrapping_sub(b)),
            Mul => stack_op!(stack { a: i64, b: i64 } => a.wrapping_mul(b)),
            Div => {
                stack_op!(stack { a: i64, b: i64 } => {
                    if b == 0 { 0 } else { a.wrapping_div(b) }
                })
            }
            Mod => {
                stack_op!(stack { a: i64, b: i64 } => {
                    if b == 0 { 0 } else { a.wrapping_rem(b) }
                })
            }
            Shl => stack_op!(stack { a: i64, b: i64 } => a << (((b % 64) + 64) % 64)),
            Shr => {
                stack_op!(stack { a: i64, b: i64 } => {
                    let mut b = b % 64;
                    if b < 0 {
                        b += 64;
                    }
                    a >> b
                })
            }
            And => stack_op!(stack { a: i64, b: i64 } => a & b),
            Orr => stack_op!(stack { a: i64, b: i64 } => a | b),
            Xor => stack_op!(stack { a: i64, b: i64 } => a ^ b),
            Sin => stack_op!(stack { a: f64 } => a.sin()),
            Cos => stack_op!(stack { a: f64 } => a.cos()),
            Tan => stack_op!(stack { a: f64 } => a.tan()),
            Pow => stack_op!(stack { a: f64, b: f64 } => a.powf(b)),
            AddF => stack_op!(stack { a: f64, b: f64 } => a + b),
            SubF => stack_op!(stack { a: f64, b: f64 } => a - b),
            MulF => stack_op!(stack { a: f64, b: f64 } => a * b),
            DivF => {
                stack_op!(stack { a: f64, b: f64 } => {
                    if b == 0.0 { 0.0 } else { a / b }
                })
            }
            ModF => {
                stack_op!(stack { a: f64, b: f64 } => {
                    if b == 0.0 { 0.0 } else { a % b }
                })
            }
            Lt => stack_op!(stack { a: Val, b: Val } => a < b),
            Gt => stack_op!(stack { a: Val, b: Val } => a > b),
            Leq => stack_op!(stack { a: Val, b: Val } => a <= b),
            Geq => stack_op!(stack { a: Val, b: Val } => a >= b),
            Eq => stack_op!(stack { a: Val, b: Val } => a == b),
            Neq => stack_op!(stack { a: Val, b: Val } => a != b),
            Cond => {
                stack_op!(stack { a: Val, b: Val, cond: bool } => {
                    if cond { a } else { b }
                })
            }
            Arr(0) => stack.push(0.into()),
            Arr(size) => {
                let index: i64 = stack.pop().unwrap().into();
                // We want to split off from the end, so we must subtract here.
                let split_index = stack.len() - size;
                let vec = stack.split_off(split_index);
                let size = size as i64;
                // Calculate the positive modulus (% gives remainder, which
                // is slightly different than mod for negative values)
                let index = ((index % size) + size) % size;
                stack.push(vec[index as usize]);
            }
            // These have no runtime effect
            Fg(..) | Bg(..) | Khz(..) | Comment(..) => (),
        }
    }
    stack.pop().unwrap()
}

pub fn parse_beat(text: &str) -> Result<Vec<Cmd>, ParseError> {
    use Cmd::*;
    use ParseError::*;
    text.split_whitespace()
        .enumerate()
        .map(|(i, x)| match x {
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
            x if x.starts_with('[') => x[1..].parse().map(Arr).map_err(|_| BadArr(x, i)),
            x if x.starts_with("!fg:") => {
                let raw = u16::from_str_radix(&x[4..], 16).map_err(|_| BadFG(x, i))?;
                let r = (raw >> 8 & 0xF) as u8;
                let g = (raw >> 4 & 0xF) as u8;
                let b = (raw & 0xF) as u8;
                Ok(Fg(Color([r << 4 | r, g << 4 | g, b << 4 | b])))
            }
            x if x.starts_with("!bg:") => {
                let raw = u16::from_str_radix(&x[4..], 16).map_err(|_| BadBG(x, i))?;
                let r = (raw >> 8 & 0xF) as u8;
                let g = (raw >> 4 & 0xF) as u8;
                let b = (raw & 0xF) as u8;
                Ok(Bg(Color([r << 4 | r, g << 4 | g, b << 4 | b])))
            }
            x if x.starts_with("!khz:") => x[5..].parse().map(Khz).map_err(|_| BadKhz(x, i)),
            x if x.starts_with('#') => Ok(Comment(x[1..].into())),
            x => {
                if x.contains('.') {
                    x.parse().map(NumF).map_err(|_| UnknownToken(x, i))
                } else {
                    x.parse().map(NumI).map_err(|_| UnknownToken(x, i))
                }
            }
        })
        .collect()
}

#[derive(Debug, PartialEq)]
pub enum ParseError<'a> {
    BadArr(&'a str, usize),
    BadFG(&'a str, usize),
    BadBG(&'a str, usize),
    BadKhz(&'a str, usize),
    UnknownToken(&'a str, usize),
}

impl<'a> std::fmt::Display for ParseError<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use ParseError::*;
        match *self {
            BadArr(token, index) => write!(fmt, "Bad Array op: {}, index: {}", token, index),
            BadFG(token, index) => write!(fmt, "Bad Foreground Color: {}, index: {}", token, index),
            BadBG(token, index) => write!(fmt, "Bad Background Color: {}, index: {}", token, index),
            BadKhz(token, index) => write!(fmt, "Bad Sample Rate: {}, index: {}", token, index),
            UnknownToken(token, index) => write!(fmt, "Unknown Token: {}, index: {}", token, index),
        }
    }
}

impl std::fmt::Display for Cmd {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        use Cmd::*;
        match *self {
            Var => write!(fmt, "t"),
            NumF(y) => {
                let buf = format!("{}", y);
                if buf.contains('.') {
                    write!(fmt, "{}", buf)
                } else {
                    write!(fmt, "{}.0", buf)
                }
            }
            NumI(y) => write!(fmt, "{}", y),
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
            Comment(ref text) => write!(fmt, "#{}", text),
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

    macro_rules! test_invalid {
        (
            name: $name:ident,
            code: [$($cmd:expr),* $(,)*],
            index: $index:expr,
        ) => {
            mod $name {
                use super::*;
                #[test]
                fn test_err_compile() {
                    use Cmd::*;
                    let cmd = vec![$($cmd),*];
                    let result = compile(cmd);
                    assert!(result.is_err());
                }

                #[test]
                fn test_err_index() {
                    use Cmd::*;
                    let cmd = vec![$($cmd),*];
                    let result = compile(cmd).err().unwrap();
                    assert_eq!(result.index, $index);
                }
            }
        }
    }

    test_invalid! {
        name: add_empty,
        code: [Add],
        index: 0,
    }

    test_invalid! {
        name: add_small_stack,
        code: [Var, Add],
        index: 1,
    }

    test_invalid! {
        name: cond_small_stack,
        code: [Var, Var, Cond],
        index: 2,
    }

    test_invalid! {
        name: empty_arr_is_err,
        code: [Arr(0)],
        index: 0,
    }

    test_invalid! {
        name: arr_stack_too_small,
        code: [Var, Arr(1)],
        index: 1,
    }

    test_invalid! {
        name: arr_stack_too_small2,
        code: [NumI(1), NumI(2), NumI(3), Arr(3)],
        index: 3,
    }

    test_invalid! {
        name: sin_empty,
        code: [Sin],
        index: 0,
    }

    test_invalid! {
        name: should_not_dip_below_zero,
        code: [Var, Var, Var, Add, Add, Add, Var],
        index: 5,
    }

    #[test]
    fn test_comments_ok() {
        use Cmd::*;
        // These should always be valid even on an empty stack
        // !khz:8 !bg:000 !fg:000 #Hello
        let result = compile(vec![
            Khz(8),
            Bg(Color([0, 0, 0])),
            Fg(Color([0, 0, 0])),
            Comment("Hello".into()),
        ]);
        assert!(result.is_ok());
    }

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
                fn test_compile() {
                    use Cmd::*;
                    let cmd = vec![$($cmd),*];
                    let result = compile(cmd);
                    assert!(result.is_ok());
                }

                #[test]
                fn test_eval() {
                    use Cmd::*;
                    let cmd = vec![$($cmd),*];
                    let cmd = compile(cmd).unwrap();
                    $(
                        assert_eq!(
                            eval_beat(&cmd, $src),
                            $res.into(),
                            "t = {}, cmd: {}",
                            $src,
                            $text
                        );
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
        name: numi,
        text: "0 42",
        code: [NumI(0), NumI(42)],
        eval: {
            0.0 => 42.0,
            0.0 => 42.0,
        }
    }

    test_beat! {
        name: numf,
        text: "0.0 42.69",
        code: [NumF(0.0), NumF(42.69)],
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
        code: [Var, NumI(1), Sub],
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
        code: [Var, NumI(2), Div],
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
        code: [NumI(1), NumI(0), Div],
        eval: { 1.0 => 0.0 },
    }

    test_beat! {
        name: rem,
        text: "t 7 %",
        code: [Var, NumI(7), Mod],
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
        code: [NumI(7), NumI(0), Mod],
        eval: { 1.0 => 0.0 },
    }

    test_beat! {
        name: shl,
        text: "1.1 t <<",
        code: [NumF(1.1), Var, Shl],
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
        code: [NumF(1024.1), Var, Shr],
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
        code: [NumI(1), NumI(2), And],
        eval: { 1.0 => 0.0 },
    }

    test_beat! {
        name: orr,
        text: "1 2 |",
        code: [NumI(1), NumI(2), Orr],
        eval: { 1.0 => 3.0 },
    }

    test_beat! {
        name: xor,
        text: "t -1 ^",
        code: [Var, NumI(-1), Xor],
        eval: {
            0.0 => -1.0,
            3.2 => (!3i64) as f64,
        }
    }

    test_beat! {
        name: addf,
        text: "0.1 t +.",
        code: [NumF(0.1), Var, AddF],
        eval: {
            0.0 => 0.1,
            -2.0 => -1.9,
            1.0 => 1.1,
        }
    }

    test_beat! {
        name: subf,
        text: "0 t -.",
        code: [NumI(0), Var, SubF],
        eval: {
            -6.9 => 6.9,
            0.0 => 0.0,
            4.2 => -4.2,
        }
    }

    test_beat! {
        name: mulf,
        text: "7.5 t *.",
        code: [NumF(7.5), Var, MulF],
        eval: {
            4.2 => 31.5,
            0.0 => 0.0,
            -6.9 => -51.75,
        }
    }

    test_beat! {
        name: divf,
        text: "7.5 t /.",
        code: [NumF(7.5), Var, DivF],
        eval: {
            4.2 => 1.7857142857142856,
            0.0 => 0.0,
            -6.9 => -1.0869565217391304,
        }
    }

    test_beat! {
        name: modf,
        text: "7.5 t %.",
        code: [NumF(7.5), Var, ModF],
        eval: {
            4.2 => 3.3,
            0.0 => 0.0,
            -6.9 => 0.5999999999999996,
        }
    }

    test_beat! {
        name: lt,
        text: "t 64 <",
        code: [Var, NumI(64), Lt],
        eval: {
            0.0 => 1.0,
            64.0 => 0.0,
            128.0 => 0.0,
        }
    }

    test_beat! {
        name: gt,
        text: "t 64 >",
        code: [Var, NumI(64), Gt],
        eval: {
            0.0 => 0.0,
            64.0 => 0.0,
            128.0 => 1.0,
        }
    }

    test_beat! {
        name: leq,
        text: "t 64 <=",
        code: [Var, NumI(64), Leq],
        eval: {
            0.0 => 1.0,
            64.0 => 1.0,
            128.0 => 0.0,
        }
    }

    test_beat! {
        name: geq,
        text: "t 64 >=",
        code: [Var, NumI(64), Geq],
        eval: {
            0.0 => 0.0,
            64.0 => 1.0,
            128.0 => 1.0,
        }
    }

    test_beat! {
        name: eq,
        text: "t 64 ==",
        code: [Var, NumI(64), Eq],
        eval: {
            0.0 => 0.0,
            64.0 => 1.0,
            128.0 => 0.0,
        }
    }

    test_beat! {
        name: neq,
        text: "t 64 !=",
        code: [Var, NumI(64), Neq],
        eval: {
            0.0 => 1.0,
            64.0 => 0.0,
            128.0 => 1.0,
        }
    }

    test_beat! {
        name: cond,
        text: "3 2 t ?",
        code: [NumI(3), NumI(2), Var, Cond],
        eval: {
            0 => 2,
            1 => 3,
            2 => 3,
        }
    }

    test_beat! {
        name: color,
        text: "!fg:F00 !bg:00F 0",
        code: [Fg(Color([0xFF, 0x00, 0x00])), Bg(Color([0x00, 0x00, 0xFF])), NumI(0)],
        eval: { 0 => 0 },
    }

    test_beat! {
        name: khz,
        text: "!khz:8 8000",
        code: [Khz(8), NumI(8000)],
        eval: { 0 => 8000 },
    }

    test_beat! {
        name: comment,
        text: "#bbcurated 42",
        code: [Comment("bbcurated".into()), NumI(42)],
        eval: { 0 => 42 },
    }

    #[test]
    fn test_metadata() {
        use Cmd::*;
        let code = vec![
            Fg(Color([0, 0, 0])),
            Bg(Color([0, 0, 0])),
            Khz(8),
            Khz(11),
            Fg(Color([1, 0, 0])),
            Bg(Color([0, 1, 1])),
        ];
        let prog = compile(code).unwrap();
        assert_eq!(prog.hz(), Some(11_000));
        assert_eq!(prog.fg(), Some(Color([1, 0, 0])));
        assert_eq!(prog.bg(), Some(Color([0, 1, 1])));
    }

    test_beat! {
        name: even,
        text: "3 2 t 2 % 0 == ?",
        code: [NumI(3), NumI(2), Var, NumI(2), Mod, NumI(0), Eq, Cond],
        eval: {
            0 => 3,
            1 => 2,
            2 => 3,
            3 => 2,
            4 => 3,
            5 => 2,
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
        code: [Var, NumF(3.5), Pow],
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
        code: [NumI(1), NumI(2), NumI(3), Var, Arr(3)],
        eval: {
            -4 => 3,
            -3 => 1,
            -2 => 2,
            -1 => 3,
            0 => 1,
            1 => 2,
            2 => 3,
            3 => 1,
            4 => 2,
            5 => 3,
            6 => 1,
        }
    }

    test_beat! {
        name: arr_pops,
        text: "10 1 2 3 t [3 +",
        code: [NumI(10), NumI(1), NumI(2), NumI(3), Var, Arr(3), Add],
        eval: {
            0 => 11,
            1 => 12,
            2 => 13,
        }
    }

    test_beat! {
        name: arr_pushes_zero,
        text: "1 2 3 [0",
        code: [NumI(1), NumI(2), NumI(3), Arr(0)],
        eval: {
            0 => 0,
            1 => 0,
            2 => 0,
            3 => 0,
            4 => 0,
        }
    }

    test_beat! {
        name: example1,
        text: "t 1 >> t | tan 128 +",
        code: [Var, NumI(1), Shr, Var, Orr, Tan, NumI(128), Add],
        eval: { 1.0 => 129.0 },
    }

    test_beat! {
        name: example2,
        text: "t 1 >> t | tan 128 +.",
        code: [Var, NumI(1), Shr, Var, Orr, Tan, NumI(128), AddF],
        eval: { 1.0 => 129.5574077246549 },
    }

    test_beat! {
        name: example3,
        text: "2.5 1.2 %.",
        code: [NumF(2.5), NumF(1.2), ModF],
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
            NumI(10),
            Div,
            Var,
            NumI(2),
            Var,
            NumI(10),
            Shr,
            Pow,
            Mul,
            Sin,
            Add,
            Sin,
            NumI(64),
            Mul,
            NumI(128),
            Add,
        ],
        eval: { 3.0 => 128.0 },
    }

    test_beat! {
        name: example6,
        text: "t 10.0 /. t 2.0 t 10.0 >> pow *. sin +. sin 64.0 *. 128.0 +.",
        code: [
            Var,
            NumF(10.0),
            DivF,
            Var,
            NumF(2.0),
            Var,
            NumF(10.0),
            Shr,
            Pow,
            MulF,
            Sin,
            AddF,
            Sin,
            NumF(64.0),
            MulF,
            NumF(128.0),
            AddF,
        ],
        eval: { 3 => 155.324961718789 },
    }

    #[test]
    fn test_overflow() {
        // The typed evaluator ensures that we don't make as many unnecessary
        // conversions, and so we keep more precision.
        use Cmd::*;
        let code = vec![Var, Var, Mul];
        let code = compile(code).unwrap();
        let res: u8 = eval_beat(&code, 1_073_741_825.0).into();
        assert_eq!(res, (1_073_741_825i64.wrapping_mul(1_073_741_825)) as u8);
    }
}
