use super::*;

macro_rules! test_invalid {
    (
        name: $name:ident,
        code: [$($cmd:expr),* $(,)*],
        err_kind: $err_kind:expr,
    ) => {
        mod $name {
            use super::*;
            #[test]
            fn test_err_compile() {
                // TODO: Find a better way to make the "empty" test
                // not complain about unused imports?
                #[allow(unused_imports)]
                use Cmd::*;
                let cmd = vec![$($cmd),*];
                let result = compile(cmd);
                assert!(result.is_err());
            }

            #[test]
            fn test_err_kind() {
                #[allow(unused_imports)]
                use Cmd::*;
                use ErrorKind::*;
                let cmd = vec![$($cmd),*];
                let result = compile(cmd).err().unwrap();
                assert_eq!(result.error_kind, $err_kind);
            }
        }
    }
}

test_invalid! {
    name: empty,
    code: [],
    err_kind: EmptyProgram,
}

test_invalid! {
    name: add_empty,
    code: [Add],
    err_kind: UnderflowedStack { index: 0, stack_size: 0},
}

test_invalid! {
    name: add_small_stack,
    code: [Var, Add],
    err_kind: UnderflowedStack { index: 1, stack_size: 1},
}

test_invalid! {
    name: cond_small_stack,
    code: [Var, Var, Cond],
    err_kind: UnderflowedStack { index: 2, stack_size: 2},
}

test_invalid! {
    name: empty_arr_is_err,
    code: [Arr(0)],
    err_kind: UnderflowedStack { index: 0, stack_size: 0},
}

test_invalid! {
    name: arr_stack_too_small,
    code: [Var, Arr(1)],
    err_kind: UnderflowedStack { index: 1, stack_size: 1},
}

test_invalid! {
    name: arr_stack_too_small2,
    code: [NumI(1), NumI(2), NumI(3), Arr(3)],
    err_kind: UnderflowedStack { index: 3, stack_size: 3},
}

test_invalid! {
    name: sin_empty,
    code: [Sin],
    err_kind: UnderflowedStack { index: 0, stack_size: 0},
}

test_invalid! {
    name: should_not_dip_below_zero,
    code: [Var, Var, Var, Add, Add, Add, Var],
    err_kind: UnderflowedStack { index: 5, stack_size: 1},
}

test_invalid! {
    name: need_at_one_value_on_stack,
    code: [Khz(8), Bg(Color([0, 0, 0])), Fg(Color([0, 0, 0])), Comment("Hello".into())],
    err_kind: EmptyProgram,
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
        NumI(0), // Required, empty programs are invalid.
    ];
    let prog = compile(code).unwrap();
    assert_eq!(prog.hz(), Some(11_000));
    assert_eq!(prog.fg(), Some(Color([1, 0, 0])));
    assert_eq!(prog.bg(), Some(Color([0, 1, 1])));
}

test_beat! {
    name: hex,
    text: "0x0 0x1 0x2 0xA 0xF 0x10 0xA0 0xFF 0xFFFF 0xBAD1DEA",
    code: [Hex(0x0), Hex(0x1), Hex(0x2), Hex(0xA), Hex(0xF), Hex(0x10),
           Hex(0xA0), Hex(0xFF), Hex(0xFFFF), Hex(0xBAD1DEA)],
    eval: {
        0 => 195894762,
    }
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
