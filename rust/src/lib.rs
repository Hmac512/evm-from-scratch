mod opcodes;
mod stack;

use primitive_types::U256;
use crate::stack::Stack;
use crate::opcodes::Opcode;


pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

fn concat_decimals(arr: &[u8]) -> U256 {
    let hexadecimal_concat: String = arr
        .iter()
        .map(|x| format!("{:X}", x))
        .collect::<Vec<String>>()
        .join("");

    return U256::from_str_radix(&hexadecimal_concat, 16).unwrap();
}

struct Machine<'a> {
    stack: Stack,
    code: &'a [u8],
    pc: usize,
}

impl<'a> Machine<'a> {
    fn new(code: &'a [u8]) -> Self {
        Self {
            stack: Stack::new(),
            code,
            pc: 0,
        }
    }

    fn stack(&self) -> Vec<U256> {
        self.stack.data()
    }

    fn opcode(&self) -> u8 {
        self.code[self.pc]
    }

    fn step(&mut self, steps: usize) {
        self.pc += steps;
    }

    fn push_on_to_stack(&mut self) {
        let n = usize::from(self.opcode() - 0x5F);
        let start = self.pc + 1;
        let end = start + n;
        let bytes = &self.code[start..end];
        let val_to_push = concat_decimals(bytes);
        self.stack.push(val_to_push);
        self.step(n);
    }

    fn pop_from_stack(&mut self) {
        self.stack.pop();
    }

    fn add(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let res = a.overflowing_add(b).0;
        self.stack.push(res);
    }

    fn mul(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let res = a.overflowing_mul(b).0;
        self.stack.push(res);
    }

    fn sub(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let res = a.overflowing_sub(b).0;
        self.stack.push(res);
    }

    fn div(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let res = a.checked_div(b);
        match res {
            Some(result) => self.stack.push(result),
            None => self.stack.push(U256::from(0)),
        }
    }

    fn modulus(&mut self) {
        let a = self.stack.pop().unwrap();
        let b = self.stack.pop().unwrap();
        let res = a.checked_rem(b);
        match res {
            Some(result) => self.stack.push(result),
            None => self.stack.push(U256::from(0)),
        }
    }

    fn execute(&mut self) -> EvmResult {
        while self.pc < self.code.len() {
            match self.opcode() {
                Opcode::STOP => break,
                Opcode::ADD => self.add(),
                Opcode::MUL => self.mul(),
                Opcode::SUB => self.sub(),
                Opcode::DIV => self.div(),
                Opcode::MOD => self.modulus(),
                Opcode::POP => self.pop_from_stack(),
                Opcode::PUSH1..=Opcode::PUSH32 => self.push_on_to_stack(),
                _ => {}
            }

            self.step(1);
        }

        return EvmResult {
            stack: self.stack(),
            success: true,
        };
    }
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let mut machine: Machine = Machine::new(_code.as_ref());
    return machine.execute();
}
