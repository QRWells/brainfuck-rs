use std::{
    collections::HashMap,
    io::{Read, Write},
    path::Path,
};

use anyhow::Ok;

use crate::{
    error::RuntimeError,
    ir::{self, compile, BrainfuckIR},
};

const MEMORY_SIZE: usize = 4 * 1024 * 1024; // 4 MiB

pub struct VM<'io> {
    code: Vec<BrainfuckIR>,
    memory: Box<[u8]>,
    input: Box<dyn Read + 'io>,
    output: Box<dyn Write + 'io>,
}

impl<'io> VM<'io> {
    pub fn new(
        file_path: &Path,
        input: Box<dyn Read + 'io>,
        output: Box<dyn Write + 'io>,
        optimize: bool,
    ) -> anyhow::Result<Self> {
        let src = std::fs::read_to_string(file_path)?;
        let mut code = compile(&src)?;

        if optimize {
            ir::optimize(&mut code);
        }

        let memory = vec![0; MEMORY_SIZE].into_boxed_slice();

        Ok(Self {
            code,
            memory,
            input,
            output,
        })
    }

    pub fn run(&mut self) -> anyhow::Result<()> {
        let map = self.mark_left_right();
        let mut ptr = 0;
        let mut pc = 0;
        loop {
            match self.code[pc] {
                BrainfuckIR::Add(val) => self.memory[ptr] += val,
                BrainfuckIR::Sub(val) => self.memory[ptr] -= val,
                BrainfuckIR::PtrAdd(val) => {
                    if ptr == self.memory.len() {
                        return Err(RuntimeError::Overflow.into());
                    }
                    ptr += val;
                }
                BrainfuckIR::PtrSub(val) => {
                    if ptr == 0 {
                        return Err(RuntimeError::Overflow.into());
                    }
                    ptr -= val;
                }
                BrainfuckIR::Write => {
                    self.output.write_all(&self.memory[ptr..=ptr])?;
                }
                BrainfuckIR::Read => {
                    let mut byte: [u8; 1] = [0; 1];
                    self.input.read_exact(&mut byte)?;
                }
                BrainfuckIR::Jz => {
                    if self.memory[ptr] == 0 {
                        pc = map[&pc];
                        continue;
                    }
                }
                BrainfuckIR::Jnz => {
                    if self.memory[ptr] != 0 {
                        pc = map[&pc];
                        continue;
                    }
                }
            }

            if pc == self.code.len() {
                break;
            }

            pc += 1;
        }
        Ok(())
    }

    fn mark_left_right(&self) -> HashMap<usize, usize> {
        let mut i = 0;
        let mut stack = vec![];
        let mut map = HashMap::new();
        while i < self.code.len() {
            match self.code[i] {
                BrainfuckIR::Jz => {
                    stack.push(i);
                }
                BrainfuckIR::Jnz => {
                    if let Some(left) = stack.pop() {
                        map.insert(left, i);
                        map.insert(i, left);
                    } else {
                        unreachable!()
                    }
                }
                _ => {}
            }
            i += 1;
        }
        map
    }
}
