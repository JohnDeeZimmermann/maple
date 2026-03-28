use std::{
    fs::File,
    io::{Error, Read},
};

use crate::maple::{cpu::MapleCPU, memory::Memory};

pub fn setup_system(memory_size: u32) -> (MapleCPU, Memory) {
    let memory = Memory::new(memory_size);
    let cpu = MapleCPU::new();

    (cpu, memory)
}

pub fn setup_memory_from_file(
    memory: &mut Memory,
    file: &mut File,
    offset: u32,
) -> Result<(), std::io::Error> {
    let mut buffer: Vec<u8> = Vec::new();

    match file.read_to_end(&mut buffer) {
        Ok(size) => {
            let total_size = (size as u32) + offset;

            if total_size > memory.get_size() {
                return Err(Error::new(
                    std::io::ErrorKind::Other,
                    "File size exceeds memory size",
                ));
            }

            fill_memory(&mut buffer, memory, offset);
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn fill_memory(buffer: &mut Vec<u8>, memory: &mut Memory, offset: u32) {
    let mut filled_value: u64 = 0;
    buffer.iter().enumerate().for_each(|(index, &byte)| {
        filled_value = (filled_value << (8 * index)) | (byte as u64);
        if index % 4 == 3 {
            let memory_position = ((index / 4) as u32) + offset;
            memory.set(memory_position, filled_value);
        }
    });
}
