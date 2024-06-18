use std::fs::File;
use std::io::Read;

pub struct Memory {
    rom: Vec<u8>,
    ram: Vec<u8>,
    vram: Vec<u8>,
}

impl Memory {
    pub fn new(rom_data: Vec<u8>, ram_size: usize, vram_size: usize) -> Self {
        let rom = rom_data;
        let ram = vec![0; ram_size];
        let vram = vec![0; vram_size];
        Memory { rom, ram, vram }
    }

    pub fn load_rom(&mut self, path: &str) {
        let mut file = File::open(path).expect("Failed to open ROM file");
        file.read_to_end(&mut self.rom).expect("Failed to read ROM file");
    }

    pub fn read_byte(&self, address: usize) -> u8 {
        let rom_len = self.rom.len();
        let ram_len = self.ram.len();
        let vram_len = self.vram.len();

        if address < rom_len {
            self.rom[address]
        } else if let Some(ram_address) = address.checked_sub(rom_len) {
            if ram_address < ram_len {
                self.ram[ram_address]
            } else if let Some(vram_address) = ram_address.checked_sub(ram_len) {
                if vram_address < vram_len {
                    self.vram[vram_address]
                } else {
                    panic!("Memory address out of range when reading byte: {:#X}", address)
                }
            } else {
                panic!("Memory address out of range when reading byte: {:#X}", address)
            }
        } else {
            panic!("Memory address out of range when reading byte: {:#X}", address)
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        let rom_len = self.rom.len();
        let ram_len = self.ram.len();
        let vram_len = self.vram.len();

        if address < rom_len {
            panic!("Cannot write to ROM: {:#X}", address)
        } else if let Some(ram_address) = address.checked_sub(rom_len) {
            if ram_address < ram_len {
                self.ram[ram_address] = value;
            } else if let Some(vram_address) = ram_address.checked_sub(ram_len) {
                if vram_address < vram_len {
                    self.vram[vram_address] = value;
                } else {
                    panic!("Memory address out of range when writing byte: {:#X}", address)
                }
            } else {
                panic!("Memory address out of range when writing byte: {:#X}", address)
            }
        } else {
            panic!("Memory address out of range when writing byte: {:#X}", address)
        }
    }

    pub fn read_word(&self, address: usize) -> u32 {
        let rom_len = self.rom.len();
        let ram_len = self.ram.len();
        let vram_len = self.vram.len();

        if let Some(checked_address) = address.checked_add(3) {
            if checked_address < rom_len {
                u32::from_le_bytes([
                    self.rom[address],
                    self.rom[address + 1],
                    self.rom[address + 2],
                    self.rom[address + 3],
                ])
            } else if let Some(ram_address) = checked_address.checked_sub(rom_len) {
                if ram_address < ram_len {
                    u32::from_le_bytes([
                        self.ram[ram_address],
                        self.ram[ram_address + 1],
                        self.ram[ram_address + 2],
                        self.ram[ram_address + 3],
                    ])
                } else if let Some(vram_address) = ram_address.checked_sub(ram_len) {
                    if vram_address < vram_len {
                        u32::from_le_bytes([
                            self.vram[vram_address],
                            self.vram[vram_address + 1],
                            self.vram[vram_address + 2],
                            self.vram[vram_address + 3],
                        ])
                    } else {
                        panic!("Memory address out of range when reading word: {:#X}", address)
                    }
                } else {
                    panic!("Memory address out of range when reading word: {:#X}", address)
                }
            } else {
                panic!("Memory address out of range when reading word: {:#X}", address)
            }
        } else {
            panic!("Invalid memory address when reading word: {:#X}", address)
        }
    }

    pub fn write_word(&mut self, address: usize, value: u32) {
        let bytes = value.to_le_bytes();

        if let Some(checked_address) = address.checked_add(3) {
            let rom_len = self.rom.len();
            let ram_len = self.ram.len();
            let vram_len = self.vram.len();

            if checked_address < rom_len {
                panic!("Cannot write to ROM: {:#X}", address)
            } else if let Some(ram_address) = checked_address.checked_sub(rom_len) {
                if ram_address < ram_len {
                    self.ram[ram_address..ram_address + 4].copy_from_slice(&bytes);
                } else if let Some(vram_address) = ram_address.checked_sub(ram_len) {
                    if vram_address < vram_len {
                        self.vram[vram_address..vram_address + 4].copy_from_slice(&bytes);
                    } else {
                        panic!("Memory address out of range when writing word: {:#X}", address)
                    }
                } else {
                    panic!("Memory address out of range when writing word: {:#X}", address)
                }
            } else {
                panic!("Memory address out of range when writing word: {:#X}", address)
            }
        } else {
            panic!("Invalid memory address when writing word: {:#X}", address)
        }
    }
}
