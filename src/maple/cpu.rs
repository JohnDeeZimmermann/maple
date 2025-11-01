pub enum ExecutionMode {
    User,
    Kernel,
}

pub trait CPU {
    fn raise_interrupt(&mut self, code: u64);

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

pub const REGISTER_STACK_POINTER: usize = 6;
pub const REGISTER_PROGRAM_COUNTER: usize = 7;
pub const REGISTER_DYNAMIC_LINK: usize = 8;
pub const REGISTER_RESULT: usize = 9;
pub const REGISTER_IO_POINTER: usize = 10;
pub const REGISTER_PAGE_TABLE_BASE: usize = 11;
pub const REGISTER_SYSTEM_INFO: usize = 12;
pub const REGISTER_FRAME_POINTER: usize = 13;

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
    fn raise_interrupt(&mut self, code: u64) {
        todo!()
    }

    fn get_stack_pointer(&self) -> u64 {
        return self.registers[REGISTER_STACK_POINTER];
    }

    fn set_stack_pointer(&mut self, value: u64) {
        self.registers[REGISTER_STACK_POINTER] = value;
    }

    fn get_program_counter(&self) -> u64 {
        return self.registers[REGISTER_PROGRAM_COUNTER];
    }

    fn set_program_counter(&mut self, value: u64) {
        self.registers[REGISTER_PROGRAM_COUNTER] = value;
    }

    fn get_dynamic_link(&self) -> u64 {
        return self.registers[REGISTER_DYNAMIC_LINK];
    }

    fn set_dynamic_link(&mut self, value: u64) {
        self.registers[REGISTER_DYNAMIC_LINK] = value;
    }

    fn get_result(&self) -> u64 {
        return self.registers[REGISTER_RESULT];
    }

    fn set_result(&mut self, value: u64) {
        self.registers[REGISTER_RESULT] = value;
    }

    fn get_io_pointer(&self) -> u64 {
        return self.registers[REGISTER_IO_POINTER];
    }

    fn set_io_pointer(&mut self, value: u64) {
        self.registers[REGISTER_IO_POINTER] = value;
    }

    fn get_page_table_base(&self) -> u64 {
        return self.registers[REGISTER_PAGE_TABLE_BASE];
    }

    fn set_page_table_base(&mut self, value: u64) {
        self.registers[REGISTER_PAGE_TABLE_BASE] = value;
    }

    fn get_system_info(&self) -> u64 {
        return self.registers[REGISTER_SYSTEM_INFO];
    }

    fn set_system_info(&mut self, value: u64) {
        self.registers[REGISTER_SYSTEM_INFO] = value;
    }

    fn get_frame_pointer(&self) -> u64 {
        return self.registers[REGISTER_FRAME_POINTER];
    }

    fn set_frame_pointer(&mut self, value: u64) {
        self.registers[REGISTER_FRAME_POINTER] = value;
    }

    fn get_gp_register(&self, gp: usize) -> u64 {
        if gp < 6 {
            return self.registers[gp];
        }
        panic!("Invalid general purpose register {}", gp);
    }

    fn set_gp_register(&mut self, gp: usize, value: u64) {
        if gp >= 6 {
            panic!("Invalid general purpose register {}", gp);
        }
        if gp < 6 {
            self.registers[gp] = value;
        }
    }

    fn get_hw_register(&self, hw: usize) -> u64 {
        let hw_base = 14;
        if hw < 2 {
            return self.registers[hw_base + hw];
        }
        panic!("Invalid hardware register {}", hw);
    }

    fn set_hw_register(&mut self, hw: usize, value: u64) {
        if hw >= 6 {
            panic!("Invalid general purpose register {}", gp);
        }
        let hw_base = 14;
        if hw < 2 {
            self.registers[hw_base + hw] = value;
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
