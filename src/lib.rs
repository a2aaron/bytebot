#[derive(Debug, PartialEq)]
pub enum Cmd {
    Var,
    Num(f32),
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
}

pub fn eval_beat(cmds: &[Cmd], t: f32) -> Result<f32, ()> {
    let mut stack = Vec::new();
    for cmd in cmds {
        match *cmd {
            Cmd::Var => stack.push(t),
            Cmd::Num(y) => stack.push(y),
            Cmd::Add => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push(a + b);
            }
            Cmd::Sub => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push(a - b);
            }
            Cmd::Mul => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                stack.push(a * b);
            }
            Cmd::Div => {
                let b = stack.pop().ok_or(())?;
                let a = stack.pop().ok_or(())?;
                if b == 0.0 {
                    stack.push(0.0);
                } else {
                    stack.push(a / b);
                }
            }
            Cmd::Mod => {
                let b = stack.pop().ok_or(())? as i32;
                let a = stack.pop().ok_or(())? as i32;
                if b == 0 {
                    stack.push(0.0);
                } else {
                    stack.push(a.wrapping_rem(b) as f32);
                }
            }
            Cmd::Shl => {
                let b = stack.pop().ok_or(())? as i32;
                let a = stack.pop().ok_or(())? as i32;
                stack.push((a << (b % 32)) as f32);
            }
            Cmd::Shr => {
                let b = stack.pop().ok_or(())? as i32;
                let a = stack.pop().ok_or(())? as i32;
                stack.push((a >> (b % 32)) as f32);
            }
            Cmd::And => {
                let b = stack.pop().ok_or(())? as i32;
                let a = stack.pop().ok_or(())? as i32;
                stack.push((a & b) as f32);
            }
            Cmd::Orr => {
                let b = stack.pop().ok_or(())? as i32;
                let a = stack.pop().ok_or(())? as i32;
                stack.push((a | b) as f32);
            }
            Cmd::Xor => {
                let b = stack.pop().ok_or(())? as i32;
                let a = stack.pop().ok_or(())? as i32;
                stack.push((a ^ b) as f32);
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
        assert_eq!(
            eval_beat(&[Num(8.0), Var, Div, Num(2.0), Mod], 3.0),
            Ok(0.0)
        );
        assert_eq!(eval_beat(&[Var, Num(-1.0), Xor], 7.0), Ok((!7i32) as f32));
        assert_eq!(
            eval_beat(&[Var, Num(1.0), Shr, Num(1.0), Shl], 3.0),
            Ok(2.0)
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
    }
}
