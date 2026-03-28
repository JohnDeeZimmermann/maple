use crate::maple::{
    cpu::{ExecutionMode, MapleCPU},
    interrupt_codes::INTERRUPT_CODE_PAGE_FAULT,
};

pub struct Memory {
    memory: Vec<u64>,
}
impl Memory {
    pub fn new(size: u32) -> Self {
        Memory {
            memory: vec![0; size as usize],
        }
    }
}

impl Memory {
    pub fn get(&self, memory_address: u32) -> u64 {
        if memory_address >= self.memory.len() as u32 {
            panic!("Memory access out of bounds");
        }
        self.memory[memory_address as usize]
    }

    pub fn set(&mut self, memory_address: u32, value: u64) {
        if memory_address >= self.memory.len() as u32 {
            panic!("Memory access out of bounds");
        }
        self.memory[memory_address as usize] = value;
    }

    pub fn get_size(&self) -> u32 {
        return self.memory.len() as u32;
    }

    pub fn virtual_to_physical(&self, address: u32, cpu: &mut MapleCPU) -> u32 {
        let table_base = cpu.get_page_table_base() as u32;
        let page_offset = address & 0xFFF; // The last twelve bits
        let page_table_index = (address >> 12) & 0x3FF; // The next ten bits
        let page_directory_index = (address >> 22) & 0x3FF; // The next ten bits

        let table_base = table_base;

        // Table base = index = address of page table
        let page_directory_pointer = table_base + page_directory_index;
        let page_table_address = (self.get(page_directory_pointer) & 0xFFFFFFFF) as u32;

        // Address of page table + table offset = page address
        let page_table_pointer = page_table_address + page_table_index;
        let page_address = (self.get(page_table_pointer) & 0xFFFFFFFF) as u32;

        // The first 32 bits of the first entries determine the length which may not be exceeded.
        let directory_length = (self.get(table_base) >> 32) as u32;
        let page_table_length = (self.get(page_table_address) >> 32) as u32;

        if page_table_address == 0
            || page_address == 0
            || page_directory_index > directory_length
            || page_table_length > page_table_index
        {
            cpu.raise_interrupt(INTERRUPT_CODE_PAGE_FAULT);
            return 0;
        }

        // Page address = page offset = Physical address
        page_address + page_offset
    }

    pub fn read(&self, address: u32, cpu: &mut MapleCPU) -> u64 {
        let actual_address: u32 = match cpu.mode {
            ExecutionMode::User => {
                let result = self.virtual_to_physical(address, cpu);
                if result == 0 {
                    return 0;
                }
                result
            }
            ExecutionMode::Kernel => address,
        };

        self.get(actual_address)
    }

    pub fn write(&mut self, address: u32, value: u64, cpu: &mut MapleCPU) {
        let actual_address: u32 = match cpu.mode {
            ExecutionMode::User => {
                let result = self.virtual_to_physical(address, cpu);
                if result == 0 {
                    return;
                }
                result
            }
            ExecutionMode::Kernel => address,
        };

        self.set(actual_address, value);
    }
}
