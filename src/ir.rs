use crate::error::{CompileError, CompileErrorKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrainfuckIR {
    Add,    // +
    Sub,    // -
    PtrAdd, // >
    PtrSub, // <
    Write,  // .
    Read,   // ,
    Jz,     // [
    Jnz,    // ]
}

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
            '+' => code.push(BrainfuckIR::Add),
            '-' => code.push(BrainfuckIR::Sub),
            '>' => code.push(BrainfuckIR::PtrAdd),
            '<' => code.push(BrainfuckIR::PtrSub),
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

#[test]
fn compile_test() {
    let code = compile("+[,.]");
    assert_eq!(
        code.unwrap(),
        vec![
            BrainfuckIR::Add,
            BrainfuckIR::Jz,
            BrainfuckIR::Read,
            BrainfuckIR::Write,
            BrainfuckIR::Jnz,
        ]
    );

    let code = compile("[[]");
    assert_eq!(code.unwrap_err().kind, CompileErrorKind::UnclosedLeftBracket);

    let code = compile("[]]");
    assert_eq!(code.unwrap_err().kind, CompileErrorKind::UnexpectedRightBracket);
}
