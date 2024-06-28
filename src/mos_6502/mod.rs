/// # 参考
/// ## 6502 指令集参考
/// - [https://www.nesdev.org/obelisk-6502-guide/reference.html](https://www.nesdev.org/obelisk-6502-guide/reference.html)
/// - [http://www.6502.org/tutorials/6502opcodes.html](http://www.6502.org/tutorials/6502opcodes.html)
///
/// ## 寻址模式参考
/// - [https://skilldrick.github.io/easy6502/#addressing](https://skilldrick.github.io/easy6502/#addressing)
/// - [https://www.nesdev.org/obelisk-6502-guide/reference.html](https://www.nesdev.org/obelisk-6502-guide/reference.html)





/// CPU 结构体
pub struct CPU {
    // 寄存器A
    pub register_a: u8,
    // 寄存器X
    pub register_x: u8,
    // 状态寄存器
    pub status: u8,
    // 程序计数器
    pub program_counter: u16,
    // pub memory: [u8; 0xFFFF],
}

impl CPU {
    /// Construct a new CPU
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
        }
    }

    /// LDA - Loads a byte of memory into the accumulator setting the zero and negative flags as appropriate.
    fn lda(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    /// TAX - Copies the current contents of the accumulator into the X register and sets the zero and negative flags as appropriate.
    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// INX - Adds one to the X register setting the zero and negative flags as appropriate.
    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

    /// Updates the zero and negative flags based on the given register value.
    fn update_zero_and_negative_flags(&mut self, register: u8) {
        if register == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        if register & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    /// 解释程序
    pub fn interpret(&mut self, program: Vec<u8>) {
        self.program_counter = 0;

        loop {
            let opcode = program[self.program_counter as usize];
            self.program_counter += 1;

            match opcode {
                // LDA
                0xA9 => {
                    let param = program[self.program_counter as usize];
                    self.program_counter += 1;

                    self.lda(param);
                },
                // TAX
                0xAA => self.tax(),
                // INX
                0xE8 => self.inx(),
                // BRK - The BRK instruction forces the generation of an interrupt request. The program counter and processor status are pushed on the stack then the IRQ interrupt vector at $FFFE/F is loaded into the PC and the break flag in the status set to one.
                0x00 => return,
                _ => {}
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert_eq!(cpu.status & 0b0000_0010, 0b00);
        assert_eq!(cpu.status & 0b1000_0000, 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0x00, 0x00]);
        assert_eq!(cpu.status & 0b0000_0010, 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.register_a = 10;
        cpu.interpret(vec![0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10);
        assert_eq!(cpu.status, 0);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.interpret(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xff;
        cpu.interpret(vec![0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1);
        assert_eq!(cpu.status, 0b0000_0000);
    }
}