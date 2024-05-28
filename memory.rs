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
                    panic!("Endereço de memória fora do intervalo ao tentar ler um byte: {:#X}", address)
                }
            } else {
                panic!("Endereço de memória fora do intervalo ao tentar ler um byte: {:#X}", address)
            }
        } else {
            panic!("Endereço de memória fora do intervalo ao tentar ler um byte: {:#X}", address)
        }
    }

    pub fn write_byte(&mut self, address: usize, value: u8) {
        let rom_len = self.rom.len();
        let ram_len = self.ram.len();
        let vram_len = self.vram.len();

        if address < rom_len {
            panic!("Não é possível escrever na ROM: {:#X}", address)
        } else if let Some(ram_address) = address.checked_sub(rom_len) {
            if ram_address < ram_len {
                self.ram[ram_address] = value;
            } else if let Some(vram_address) = ram_address.checked_sub(ram_len) {
                if vram_address < vram_len {
                    self.vram[vram_address] = value;
                } else {
                    panic!("Endereço de memória fora do intervalo ao tentar escrever um byte: {:#X}", address)
                }
            } else {
                panic!("Endereço de memória fora do intervalo ao tentar escrever um byte: {:#X}", address)
            }
        } else {
            panic!("Endereço de memória fora do intervalo ao tentar escrever um byte: {:#X}", address)
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
                        panic!("Endereço de memória fora do intervalo ao tentar ler uma palavra de 32 bits: {:#X}", address)
                    }
                } else {
                    panic!("Endereço de memória fora do intervalo ao tentar ler uma palavra de 32 bits: {:#X}", address)
                }
            } else {
                panic!("Endereço de memória fora do intervalo ao tentar ler uma palavra de 32 bits: {:#X}", address)
            }
        } else {
            panic!("Endereço de memória inválido ao tentar ler uma palavra de 32 bits: {:#X}", address)
        }
    }

    pub fn write_word(&mut self, address: usize, value: u32) {
        let bytes = value.to_le_bytes();

        if let Some(checked_address) = address.checked_add(3) {
            let rom_len = self.rom.len();
            let ram_len = self.ram.len();
            let vram_len = self.vram.len();

            if checked_address < rom_len {
                panic!("Não é possível escrever na ROM: {:#X}", address)
            } else if let Some(ram_address) = checked_address.checked_sub(rom_len) {
                if ram_address < ram_len {
                    self.ram[ram_address..ram_address + 4].copy_from_slice(&bytes);
                } else if let Some(vram_address) = ram_address.checked_sub(ram_len) {
                    if vram_address < vram_len {
                        self.vram[vram_address..vram_address + 4].copy_from_slice(&bytes);
                    } else {
                        panic!("Endereço de memória fora do intervalo ao tentar escrever uma palavra de 32 bits: {:#X}", address)
                    }
                } else {
                    panic!("Endereço de memória fora do intervalo ao tentar escrever uma palavra de 32 bits: {:#X}", address)
                }
            } else {
                panic!("Endereço de memória fora do intervalo ao tentar escrever uma palavra de 32 bits: {:#X}", address)
            }
        } else {
            panic!("Endereço de memória inválido ao tentar escrever uma palavra de 32 bits: {:#X}", address)
        }
    }
}

fn main() {
    let ram_size = 0x40000; // 256KB
    let vram_size = 0x8000; // 32KB
    let rom_data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
    let memory = Memory::new(rom_data, ram_size, vram_size);

    let data = memory.read_byte(0x0000);
    println!("Data read from ROM: 0x{:X}", data);
}