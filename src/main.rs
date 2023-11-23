// u8 => 0000 0000 => 1 byte




/* REGISTER VISUALISATION

15 .. 8 7 .. 0
    A      F
    B      C
    D      E
    H      L
*/

struct CPU{
    //registers
    a:u8,    f:u8,
    b:u8,    c:u8,
    d:u8,    e:u8,
    h:u8,    l:u8,

    //pointers
    sp:u16, //Stack Pointer 
    pc:u16, //Program Counter

    //flags
    zf:bool,
    nf:bool,
    hf:bool,
    cf:bool,

    //ram
    ram:[u8;0xFFFF]
}
// 0x0000 => 0xFFFF (65 535) (4 bytes)
// liste de 0xFFFF u8

impl CPU{

    fn init() -> CPU{

        let cpu:CPU = CPU{
            a:0x0,    f:0x0,
            b:0x0,    c:0x0,
            d:0x0,    e:0x0,
            h:0x0,    l:0x0,
        
            //pointers
            sp:0x00, //Stack Pointer 
            pc:0x00, //Program Counter
        
            //flags
            zf:false, //0x0
            nf:false,
            hf:false,
            cf:false,
        
            //ram
            ram:[0;0xFFFF] 
        };

        return cpu;


    }

    fn get_a(&mut self) -> u8{
        return self.a;
    }

    fn read(&mut self,pointer: usize) -> u8{
        return self.ram[pointer];
    }

    fn execute(&mut self,opcode: u8){
        self.a = 0;
        match opcode{
            0x00=>{
                //NOP
            },
            0x01=>{
                println!("hey")
            },    
            0x02 => {
                 println!("hey") 
                },
            0x03 => { println!("hey") },
            0x04 => { println!("hey") },
            0x05 => { println!("hey") },
            0x06 => { println!("hey") },
            0x07 => { println!("hey") },
            0x08 => { println!("hey") },
            0x09 => { println!("hey") },
            0x0A => { println!("hey") },
            0x0B => { println!("hey") },
            0x0C => { println!("hey") },
            0x0D => { println!("hey") },
            0x0E => { println!("hey") },
            0x0F => { println!("hey") },
            0x10 => { println!("hey") },
            0x11 => { println!("hey") },
            0x12 => { println!("hey") },
            0x13 => { println!("hey") },
            0x14 => { println!("hey") },
            0x15 => { println!("hey") },
            0x16 => { println!("hey") },
            0x17 => { println!("hey") },
            0x18 => { println!("hey") },
            0x19 => { println!("hey") },
            0x1A => { println!("hey") },
            0x1B => { println!("hey") },
            0x1C => { println!("hey") },
            0x1D => { println!("hey") },
            0x1E => { println!("hey") },
            0x1F => { println!("hey") },
            0x20 => { println!("hey") },
            0x21 => { println!("hey") },
            0x22 => { println!("hey") },
            0x23 => { println!("hey") },
            0x24 => { println!("hey") },
            0x25 => { println!("hey") },
            0x26 => { println!("hey") },
            0x27 => { println!("hey") },
            0x28 => { println!("hey") },
            0x29 => { println!("hey") },
            0x2A => { println!("hey") },
            0x2B => { println!("hey") },
            0x2C => { println!("hey") },
            0x2D => { println!("hey") },
            0x2E => { println!("hey") },
            0x2F => { println!("hey") },
            0x30 => { println!("hey") },
            0x31 => { println!("hey") },
            0x32 => { println!("hey") },
            0x33 => { println!("hey") },
            0x34 => { println!("hey") },
            0x35 => { println!("hey") },
            0x36 => { println!("hey") },
            0x37 => { println!("hey") },
            0x38 => { println!("hey") },
            0x39 => { println!("hey") },
            0x3A => { println!("hey") },
            0x3B => { println!("hey") },
            0x3C => { println!("hey") },
            0x3D => { println!("hey") },
            0x3E => { println!("hey") },
            0x3F => { println!("hey") },
            0x40 => { println!("hey") },
            0x41 => {
                //LD B,C
                self.b = self.c;     
            },
            0x42 => {
                //LD B,D
                self.b = self.d;     
 
            },
            0x43 => { 
                //LD B,E
                self.b = self.e; 
            },
            0x44 => { 
                //LD B,H
                self.b = self.h;
             },
            0x45 => { 
                //LD B,L
                self.b = self.l; 
            },
            0x46 => { println!("hey") },
            0x47 => { println!("hey") },
            0x48 => { println!("hey") },
            0x49 => { println!("hey") },
            0x4A => { println!("hey") },
            0x4B => { println!("hey") },
            0x4C => { println!("hey") },
            0x4D => { println!("hey") },
            0x4E => { println!("hey") },
            0x4F => { println!("hey") },
            0x50 => { println!("hey") },
            0x51 => { println!("hey") },
            0x52 => { println!("hey") },
            0x53 => { println!("hey") },
            0x54 => { println!("hey") },
            0x55 => { println!("hey") },
            0x56 => { println!("hey") },
            0x57 => { println!("hey") },
            0x58 => { println!("hey") },
            0x59 => { println!("hey") },
            0x5A => { println!("hey") },
            0x5B => { println!("hey") },
            0x5C => { println!("hey") },
            0x5D => { println!("hey") },
            0x5E => { println!("hey") },
            0x5F => { println!("hey") },
            0x60 => { println!("hey") },
            0x61 => { println!("hey") },
            0x62 => { println!("hey") },
            0x63 => { println!("hey") },
            0x64 => { println!("hey") },
            0x65 => { println!("hey") },
            0x66 => { println!("hey") },
            0x67 => { println!("hey") },
            0x68 => { println!("hey") },
            0x69 => { println!("hey") },
            0x6A => { println!("hey") },
            0x6B => { println!("hey") },
            0x6C => { println!("hey") },
            0x6D => { println!("hey") },
            0x6E => { println!("hey") },
            0x6F => { println!("hey") },
            0x70 => { println!("hey") },
            0x71 => { println!("hey") },
            0x72 => { println!("hey") },
            0x73 => { println!("hey") },
            0x74 => { println!("hey") },
            0x75 => { println!("hey") },
            0x76 => { println!("hey") },
            0x77 => { println!("hey") },
            0x78 => { println!("hey") },
            0x79 => { println!("hey") },
            0x7A => { println!("hey") },
            0x7B => { println!("hey") },
            0x7C => { println!("hey") },
            0x7D => { println!("hey") },
            0x7E => { println!("hey") },
            0x7F => { println!("hey") },
            0x80 => { println!("hey") },
            0x81 => { println!("hey") },
            0x82 => { println!("hey") },
            0x83 => { println!("hey") },
            0x84 => { println!("hey") },
            0x85 => { println!("hey") },
            0x86 => { println!("hey") },
            0x87 => { println!("hey") },
            0x88 => { println!("hey") },
            0x89 => { println!("hey") },
            0x8A => { println!("hey") },
            0x8B => { println!("hey") },
            0x8C => { println!("hey") },
            0x8D => { println!("hey") },
            0x8E => { println!("hey") },
            0x8F => { println!("hey") },
            0x90 => { println!("hey") },
            0x91 => { println!("hey") },
            0x92 => { println!("hey") },
            0x93 => { println!("hey") },
            0x94 => { println!("hey") },
            0x95 => { println!("hey") },
            0x96 => { println!("hey") },
            0x97 => { println!("hey") },
            0x98 => { println!("hey") },
            0x99 => { println!("hey") },
            0x9A => { println!("hey") },
            0x9B => { println!("hey") },
            0x9C => { println!("hey") },
            0x9D => { println!("hey") },
            0x9E => { println!("hey") },
            0x9F => { println!("hey") },
            0xA0 => { println!("hey") },
            0xA1 => { println!("hey") },
            0xA2 => { println!("hey") },
            0xA3 => { println!("hey") },
            0xA4 => { println!("hey") },
            0xA5 => { println!("hey") },
            0xA6 => { println!("hey") },
            0xA7 => { println!("hey") },
            0xA8 => { println!("hey") },
            0xA9 => { println!("hey") },
            0xAA => { println!("hey") },
            0xAB => { println!("hey") },
            0xAC => { println!("hey") },
            0xAD => { println!("hey") },
            0xAE => { println!("hey") },
            0xAF => { println!("hey") },
            0xB0 => { println!("hey") },
            0xB1 => { println!("hey") },
            0xB2 => { println!("hey") },
            0xB3 => { println!("hey") },
            0xB4 => { println!("hey") },
            0xB5 => { println!("hey") },
            0xB6 => { println!("hey") },
            0xB7 => { println!("hey") },
            0xB8 => { println!("hey") },
            0xB9 => { println!("hey") },
            0xBA => { println!("hey") },
            0xBB => { println!("hey") },
            0xBC => { println!("hey") },
            0xBD => { println!("hey") },
            0xBE => { println!("hey") },
            0xBF => { println!("hey") },
            0xC0 => { println!("hey") },
            0xC1 => { println!("hey") },
            0xC2 => { println!("hey") },
            0xC3 => { println!("hey") },
            0xC4 => { println!("hey") },
            0xC5 => { println!("hey") },
            0xC6 => { println!("hey") },
            0xC7 => { println!("hey") },
            0xC8 => { println!("hey") },
            0xC9 => { println!("hey") },
            0xCA => { println!("hey") },
            0xCB => { println!("hey") },
            0xCC => { println!("hey") },
            0xCD => { println!("hey") },
            0xCE => { println!("hey") },
            0xCF => { println!("hey") },
            0xD0 => { println!("hey") },
            0xD1 => { println!("hey") },
            0xD2 => { println!("hey") },
            0xD3 => { println!("hey") },
            0xD4 => { println!("hey") },
            0xD5 => { println!("hey") },
            0xD6 => { println!("hey") },
            0xD7 => { println!("hey") },
            0xD8 => { println!("hey") },
            0xD9 => { println!("hey") },
            0xDA => { println!("hey") },
            0xDB => { println!("hey") },
            0xDC => { println!("hey") },
            0xDD => { println!("hey") },
            0xDE => { println!("hey") },
            0xDF => { println!("hey") },
            0xE0 => { println!("hey") },
            0xE1 => { println!("hey") },
            0xE2 => { println!("hey") },
            0xE3 => { println!("hey") },
            0xE4 => { println!("hey") },
            0xE5 => { println!("hey") },
            0xE6 => { println!("hey") },
            0xE7 => { println!("hey") },
            0xE8 => { println!("hey") },
            0xE9 => { println!("hey") },
            0xEA => { println!("hey") },
            0xEB => { println!("hey") },
            0xEC => { println!("hey") },
            0xED => { println!("hey") },
            0xEE => { println!("hey") },
            0xEF => { println!("hey") },
            0xF0 => { println!("hey") },
            0xF1 => { println!("hey") },
            0xF2 => { println!("hey") },
            0xF3 => { println!("hey") },
            0xF4 => { println!("hey") },
            0xF5 => { println!("hey") },
            0xF6 => { println!("hey") },
            0xF7 => { println!("hey") },
            0xF8 => { println!("hey") },
            0xF9 => { println!("hey") },
            0xFA => { println!("hey") },
            0xFB => { println!("hey") },
            0xFC => { println!("hey") },
            0xFD => { println!("hey") },
            0xFE => { println!("hey") },
            0xFF => { println!("hey") },
            _=>println!("rien")
        }
    }


}


fn main() {
    let mut cpu:CPU = CPU::init();
    let x =cpu.get_a();
    println!("get a return {x}");

}
