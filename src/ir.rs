use std::ops::Add;

use crate::error::{CompileError, CompileErrorKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrainfuckIR {
    Add(usize),    // +
    Sub(usize),    // -
    PtrAdd(usize), // >
    PtrSub(usize), // <
    Write,         // .
    Read,          // ,
    Jz,            // [
    Jnz,           // ]
}

/// Compile source code into IR
pub fn compile(source: &str) -> Result<Vec<BrainfuckIR>, CompileError> {
    let mut code = vec![];
    let mut stack = vec![];
    let mut line = 1;
    let mut col = 1;

    for ch in source.chars() {
        col += 1;
        match ch {
            '\n' => {
                line += 1;
                col = 1;
            }
            '+' => code.push(BrainfuckIR::Add(1)),
            '-' => code.push(BrainfuckIR::Sub(1)),
            '>' => code.push(BrainfuckIR::PtrAdd(1)),
            '<' => code.push(BrainfuckIR::PtrSub(1)),
            ',' => code.push(BrainfuckIR::Read),
            '.' => code.push(BrainfuckIR::Write),
            '[' => {
                let pos = code.len();
                stack.push((pos, line, col));
                code.push(BrainfuckIR::Jz)
            }
            ']' => {
                stack.pop().ok_or(CompileError {
                    line,
                    col,
                    kind: CompileErrorKind::UnexpectedRightBracket,
                })?;

                code.push(BrainfuckIR::Jnz)
            }
            _ => {
                return Err(CompileError {
                    line,
                    col,
                    kind: CompileErrorKind::UnclosedCharacter,
                });
            }
        }
    }

    if let Some((_, line, col)) = stack.pop() {
        Err(CompileError {
            line,
            col,
            kind: CompileErrorKind::UnclosedLeftBracket,
        })
    } else {
        Ok(code)
    }
}

pub fn optimize(code: Vec<BrainfuckIR>) -> Vec<BrainfuckIR> {
    let mut opt = vec![];
    let mut i = 0;

    macro_rules! compact {
        ($ir:ident, $val:ident) => {{
            let mut j = i + 1;
            while j < code.len() {
                if let $ir(val_new) = code[j] {
                    $val = $val.wrapping_add(val_new);
                    j += 1;
                } else {
                    break;
                }
            }
            i = j;
            opt.push($ir($val));
        }};
    }

    while i < code.len() {
        use BrainfuckIR::*;
        match code[i] {
            Add(mut val) => compact!(Add, val),
            Sub(mut val) => compact!(Sub, val),
            PtrAdd(mut val) => compact!(PtrAdd, val),
            PtrSub(mut val) => compact!(PtrSub, val),
            Write | Read | Jz | Jnz => {
                opt.push(code[i]);
                i += 1;
            }
        }
    }
    opt
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compile_test() {
        let code = compile("+[,.]");
        assert_eq!(
            code.unwrap(),
            vec![
                BrainfuckIR::Add(1),
                BrainfuckIR::Jz,
                BrainfuckIR::Read,
                BrainfuckIR::Write,
                BrainfuckIR::Jnz,
            ]
        );

        let code = compile("[[]");
        assert_eq!(
            code.unwrap_err().kind,
            CompileErrorKind::UnclosedLeftBracket
        );

        let code = compile("[]]");
        assert_eq!(
            code.unwrap_err().kind,
            CompileErrorKind::UnexpectedRightBracket
        );
    }

    #[test]
    fn optimize_test() {
        let code = compile("++++++++++----->><<");
        assert!(code.is_ok());
        let opt = optimize(code.unwrap());
        assert_eq!(
            opt,
            vec![
                BrainfuckIR::Add(10),
                BrainfuckIR::Sub(5),
                BrainfuckIR::PtrAdd(2),
                BrainfuckIR::PtrSub(2)
            ]
        );

        let code = vec![
            BrainfuckIR::Add(2),
            BrainfuckIR::Add(3),
            BrainfuckIR::Add(4),
        ];
        let opt = optimize(code);
        assert_eq!(opt, vec![BrainfuckIR::Add(9)]);
    }
}
