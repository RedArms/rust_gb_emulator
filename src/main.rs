// u8 => 0000 0000 => 1 byte

/*
Start	End	Description	Notes
0000	3FFF	16 KiB ROM bank 00	From cartridge, usually a fixed bank
4000	7FFF	16 KiB ROM Bank 01~NN	From cartridge, switchable bank via mapper (if any)
8000	9FFF	8 KiB Video RAM (VRAM)	In CGB mode, switchable bank 0/1
A000	BFFF	8 KiB External RAM	From cartridge, switchable bank if any
C000	CFFF	4 KiB Work RAM (WRAM)	
D000	DFFF	4 KiB Work RAM (WRAM)	In CGB mode, switchable bank 1~7
E000	FDFF	Mirror of C000~DDFF (ECHO RAM)	Nintendo says use of this area is prohibited.
FE00	FE9F	Object attribute memory (OAM)	
FEA0	FEFF	Not Usable	Nintendo says use of this area is prohibited
FF00	FF7F	I/O Registers	
FF80	FFFE	High RAM (HRAM)	
FFFF	FFFF	Interrupt Enable register (IE)	

8kb u8  
*/

/*
    x0  x1  x2
0x  00  12  FF
1x  55  F1  F4
2x  44  02  25
*/

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
// ram pointer 0x0000 => 0xFFFF (65 535) stored in (2 bytes) (u16) 
// ram memory 0x00 => 0xFF (255) stored in (1byte) (u8)
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
        let x = self.a;
        print!("its {x} \n");
        return self.a;
    }

    fn cast_xy(x_: u8,_y:u8) -> u16{
        return ((x_ as u16) << 4) + _y as u16;
    }

    fn set_a(&mut self,value: u8){
            self.a = value;
    }

    fn read(&mut self,pointer: usize) -> u8{ //
        return self.ram[pointer];
    }

    fn write(&mut self,pointeur: usize,value: u8){
        self.ram[pointeur] = value;
    }

    fn execute(&mut self,opcode: u8){
        match opcode{
            0x00=>{
                let x = self.a;
                print!("its {x} \n");        
                let x = (self.a as u16)<<4;
                print!("get a return {:#06x} \n",x)
                //NOP
            },
            0x01=>{
                println!("hey")
            },    
            0x02 => {
                 println!("hey") 
                },
            0x03 => { 
                println!("hey") 
            },
            0x04 => { 

                println!("hey") 
            },
            0x05 => { 

                println!("hey") 
            },
            0x06 => { 

                println!("hey") 
            },
            0x07 => { 

                println!("hey") 
            },
            0x08 => { 

                println!("hey") 
            },
            0x09 => { 

                println!("hey") 
            },
            0x0A => { 

                println!("hey") 
            },
            0x0B => { 

                println!("hey") 
            },
            0x0C => { 

                println!("hey") 
            },
            0x0D => { 

                println!("hey") 
            },
            0x0E => { 

                println!("hey") 
            },
            0x0F => { 

                println!("hey") 
            },
            0x10 => { 

                println!("hey") 
            },
            0x11 => { 

                println!("hey") 
            },
            0x12 => { 

                println!("hey") 
            },
            0x13 => { 

                println!("hey") 
            },
            0x14 => { 

                println!("hey") 
            },
            0x15 => { 

                println!("hey") 
            },
            0x16 => { 

                println!("hey") 
            },
            0x17 => { 

                println!("hey") 
            },
            0x18 => { 

                println!("hey") 
            },
            0x19 => { 

                println!("hey") 
            },
            0x1A => { 

                println!("hey") 
            },
            0x1B => { 

                println!("hey") 
            },
            0x1C => { 

                println!("hey") 
            },
            0x1D => { 

                println!("hey") 
            },
            0x1E => { 

                println!("hey") 
            },
            0x1F => { 

                println!("hey") 
            },
            0x20 => { 

                println!("hey") 
            },
            0x21 => { 

                println!("hey") 
            },
            0x22 => { 

                println!("hey") 
            },
            0x23 => { 

                println!("hey") 
            },
            0x24 => { 

                println!("hey") 
            },
            0x25 => { 

                println!("hey") 
            },
            0x26 => { 

                println!("hey") 
            },
            0x27 => { 

                println!("hey") 
            },
            0x28 => { 

                println!("hey") 
            },
            0x29 => { 

                println!("hey") 
            },
            0x2A => { 

                println!("hey") 
            },
            0x2B => { 

                println!("hey") 
            },
            0x2C => { 

                println!("hey") 
            },
            0x2D => { 

                println!("hey") 
            },
            0x2E => { 

                println!("hey") 
            },
            0x2F => { 

                println!("hey") 
            },
            0x30 => { 

                println!("hey") 
            },
            0x31 => { 

                println!("hey") 
            },
            0x32 => { 

                println!("hey") 
            },
            0x33 => { 

                println!("hey") 
            },
            0x34 => { 

                println!("hey") 
            },
            0x35 => { 

                println!("hey") 
            },
            0x36 => { 

                println!("hey") 
            },
            0x37 => { 

                println!("hey") 
            },
            0x38 => { 

                println!("hey") 
            },
            0x39 => { 

                println!("hey") 
            },
            0x3A => { 

                println!("hey") 
            },
            0x3B => { 

                println!("hey") 
            },
            0x3C => { 

                println!("hey") 
            },
            0x3D => { 

                println!("hey") 
            },
            0x3E => { 

                println!("hey") 
            },
            0x3F => { 

                println!("hey") 
            },
            0x40 => { 

                println!("hey") 
            },
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
            0x46 => { 
                // How to cast H(0xF) with L(0xE) TO HL(0xFE) 

                // h = 0xF (as u16) => 0x0F (<<4) => 0xF0 (+0x0E) => 0XFE
                let p:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.b = self.read(p as usize);
             },
            0x47 => { 
                //LD B,A
                self.b = self.a; 
            },
            0x48 => { 
                //LD C,B
                self.c = self.b;
             },
            0x49 => { 
                //LD C,C
                self.c = self.c;
             },
            0x4A => { 
                //LD C,D
                self.c = self.d;
             },
            0x4B => { 
                //LD C,E
                self.c = self.e; 
            },
            0x4C => { 
                //LD C,H
                self.c = self.h; 
            },
            0x4D => { 
                //LD C,L
                self.c = self.l; 
            },
            0x4E => { 

                let p:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.c = self.read(p as usize); 
            },
            0x4F => { 

                println!("hey") 
            },
            0x50 => { 

                println!("hey") 
            },
            0x51 => { 

                println!("hey") 
            },
            0x52 => { 

                println!("hey") 
            },
            0x53 => { 

                println!("hey") 
            },
            0x54 => { 

                println!("hey") 
            },
            0x55 => { 

                println!("hey") 
            },
            0x56 => { 

                println!("hey") 
            },
            0x57 => { 

                println!("hey") 
            },
            0x58 => { 

                println!("hey") 
            },
            0x59 => { 

                println!("hey") 
            },
            0x5A => { 

                println!("hey") 
            },
            0x5B => { 

                println!("hey") 
            },
            0x5C => { 

                println!("hey") 
            },
            0x5D => { 

                println!("hey") 
            },
            0x5E => { 

                println!("hey") 
            },
            0x5F => { 

                println!("hey") 
            },
            0x60 => { 

                println!("hey") 
            },
            0x61 => { 

                println!("hey") 
            },
            0x62 => { 

                println!("hey") 
            },
            0x63 => { 

                println!("hey") 
            },
            0x64 => { 

                println!("hey") 
            },
            0x65 => { 

                println!("hey") 
            },
            0x66 => { 

                println!("hey") 
            },
            0x67 => { 

                println!("hey") 
            },
            0x68 => { 

                println!("hey") 
            },
            0x69 => { 

                println!("hey") 
            },
            0x6A => { 

                println!("hey") 
            },
            0x6B => { 

                println!("hey") 
            },
            0x6C => { 

                println!("hey") 
            },
            0x6D => { 

                println!("hey") 
            },
            0x6E => { 

                println!("hey") 
            },
            0x6F => { 

                println!("hey") 
            },
            0x70 => { 

                println!("hey") 
            },
            0x71 => { 

                println!("hey") 
            },
            0x72 => { 

                println!("hey") 
            },
            0x73 => { 

                println!("hey") 
            },
            0x74 => { 

                println!("hey") 
            },
            0x75 => { 

                println!("hey") 
            },
            0x76 => { 

                println!("hey") 
            },
            0x77 => { 

                println!("hey") 
            },
            0x78 => { 

                println!("hey") 
            },
            0x79 => { 

                println!("hey") 
            },
            0x7A => { 

                println!("hey") 
            },
            0x7B => { 

                println!("hey") 
            },
            0x7C => { 

                println!("hey") 
            },
            0x7D => { 

                println!("hey") 
            },
            0x7E => { 

                println!("hey") 
            },
            0x7F => { 

                println!("hey") 
            },
            0x80 => { 

                println!("hey") 
            },
            0x81 => { 

                println!("hey") 
            },
            0x82 => { 

                println!("hey") 
            },
            0x83 => { 

                println!("hey") 
            },
            0x84 => { 

                println!("hey") 
            },
            0x85 => { 

                println!("hey") 
            },
            0x86 => { 

                println!("hey") 
            },
            0x87 => { 

                println!("hey") 
            },
            0x88 => { 

                println!("hey") 
            },
            0x89 => { 

                println!("hey") 
            },
            0x8A => { 

                println!("hey") 
            },
            0x8B => { 

                println!("hey") 
            },
            0x8C => { 

                println!("hey") 
            },
            0x8D => { 

                println!("hey") 
            },
            0x8E => { 

                println!("hey") 
            },
            0x8F => { 

                println!("hey") 
            },
            0x90 => { 

                println!("hey") 
            },
            0x91 => { 

                println!("hey") 
            },
            0x92 => { 

                println!("hey") 
            },
            0x93 => { 

                println!("hey") 
            },
            0x94 => { 

                println!("hey") 
            },
            0x95 => { 

                println!("hey") 
            },
            0x96 => { 

                println!("hey") 
            },
            0x97 => { 

                println!("hey") 
            },
            0x98 => { 

                println!("hey") 
            },
            0x99 => { 

                println!("hey") 
            },
            0x9A => { 

                println!("hey") 
            },
            0x9B => { 

                println!("hey") 
            },
            0x9C => { 

                println!("hey") 
            },
            0x9D => { 

                println!("hey") 
            },
            0x9E => { 

                println!("hey") 
            },
            0x9F => { 

                println!("hey") 
            },
            0xA0 => { 

                println!("hey") 
            },
            0xA1 => { 

                println!("hey") 
            },
            0xA2 => { 

                println!("hey") 
            },
            0xA3 => { 

                println!("hey") 
            },
            0xA4 => { 

                println!("hey") 
            },
            0xA5 => { 

                println!("hey") 
            },
            0xA6 => { 

                println!("hey") 
            },
            0xA7 => { 

                println!("hey") 
            },
            0xA8 => { 

                println!("hey") 
            },
            0xA9 => { 

                println!("hey") 
            },
            0xAA => { 

                println!("hey") 
            },
            0xAB => { 

                println!("hey") 
            },
            0xAC => { 

                println!("hey") 
            },
            0xAD => { 

                println!("hey") 
            },
            0xAE => { 

                println!("hey") 
            },
            0xAF => { 

                println!("hey") 
            },
            0xB0 => { 

                println!("hey") 
            },
            0xB1 => { 

                println!("hey") 
            },
            0xB2 => { 

                println!("hey") 
            },
            0xB3 => { 

                println!("hey") 
            },
            0xB4 => { 

                println!("hey") 
            },
            0xB5 => { 

                println!("hey") 
            },
            0xB6 => { 

                println!("hey") 
            },
            0xB7 => { 

                println!("hey") 
            },
            0xB8 => { 

                println!("hey") 
            },
            0xB9 => { 

                println!("hey") 
            },
            0xBA => { 

                println!("hey") 
            },
            0xBB => { 

                println!("hey") 
            },
            0xBC => { 

                println!("hey") 
            },
            0xBD => { 

                println!("hey") 
            },
            0xBE => { 

                println!("hey") 
            },
            0xBF => { 

                println!("hey") 
            },
            0xC0 => { 

                println!("hey") 
            },
            0xC1 => { 

                println!("hey") 
            },
            0xC2 => { 

                println!("hey") 
            },
            0xC3 => { 

                println!("hey") 
            },
            0xC4 => { 

                println!("hey") 
            },
            0xC5 => { 

                println!("hey") 
            },
            0xC6 => { 

                println!("hey") 
            },
            0xC7 => { 

                println!("hey") 
            },
            0xC8 => { 

                println!("hey") 
            },
            0xC9 => { 

                println!("hey") 
            },
            0xCA => { 

                println!("hey") 
            },
            0xCB => { 

                println!("hey") 
            },
            0xCC => { 

                println!("hey") 
            },
            0xCD => { 

                println!("hey") 
            },
            0xCE => { 

                println!("hey") 
            },
            0xCF => { 

                println!("hey") 
            },
            0xD0 => { 

                println!("hey") 
            },
            0xD1 => { 

                println!("hey") 
            },
            0xD2 => { 

                println!("hey") 
            },
            0xD3 => { 

                println!("hey") 
            },
            0xD4 => { 

                println!("hey") 
            },
            0xD5 => { 

                println!("hey") 
            },
            0xD6 => { 

                println!("hey") 
            },
            0xD7 => { 

                println!("hey") 
            },
            0xD8 => { 

                println!("hey") 
            },
            0xD9 => { 

                println!("hey") 
            },
            0xDA => { 

                println!("hey") 
            },
            0xDB => { 

                println!("hey") 
            },
            0xDC => { 

                println!("hey") 
            },
            0xDD => { 

                println!("hey") 
            },
            0xDE => { 

                println!("hey") 
            },
            0xDF => { 

                println!("hey") 
            },
            0xE0 => { 

                println!("hey") 
            },
            0xE1 => { 

                println!("hey") 
            },
            0xE2 => { 

                println!("hey") 
            },
            0xE3 => { 

                println!("hey") 
            },
            0xE4 => { 

                println!("hey") 
            },
            0xE5 => { 

                println!("hey") 
            },
            0xE6 => { 

                println!("hey") 
            },
            0xE7 => { 

                println!("hey") 
            },
            0xE8 => { 

                println!("hey") 
            },
            0xE9 => { 

                println!("hey") 
            },
            0xEA => { 

                println!("hey") 
            },
            0xEB => { 

                println!("hey") 
            },
            0xEC => { 

                println!("hey") 
            },
            0xED => { 

                println!("hey") 
            },
            0xEE => { 

                println!("hey") 
            },
            0xEF => { 

                println!("hey") 
            },
            0xF0 => { 

                println!("hey") 
            },
            0xF1 => { 

                println!("hey") 
            },
            0xF2 => { 

                println!("hey") 
            },
            0xF3 => { 

                println!("hey") 
            },
            0xF4 => { 

                println!("hey") 
            },
            0xF5 => { 

                println!("hey") 
            },
            0xF6 => { 

                println!("hey") 
            },
            0xF7 => { 

                println!("hey") 
            },
            0xF8 => { 

                println!("hey") 
            },
            0xF9 => { 

                println!("hey") 
            },
            0xFA => { 

                println!("hey") 
            },
            0xFB => { 

                println!("hey") 
            },
            0xFC => { 

                println!("hey") 
            },
            0xFD => { 

                println!("hey") 
            },
            0xFE => { 

                println!("hey") 
            },
            0xFF => { 

                println!("hey") 
            },
            _=>println!("rien")
        }
    }


}


fn main() {
    let mut cpu:CPU = CPU::init();

    cpu.set_a(0x1);

    cpu.get_a();

    cpu.execute(0x00);
    cpu.get_a();

    let x = (0x00ff<<8) + 0xee; //0x0ff (255) (0000 1111 1111) << 1 = 0x01fe (510) (0001 1111 1110)

}
