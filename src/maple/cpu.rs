use crate::maple::memory::Memory;
use crate::maple::instructions;

pub enum ExecutionMode {
    User,
    Kernel,
}

pub enum ExecutionResult {
    Ok,
    Exit
}

pub trait CPU {

    fn process(&mut self, memory: &mut Memory) -> ExecutionResult;
    fn fetch(&mut self, memory: &mut Memory) -> u64;
    fn decode_and_execute(&mut self, memory: &mut Memory, instruction: u64) -> ExecutionResult;

    fn raise_interrupt(&mut self, code: u32);

    fn get_stack_pointer(&self) -> u64;
    fn set_stack_pointer(&mut self, value: u64);

    fn get_program_counter(&self) -> u64;
    fn set_program_counter(&mut self, value: u64);

    fn get_dynamic_link(&self) -> u64;
    fn set_dynamic_link(&mut self, value: u64);

    fn get_result(&self) -> u64;
    fn set_result(&mut self, value: u64);

    fn get_io_pointer(&self) -> u64;
    fn set_io_pointer(&mut self, value: u64);

    fn get_page_table_base(&self) -> u64;
    fn set_page_table_base(&mut self, value: u64);

    fn get_system_info(&self) -> u64;
    fn set_system_info(&mut self, value: u64);

    fn get_frame_pointer(&self) -> u64;
    fn set_frame_pointer(&mut self, value: u64);

    fn get_gp_register(&self, gp: usize) -> u64;
    fn set_gp_register(&mut self, gp: usize, value: u64);

    fn get_hw_register(&self, hw: usize) -> u64;
    fn set_hw_register(&mut self, hw: usize, value: u64);

    fn get_register(&self, register: usize) -> u64;
    fn set_register(&mut self, register: usize, value: u64);
}

const REGISTER_STACK_POINTER: usize = 6;
const REGISTER_PROGRAM_COUNTER: usize = 7;
const REGISTER_DYNAMIC_LINK: usize = 8;
const REGISTER_RESULT: usize = 9;
const REGISTER_IO_POINTER: usize = 10;
const REGISTER_PAGE_TABLE_BASE: usize = 11;
const REGISTER_SYSTEM_INFO: usize = 12;
const REGISTER_FRAME_POINTER: usize = 13;

pub struct MapleCPU {
    pub mode: ExecutionMode,
    pub registers: [u64; 16],
}

impl MapleCPU {
    pub fn new() -> Self {
        MapleCPU {
            registers: [0; 16],
            mode: ExecutionMode::Kernel,
        }
    }
}

impl CPU for MapleCPU {
    fn process(&mut self, memory: &mut Memory) -> ExecutionResult {
        let instruction = self.fetch(memory);
        self.decode_and_execute(memory, instruction)
    }

    fn fetch(&mut self, memory: &mut Memory) -> u64 {
        let address = self.get_program_counter() as u32;
        memory.read(address, self)
    }

    fn decode_and_execute(&mut self, memory: &mut Memory, instruction: u64) -> ExecutionResult {
        todo!()
    }

    fn raise_interrupt(&mut self, code: u32) {
         // First 16 bits represent the address of the interrupt table
        let interrupt_table_base = self.get_system_info() >> (64 - 16);

        let result = interrupt_table_base + (code as u64);

        self.set_program_counter(result);
    }

    fn get_stack_pointer(&self) -> u64 {
        self.registers[REGISTER_STACK_POINTER]
    }

    fn set_stack_pointer(&mut self, value: u64) {
        self.registers[REGISTER_STACK_POINTER] = value;
    }

    fn get_program_counter(&self) -> u64 {
        self.registers[REGISTER_PROGRAM_COUNTER]
    }

    fn set_program_counter(&mut self, value: u64) {
        self.registers[REGISTER_PROGRAM_COUNTER] = value;
    }

    fn get_dynamic_link(&self) -> u64 {
        self.registers[REGISTER_DYNAMIC_LINK]
    }

    fn set_dynamic_link(&mut self, value: u64) {
        self.registers[REGISTER_DYNAMIC_LINK] = value;
    }

    fn get_result(&self) -> u64 {
        self.registers[REGISTER_RESULT]
    }

    fn set_result(&mut self, value: u64) {
        self.registers[REGISTER_RESULT] = value;
    }

    fn get_io_pointer(&self) -> u64 {
        self.registers[REGISTER_IO_POINTER]
    }

    fn set_io_pointer(&mut self, value: u64) {
        self.registers[REGISTER_IO_POINTER] = value;
    }

    fn get_page_table_base(&self) -> u64 {
        self.registers[REGISTER_PAGE_TABLE_BASE]
    }

    fn set_page_table_base(&mut self, value: u64) {
        self.registers[REGISTER_PAGE_TABLE_BASE] = value;
    }

    fn get_system_info(&self) -> u64 {
        self.registers[REGISTER_SYSTEM_INFO]
    }

    fn set_system_info(&mut self, value: u64) {
        self.registers[REGISTER_SYSTEM_INFO] = value;
    }

    fn get_frame_pointer(&self) -> u64 {
        self.registers[REGISTER_FRAME_POINTER]
    }

    fn set_frame_pointer(&mut self, value: u64) {
        self.registers[REGISTER_FRAME_POINTER] = value;
    }

    fn get_gp_register(&self, gp: usize) -> u64 {
        if gp < 6 {
            return self.registers[gp];
        }
        panic!("Invalid GP register {}", gp); // Panicking expected as that simply should
                                              // never happen at runtime
    }

    fn set_gp_register(&mut self, gp: usize, value: u64) {
        let mut gp = gp;
        if gp < 6 {
            self.registers[gp] = value;
        } else {
            panic!("Invalid GP register {}", gp);
        }
    }

    fn get_hw_register(&self, hw: usize) -> u64 {
        let hw_base = 14;
        if hw < 2 {
            return self.registers[hw_base + hw];
        }
        panic!("Invalid hardware register {}", hw); // Panicking expected as that simply should
                                                    // never happen at runtime
    }

    fn set_hw_register(&mut self, hw: usize, value: u64) {
        let hw_base = 14;
        if hw < 2 {
            self.registers[hw_base + hw] = value;
        } else {
            panic!("Invalid hardware register {}", hw);
        }
    }

    fn get_register(&self, register: usize) -> u64 {
        if register < 16 {
            return self.registers[register];
        }
        0
    }

    fn set_register(&mut self, register: usize, value: u64) {
        if register < 16 {
            self.registers[register] = value;
        }
    }
}
