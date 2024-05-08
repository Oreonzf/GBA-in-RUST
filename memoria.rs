struct Memory {
    // Memória ROM
    rom: Vec<u8>,
    // Memória RAM interna
    internal_ram: Vec<u8>,
    // Memória de vídeo
    video_ram: Vec<u8>,
    // Outras regiões de memória podem ser adicionadas aqui
}

impl Memory {
    fn new(rom: Vec<u8>, ram_size: usize, vram_size: usize) -> Self {
        Memory {
            rom,
            internal_ram: vec![0; ram_size],
            video_ram: vec![0; vram_size],
        }
    }

    fn read_byte(&self, address: u32) -> u8 {
        match address {
            // Região de ROM
            0x0000..=0x7FFF => {
                // Implementar lógica para leitura da ROM
                // Exemplo:
                self.rom[address as usize]
            }
            // Região de RAM interna
            0x2000000..=0x203FFFF => {
                // Implementar lógica para leitura da RAM interna
                // Exemplo:
                self.internal_ram[(address - 0x2000000) as usize]
            }
            // Região de memória de vídeo
            0x4000000..=0x4003FFFF => {
                // Implementar lógica para leitura da memória de vídeo (VRAM)
                // Exemplo:
                self.video_ram[(address - 0x4000000) as usize]
            }
            _ => panic!("Endereço de memória inválido: 0x{:X}", address),
        }
    }

    fn write_byte(&mut self, address: u32, value: u8) {
        match address {
            // Região de RAM interna
            0x2000000..=0x203FFFF => {
                // Implementar lógica para escrita na RAM interna
                // Exemplo:
                self.internal_ram[(address - 0x2000000) as usize] = value;
            }
            // Região de memória de vídeo
            0x4000000..=0x4003FFFF => {
                // Implementar lógica para escrita na memória de vídeo (VRAM)
                // Exemplo:
                self.video_ram[(address - 0x4000000) as usize] = value;
            }
            _ => panic!("Endereço de memória inválido: 0x{:X}", address),
        }
    }
}

fn main() {
    // Tamanho da RAM interna e da VRAM (valores hipotéticos)
    let ram_size = 0x40000; // 256KB
    let vram_size = 0x8000; // 32KB

    // Carregando uma ROM hipotética
    let rom_data: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

    // Criando uma instância da memória
    let mut memory = Memory::new(rom_data, ram_size, vram_size);

    // Lendo e escrevendo dados na memória
    let data = memory.read_byte(0x0000); // Lendo da ROM
    println!("Data lida da ROM: 0x{:X}", data);

    memory.write_byte(0x2000000, 0xFF); // Escrevendo na RAM interna
    let ram_data = memory.read_byte(0x2000000); // Lendo da RAM interna
    println!("Data lida da RAM interna: 0x{:X}", ram_data);

    memory.write_byte(0x4000000, 0xAA); // Escrevendo na VRAM
    let vram_data = memory.read_byte(0x4000000); // Lendo da VRAM
    println!("Data lida da VRAM: 0x{:X}", vram_data);
}
