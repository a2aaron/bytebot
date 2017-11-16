extern crate rand;

pub mod random;


#[derive(Clone, Copy, Debug, PartialEq)]
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
}

pub fn eval_beat(cmds: &[Cmd], t: f64) -> Result<f64, ()> {
    use Cmd::*;
    let mut stack = Vec::new();
    for cmd in cmds {
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

    #[test]
    fn test_eval() {
        use Cmd::*;
        assert_eq!(eval_beat(&[Var, Var, Add], 2.0), Ok(4.0));
        assert_eq!(eval_beat(&[Var, Var, Mul], 3.0), Ok(9.0));
        assert_eq!(eval_beat(&[Var, Var, Num(2.0), Mul, Sub], 3.0), Ok(-3.0));
        assert_eq!(eval_beat(&[Num(1.0), Num(2.0), And], -1.0), Ok(0.0));
        assert_eq!(eval_beat(&[Num(1.0), Num(2.0), Orr], -1.0), Ok(3.0));
        assert_eq!(eval_beat(&[Num(2.0), Num(-1.0), Shl], 1.0), Ok(0.0));
        assert_eq!(
            eval_beat(&[Num(8.0), Var, Div, Num(2.0), Mod], 3.0),
            Ok(0.0)
        );
        assert_eq!(eval_beat(&[Var, Num(-1.0), Xor], 7.0), Ok((!7i32) as f64));
        assert_eq!(
            eval_beat(&[Var, Num(1.0), Shr, Num(1.0), Shl], 3.0),
            Ok(2.0)
        );
        assert_eq!(
            eval_beat(&[Var, Num(1.0), Shr, Var, Orr, Tan, Num(128.0), Add], 1.0),
            Ok(129.0)
        );
        assert_eq!(
            eval_beat(&[Var, Num(1.0), Shr, Var, Orr, Tan, Num(128.0), AddF], 1.0),
            Ok(129.5574077246549)
        );
        assert_eq!(
            eval_beat(
                &[Var, Cos, Var, Cos, MulF, Var, Sin, Var, Sin, MulF, AddF],
                0.4,
            ),
            Ok(1.0)
        );
        assert_eq!(
            eval_beat(
                &[
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
                3.0,
            ),
            Ok(128.0)
        );
        assert_eq!(
            eval_beat(
                &[
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
                3.0,
            ),
            Ok(155.324961718789)
        );
        assert_eq!(
            eval_beat(&[Num(2.5), Num(1.2), ModF], 0.0),
            Ok(0.10000000000000009)
        );
    }

    #[test]
    fn test_format() {
        use Cmd::*;
        assert_eq!(format_beat(&[Var, Var, Add]), "t t +");
        assert_eq!(format_beat(&[Var, Var, Mul]), "t t *");
        assert_eq!(format_beat(&[Var, Var, Num(2.0), Mul, Sub]), "t t 2 * -");
        assert_eq!(format_beat(&[Num(1.0), Num(2.0), And]), "1 2 &");
        assert_eq!(format_beat(&[Num(1.0), Num(2.0), Orr]), "1 2 |");
        assert_eq!(
            format_beat(&[Num(8.0), Var, Div, Num(2.0), Mod]),
            "8 t / 2 %"
        );
        assert_eq!(format_beat(&[Var, Num(-1.0), Xor]), "t -1 ^");
        assert_eq!(
            format_beat(&[Var, Num(1.0), Shr, Num(1.0), Shl]),
            "t 1 >> 1 <<"
        );
        assert_eq!(
            format_beat(&[Var, Num(1.0), Shr, Var, Orr, Tan, Num(128.0), Add]),
            "t 1 >> t | tan 128 +"
        );
        assert_eq!(
            format_beat(&[Var, Cos, Var, Cos, Mul, Var, Sin, Var, Sin, Mul, Add]),
            "t cos t cos * t sin t sin * +"
        );
        assert_eq!(
            format_beat(
                &[
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
            ),
            "t 10 / t 2 t 10 >> pow * sin + sin 64 * 128 +"
        );
        assert_eq!(format_beat(&[AddF, SubF, MulF, DivF]), "+. -. *. /.");
        assert_eq!(format_beat(&[Num(2.5), Num(1.2), ModF]), "2.5 1.2 %.");
    }

    #[test]
    fn test_parse() {
        use Cmd::*;
        assert_eq!(parse_beat("t t +"), Ok(vec![Var, Var, Add]));
        assert_eq!(parse_beat("t t *"), Ok(vec![Var, Var, Mul]));
        assert_eq!(
            parse_beat("t t 2.0 * -"),
            Ok(vec![Var, Var, Num(2.0), Mul, Sub])
        );
        assert_eq!(parse_beat("1 2 &"), Ok(vec![Num(1.0), Num(2.0), And]));
        assert_eq!(parse_beat("1 2.0 |"), Ok(vec![Num(1.0), Num(2.0), Orr]));
        assert_eq!(
            parse_beat("8.0 t / 2.0 %"),
            Ok(vec![Num(8.0), Var, Div, Num(2.0), Mod])
        );
        assert_eq!(parse_beat("t -1.0 ^"), Ok(vec![Var, Num(-1.0), Xor]));
        assert_eq!(
            parse_beat("t 1.0 >> 1.0 <<"),
            Ok(vec![Var, Num(1.0), Shr, Num(1.0), Shl])
        );
        assert_eq!(
            parse_beat("t 1 >> t | tan 128 +"),
            Ok(vec![Var, Num(1.0), Shr, Var, Orr, Tan, Num(128.0), Add])
        );
        assert_eq!(
            parse_beat("t cos t cos * t sin t sin * +"),
            Ok(vec![Var, Cos, Var, Cos, Mul, Var, Sin, Var, Sin, Mul, Add])
        );
        assert_eq!(
            parse_beat("t 10 / t 2 t 10 >> pow * sin + sin 64 * 128 +"),
            Ok(vec![
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
            ])
        );
        assert_eq!(parse_beat("+. -. *. /."), Ok(vec![AddF, SubF, MulF, DivF]));
        assert_eq!(parse_beat("2.5 1.2 %."), Ok(vec![Num(2.5), Num(1.2), ModF]));
    }
}
