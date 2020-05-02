use rand::Rng;
use bit_vec::BitVec;
use std::fs::File;
use std::io::prelude::{Read};
use std::collections::LinkedList;
use std::path::Path;

pub struct Chip8
{
    pub display_data: [[bool; 64]; 32],
    pub display_data2: [bool; 2048],
    RAM: [u8; 4096],
    V: [u8; 16], // Regsiters V0 to VF
    key_state:  [bool; 16],
    stack: LinkedList<u16>,

    pub sound_timer: u8,
    PC: usize, // Program Counter
    address_register: usize,
    delay_timer: u8,
}

impl Chip8 {

    pub fn new() -> Self {
        Self {
            RAM: [0; 4096],
            V: [0; 16],
            key_state: [false; 16],
            display_data: [[false; 64]; 32],
            display_data2: [false; 2048],
            stack: LinkedList::new(),
            PC: 0x200,
            address_register: 0,
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn init_font(mut self) -> Self
    {
        let font_data = [
                            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
                            0x20, 0x60, 0x20, 0x20, 0x70, // 1
                            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
                            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
                            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
                            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
                            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
                            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
                            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
                            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
                            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
                            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
                            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
                            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
                            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
                            0xF0, 0x80, 0xF0, 0x80, 0x80  // F
                        ];
        for i in 0..font_data.len() {
            self.RAM[i] = font_data[i];
        }
        self
    }

    pub fn load_rom(&mut self, filename: &str)
    {
        let mut file = File::open(Path::new(filename)).expect("Couldn't load rom");
        file.read(&mut self.RAM[0x200..]).expect("Couldn't read rom");
    }


    pub fn update_timers(&mut self)
    {
        if self.delay_timer > 0
        {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0
        {
            self.sound_timer -= 1;
        }
    }

    pub fn key_press(&mut self, keycode: usize) {
        self.key_state[keycode] = true;
    }

    pub fn key_release(&mut self, keycode: usize) {
        self.key_state[keycode] = false;
    }

    pub fn execute_instruction(&mut self)
    {

        let hi = self.RAM[self.PC] as u16;
        let lo = self.RAM[self.PC+1] as u16;

        let opcode = (hi << 8) | lo;

        match opcode & 0xF000 {
            0x0000 => {
                match opcode
                {
                    0x00E0 => self.opc_00e0(),
                    0x00EE => self.opc_00ee(),
                    _ => panic!("Read invalid opcode!")
                }
            },
            0x1000 => self.opc_1nnn(opcode),
            0x2000 => self.opc_2nnn(opcode),
            0x3000 => self.opc_3xnn(opcode),
            0x4000 => self.opc_4xnn(opcode),
            0x5000 => self.opc_5xy0(opcode),
            0x6000 => self.opc_6xnn(opcode),
            0x7000 => self.opc_7xnn(opcode),
            0x8000 => {
                match opcode & 0x000F
                {
                    0x0000 => self.opc_8xy0(opcode),
                    0x0001 => self.opc_8xy1(opcode),
                    0x0002 => self.opc_8xy2(opcode),
                    0x0003 => self.opc_8xy3(opcode),
                    0x0004 => self.opc_8xy4(opcode),
                    0x0005 => self.opc_8xy5(opcode),
                    0x0006 => self.opc_8xy6(opcode),
                    0x0007 => self.opc_8xy7(opcode),
                    0x000E => self.opc_8xye(opcode),
                    _ => panic!("Read invalid opcode!")
                }
            },
            0x9000 => self.opc_9xy0(opcode),
            0xA000 => self.opc_annn(opcode),
            0xB000 => self.opc_bnnn(opcode),
            0xC000 => self.opc_cxnn(opcode),
            0xD000 => self.opc_dxyn(opcode),
            0xE000 => {
                match opcode & 0x00FF
                {
                    0x009E => self.opc_ex9e(opcode),
                    0x00A1 => self.opc_exa1(opcode),
                    _ => panic!("Read invalid opcode!")
                }
            },
            0xF000 => {
                match opcode & 0x00FF
                {
                    0x0007 => self.opc_fx07(opcode),
                    0x000A => self.opc_fx0a(opcode),
                    0x0015 => self.opc_fx15(opcode),
                    0x0018 => self.opc_fx18(opcode),
                    0x001E => self.opc_fx1e(opcode),
                    0x0029 => self.opc_fx29(opcode),
                    0x0033 => self.opc_fx33(opcode),
                    0x0055 => self.opc_fx55(opcode),
                    0x0065 => self.opc_fx65(opcode),
                    _ => panic!("Read invalid opcode!")
                }
            },
            _ => panic!("Read invalid opcode!")
        }
    }

    fn opc_00e0(&mut self) {
        self.display_data = [[false; 64]; 32];
        self.PC += 2;
    }

    fn opc_00ee(&mut self) {
        if self.stack.back().is_some()
        {
            self.PC = *self.stack.back().unwrap() as usize;
            self.stack.pop_back();
        }
        self.PC += 2;
    }

    fn opc_1nnn(&mut self, opcode: u16) {
        self.PC = (opcode & 0x0FFF) as usize;
    }

    // Call subroutine at nnn
    // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
    fn opc_2nnn(&mut self, opcode: u16) {
        self.stack.push_back(self.PC as u16);
	    self.PC = (opcode & 0x0FFF) as usize;
    }


    fn opc_3xnn(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        if self.V[x] == (opcode & 0x00FF) as u8
        {
            self.PC += 2;
        }
        self.PC += 2;
    }

    fn opc_4xnn(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        if self.V[x] != (opcode & 0x00FF) as u8
        {
            self.PC += 2;
        }
        self.PC += 2;
    }

    fn opc_5xy0(&mut self, opcode: u16) {
        let (x, y) = self.get_xy(opcode);
        if self.V[x] == self.V[y]
        {
            self.PC += 2;
        }
        self.PC += 2;
    }

    fn opc_6xnn(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        self.V[x] = (opcode & 0x00FF) as u8;
        self.PC += 2;
    }

    fn opc_7xnn(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        let value = (opcode & 0x00FF) as u8;
        self.V[x] = self.V[x].wrapping_add(value);
        self.PC += 2;
    }

    fn opc_8xy0(&mut self, opcode: u16) {
        let (x, y) = self.get_xy(opcode);
        self.V[x] = self.V[y];
        self.PC += 2;
    }

    fn opc_8xy1(&mut self, opcode: u16) {
        let (x, y) = self.get_xy(opcode);

        self.V[x] |= self.V[y];
        self.PC += 2;
    }

    fn opc_8xy2(&mut self, opcode: u16) {
        let (x, y) = self.get_xy(opcode);

        self.V[x] &= self.V[y];
        self.PC += 2;
    }

    fn opc_8xy3(&mut self, opcode: u16) {
        let (x, y) = self.get_xy(opcode);

        self.V[x] ^= self.V[y];
        self.PC += 2;
    }

    fn opc_8xy4(&mut self, opcode: u16) {
        let (x, y) = self.get_xy(opcode);

        let (_, overflow_occurs) = self.V[x].overflowing_add(self.V[y]);

        self.V[0xF] = if overflow_occurs {1} else {0};
        self.V[x] = self.V[x].wrapping_add(self.V[y]);
        self.PC += 2;
    }

    fn opc_8xy5(&mut self, opcode: u16) {
        let (x, y) = self.get_xy(opcode);

        self.V[0xF] = if self.V[y] > self.V[x]  {0} else {1};
        self.V[x] = self.V[x].wrapping_sub(self.V[y]);
        self.PC += 2;
    }

    fn opc_8xy6(&mut self, opcode: u16) {
        let (x, y) = self.get_xy(opcode);

        // Set the least significant bit to VF
        self.V[0xF] = self.V[x] & 1;
        self.V[x] = self.V[y] >> 1;
        self.PC += 2;
    }

    fn opc_8xy7(&mut self, opcode: u16) {
        let (x, y) = self.get_xy(opcode);

        self.V[0xF] = if self.V[x] > self.V[y]  {0} else {1};
        self.V[y] = self.V[y].wrapping_sub(self.V[x]);
        self.PC += 2;
    }

    fn opc_8xye(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);

        // Set most significant bit to VF
        self.V[0xF] = self.V[x] >> 7;
        self.V[x] = self.V[x] << 1;
        self.PC += 2;
    }

    fn opc_9xy0(&mut self, opcode: u16) {
        let (x, y) = self.get_xy(opcode);
        if self.V[x] != self.V[y]
        {
            self.PC += 2;
        }
        self.PC += 2;
    }

    fn opc_annn(&mut self, opcode: u16) {
        self.address_register = (opcode & 0x0FFF) as usize;
        self.PC += 2;
    }

    fn opc_bnnn(&mut self, opcode: u16) {
        self.PC = (opcode & 0x0FFF) as usize + self.V[0] as usize;
    }

    fn opc_cxnn(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        let mut rng = rand::thread_rng();
        self.V[x] = rng.gen::<u8>() & (opcode & 0x00FF) as u8;
        self.PC += 2;
    }

    fn opc_dxyn(&mut self, opcode: u16) {
        let (x, y) = self.get_xy(opcode);
        let height = (opcode & 0x000F) as usize;

        let sprite_data = &mut self.RAM[self.address_register .. (self.address_register+height)];
        let sprite = BitVec::from_bytes(sprite_data);

        let mut set_flag: u8 = 0;
        for i in 0..height {
            for j in 0..8usize
            {
                let y_coord = (self.V[y] as usize + i) % 32;
                let x_coord = (self.V[x] as usize + j) % 64;
                let new_value = self.display_data[y_coord][x_coord] ^ sprite[(8*i) + j];

                if self.display_data[y_coord][x_coord] && !new_value
                {
                    set_flag = 1;
                }

                self.display_data[y_coord][x_coord] = new_value;
            }
        }

        self.V[0xF] = set_flag;
        self.PC += 2;
    }


    fn opc_ex9e(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        if self.key_state[self.V[x] as usize]
        {
            self.PC += 2;
        }
        self.PC += 2;
    }

    fn opc_exa1(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        if !self.key_state[self.V[x] as usize]
        {
            self.PC += 2;
        }
        self.PC += 2;
    }

    fn opc_fx07(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        self.V[x] = self.delay_timer;
        self.PC += 2;
    }

    // Wait for a keypress and store the result in register VX
    fn opc_fx0a(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);

        match self.key_state.iter().position(|&x| x)
        {
            Some(value) => self.V[x] = value as u8,
            None => self.PC -= 2
        }
        self.PC += 2;
    }

    fn opc_fx15(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        self.delay_timer = self.V[x];
        self.PC += 2;
    }

    fn opc_fx18(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        self.sound_timer = self.V[x];
        self.PC += 2;
    }

    fn opc_fx1e(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        self.address_register += self.V[x] as usize;
        self.PC += 2;
    }

    // The address register is set to the location of the hexadecimal sprite corresponding to the value of Vx
    // Since all fonts have 5 bytes, correspondind to their height, the value is multiplied by 5
    fn opc_fx29(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);

        self.address_register = (self.V[x] * 5) as usize;
        self.PC += 2;
    }

    fn opc_fx33(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);
        let value = self.V[x];

        self.RAM[self.address_register] = value / 100;
        self.RAM[self.address_register+1] = (value / 10) % 10;
        self.RAM[self.address_register+2] = value % 10;
        self.PC += 2;
    }

    fn opc_fx55(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);

        for pos in 0..x+1 {
            self.RAM[self.address_register+pos] = self.V[pos];
        }
        self.address_register += x + 1;
        self.PC += 2;
    }

    fn opc_fx65(&mut self, opcode: u16) {
        let (x, _) = self.get_xy(opcode);

        for pos in 0..x+1 {
            self.V[pos] = self.RAM[self.address_register];
            self.address_register += 1;
        }
        self.PC += 2;
    }

    fn get_xy(&mut self, opcode: u16) -> (usize, usize)
    {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        (x, y)
    }
}
