use crate::maple::cpu::{CPU, ExecutionMode, MapleCPU, REGISTER_PAGE_TABLE_BASE};

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
    fn get(&self, memory_address: u32) -> u64 {
        if memory_address >= self.memory.len() as u32 {
            panic!("Memory access out of bounds");
        }
        return self.memory[memory_address as usize]
    }

    fn set(&mut self, memory_address: u32, value: u64) {
        if memory_address >= self.memory.len() as u32 {
            panic!("Memory access out of bounds");
        }
        self.memory[memory_address as usize] = value;
    }
    fn virtual_to_physical(&self, address: u32, page_table_base: u32) -> u32 {
        let page_offset = address & 0xFFF; // The last twelve bits
        let page_table_index = (address >> 12) & 0x3FF; // The next ten bits
        let page_directory_index = (address >> 22) & 0x3FF; // The next ten bits

        let page_table_base = page_table_base;

        // Table base = index = address of page table
        let page_directory_pointer = page_table_base + page_directory_index;
        let page_table_address = self.get(page_directory_pointer) as u32;

        // Address of page table + table offset = page address
        let page_table_pointer = page_table_address + page_table_index;
        let page_address = self.get(page_table_pointer) as u32;

        // Page address = page offset = Physical address
        return page_address + page_offset;
    }


    pub fn read(&self, address: u32, cpu: &MapleCPU) -> u64 {
        let table_base: u32 = cpu.get_page_table_base().try_into().unwrap();
        let actual_address = self.virtual_to_physical(address, table_base);

        match cpu.mode {
            ExecutionMode::User => {

            },
            ExecutionMode::Kernel => {

            }
        }

        return 0;
    }

    pub fn write(&mut self, address: u32, value: u64) {
        // Implementation of write function
    }
}
