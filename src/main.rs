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
    
            //7 	6 	5 	4 	3 	2 	1 	0
            //Z 	N 	H 	C 	0 	0 	0 	0
            
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

    fn up_z(&mut self){
        self.f = self.f | 0b1000_0000;
    }
    fn up_n(&mut self){
        self.f = self.f | 0b0100_0000;
    }
    fn up_h(&mut self){
        self.f = self.f | 0b0010_0000;
    }
    fn up_c(&mut self){
        self.f = self.f | 0b0001_0000;
    }


    fn is_z(&mut self) -> bool{
        return (self.f & 0b1000_0000) == 0b1000_0000;
    }
    fn is_n(&mut self) -> bool{
        return (self.f & 0b0100_0000) == 0b0100_0000;
    }
    fn is_h(&mut self) -> bool{
        return (self.f & 0b0010_0000) == 0b0010_0000;
    }
    fn is_c(&mut self) -> bool{
        return (self.f & 0b0001_0000) == 0b0001_0000;
    }

    fn down_z(&mut self){
        self.f = self.f & 0b0111_1111;
    }
    fn down_n(&mut self){
        self.f = self.f & 0b1011_1111;
    }
    fn down_h(&mut self){
        self.f = self.f & 0b1101_1111;
    }
    fn down_c(&mut self){
        self.f = self.f & 0b1110_1111;
    }



    fn rlc(&mut self,x:u8) -> u8{
        if (x & 0b1000_0000) == 0b1000_0000 {
            return (x<<1) + 1;
        } else {
            return x<<1;
        }
    }

    fn rrc(&mut self,x:u8) -> u8{
        if (x & 0b0000_0001) == 0b0000_0001 {
            return (x>>1) + 0b1000_0000;
        } else {
            return x>>1;
        }
    }

    fn rl(&mut self,x:u8) -> u8{
        let mut c = 0; 

        if self.f & 0b0001_0000 == 0b0001_0000 {
            c = 1;
        }

        if (x & 0b1000_0000) == 0b1000_0000 {
            self.f = self.f | 0b0001_0000;
        }

        return (x<<1) + c;
    }

    fn rr(&mut self,x:u8) -> u8{
        let mut c = 0; 

        if self.f & 0b0001_0000 == 0b0001_0000 {
            c = 0b1000_0000;
        }

        if (x & 0b0000_0001) == 0b0000_0001 {
            self.f = self.f | 0b0001_0000;
        }
        return (x>>1) + c ;

    }

    fn sll(&mut self,x:u8) -> u8{
        if (x & 0b1000_0000) == 0b1000_0000 {
            self.f = self.f | 0b0001_0000;
        }

        return (x<<1) + 1;
    }

    fn srl(&mut self,x:u8) -> u8{
        if (x & 0b0000_0001) == 0b0000_0001 {
            self.f = self.f | 0b0001_0000;
        }
        return (x>>1) + 0b1000_0000 ;

    }

    

    fn print_reg(&mut self){

        print!("Registers\n
        A:{:#04x} F:{:#04x} \n
        B:{:#04x} C:{:#04x} \n
        D:{:#04x} E:{:#04x} \n
        H:{:#04x} L:{:#04x} \n",
        self.a,self.f,
        self.b,self.c,
        self.d,self.e,
        self.h,self.l)

    }

    fn halfcarry(checked:u8,add:u8) -> bool{

        let low4bit = (checked & 0b0000_1111) + (add & 0b0000_1111);
        return low4bit & 0b1111_0000 != 0;
    }

    fn carry(checked:u8,add:u8) -> bool{
        match checked.checked_add(add){
            Some(v) =>{
                return false;
            }
            None => {
                return true;
            }
        }
    }

    fn pop(&mut self) -> u8 {
        let ret = self.read(self.sp as usize);
        self.sp += 1;
        return ret;
    }

    fn push(&mut self,value: u8){
        self.sp -= 1;
        self.write(self.sp as usize, value);
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

    fn get_HLadr(&mut self) -> u8{
        let hl:u16 = ((self.h as u16) << 4) + (self.l as u16);
        return self.read(hl as usize);
    }

    fn get_BC(&mut self) -> u16{
        let hl:u16 = ((self.b as u16) << 4) + (self.c as u16);
        return hl;
    }
    fn get_DE(&mut self) -> u16{
        let hl:u16 = ((self.d as u16) << 4) + (self.e as u16);
        return hl;
    }

    fn get_HL(&mut self) -> u16{
        let hl:u16 = ((self.h as u16) << 4) + (self.l as u16);
        return hl;
    }

    fn get_AF(&mut self) -> u16{
        let hl:u16 = ((self.a as u16) << 4) + (self.f as u16);
        return hl;
    }



    fn execute_CB(&mut self){
        let opcode = self.read(self.pc as usize);
        self.pc +=1;
        match opcode{
            0x00=>{
                //RLC B
                self.b = self.rlc(self.b);
            },
            0x01=>{
                //RLC C
                self.c = self.rlc(self.c);
            },    
            0x02 => {
                //RLC D
                self.d = self.rlc(self.d);
                
                },
            0x03 => { 
                //RLC E
                self.e = self.rlc(self.e);
            },
            0x04 => {
                //RLC H
                self.h = self.rlc(self.h);
   
            },
            0x05 => {
                //RLC L
                self.l = self.rlc(self.l);
            },
            0x06 => { 
                //RLC (HL)
                //TODO

            },
            0x07 => { 
                //RLC A
                self.a = self.rlc(self.a);

            },
            0x08 => { 
                //RRC B
                self.b = self.rrc(self.b);
            },
            0x09 => { 
                //RRC C
                self.c = self.rrc(self.c);
                 
            },
            0x0A => { 
                //RRC D
                self.d = self.rrc(self.d);
            },
            0x0B => { 
                //RRC E
                self.e = self.rrc(self.e);

            },
            0x0C => { 
                //RRC H
                self.h = self.rrc(self.h);

            },
            0x0D => { 
                //RRC L
                self.l = self.rrc(self.l);

            },
            0x0E => { 
                //RRC (HL)
                //todo
            },
            0x0F => { 
                //RRC A
                self.a = self.rrc(self.a);
            },
            0x10 => {
                //RL B
                self.b = self.rl(self.b);
            },
            0x11 => { 
                //RL C
                self.c = self.rl(self.c);
            },
            0x12 => {
                //RL D
                self.d = self.rl(self.d); 
            },
            0x13 => {                 
                //RL E
                self.e = self.rl(self.e);
            },
            0x14 => {                 
                //RL H
                self.h = self.rl(self.h);
            },
            0x15 => {                
                //RL L
                self.l = self.rl(self.l);
            },
            0x16 => { 
                //RL (HL)
                //TO DO
            },
            0x17 => { 
                //RL A
                self.a = self.rl(self.a); 
            },
            0x18 => { 
                //RR B
                self.b = self.rr(self.b); 
            },
            0x19 => { 
                //RR C
                self.c = self.rr(self.c); 
            },
            0x1A => { 
                //RR D
                self.d = self.rr(self.d); 
            },
            0x1B => { 
                //RR E
                self.e = self.rr(self.e); 
            },
            0x1C => { 
                //RR H
                self.h = self.rr(self.h); 
            },
            0x1D => { 
                //RR L
                self.l = self.rr(self.l); 
            },
            0x1E => { 
                //RR (HL)
                //todo
            },
            0x1F => { 
                //RR A
                self.a = self.rr(self.a); 
            },
            0x20 => { 

                println!("hey") 
            },
            0x21 => { 

                println!("hey") 
            },
            0x22 => { 
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
            },
            0x41 => {
            },
            0x42 => {
            },
            0x43 => { 
            },
            0x44 => { 
             },
            0x45 => { 
            },
            0x46 => { 
             },
            0x47 => { 
            },

            0x48 => { 
             },
            0x49 => { 
             },
            0x4A => { 
             },
            0x4B => { 
            },
            0x4C => { 
            },
            0x4D => { 
            },
            0x4E => { 
            },
            0x4F => { 
            },
            0x50 => { 
            },
            0x51 => { 
            },
            0x52 => { 
            },
            0x53 => { 
            },
            0x54 => { 
            },
            0x55 => { 
            },
            0x56 => { 
            },
            0x57 => { 
            },

            0x58 => { 
            },
            0x59 => { 
            },
            0x5A => { 
            },
            0x5B => { 
            },
            0x5C => { 
            },
            0x5D => { 
            },
            0x5E => {
            },
            0x5F => { 
            },
            0x60 => { 
            },
            0x61 => { 
            },
            0x62 => { 
            },
            0x63 => { 
            },
            0x64 => { 
            },
            0x65 => { 
            },
            0x66 => {
            },
            0x67 => { 
            },
            0x68 => { 
            },
            0x69 => { 
            },
            0x6A => { 
            },
            0x6B => { 
            },
            0x6C => { 
            },
            0x6D => { 
            },
            0x6E => {
            },
            0x6F => { 
            },
            0x70 => {  
            },
            0x71 => { 
            },
            0x72 => { 
            },
            0x73 => { 
            },
            0x74 => { 
            },
            0x75 => { 
            },
            0x76 => { 
            },
            0x77 => { 
            },
            0x78 => { 
            },
            0x79 => { 
            },
            0x7A => { 
            },
            0x7B => { 
            },
            0x7C => { 
            },
            0x7D => { 
            },
            0x7E => {
            },
            0x7F => { 
            },
            0x80 => { 
            },
            0x81 => { 
            },
            0x82 => {                
            },
            0x83 => { 
            },
            0x84 => { 
            },
            0x85 => {                         
            },
            0x86 => { 

            },
            0x87 => {                 
            },
            0x88 => { 
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
            },
            0xA1 => { 
            },
            0xA2 => { 
            },
            0xA3 => { 
            },
            0xA4 => { 
            },
            0xA5 => { 
            },
            0xA6 => { 
            },
            0xA7 => { 
            },


            0xA8 => { 
            },
            0xA9 => { 
            },
            0xAA => { 
            },
            0xAB => { 
            },
            0xAC => { 
            },
            0xAD => { 
            },
            0xAE => { 
            },
            0xAF => { 
            },

            0xB0 => { 
            },
            0xB1 => { 
            },
            0xB2 => { 
            },
            0xB3 => { 
            },
            0xB4 => { 
            },
            0xB5 => { 
            },
            0xB6 => { 
            },
            0xB7 => { 
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
            _=>{

            }
        }

    }

    fn execute(&mut self){
        let opcode = self.read(self.pc as usize);
        self.pc +=1;
        match opcode{
            0x00=>{
                //NOP
            },
            0x01=>{
                //LD BC,d16(nn)

                let nn:u16 = Self::cast_xy(self.read((self.pc+1) as usize),self.read((self.pc+2) as usize));
                // ex: want to cast NN (0xFE) to BC
                // B = NN>>4 (0XFE>>4 = 0xF)
                // C = (NN<<4) as u8)>>4  (0XFE<<4 = 0XFE0) as u8 => 0xE0 >>4 = 0xE
                self.b = (nn>>4) as u8;
                self.c = ((nn<<4) as u8)>>4;
                println!("hey")
            },    
            0x02 => {
                //LD (BC),A
                let bc:u16 = ((self.b as u16) << 4) + self.c as u16;
                self.write(bc as usize,self.a);
                },
            0x03 => { 
                println!("hey") 
            },
            0x04 => { 
                
                //INC B

                self.f = self.f & 0b1011_1111; //N down => AND to 0b1110_1111 so N gonna down
                
                if Self::halfcarry(self.b, 1) {
                    self.f = self.f |0b0010_0000; //H up
                }
 
                match self.b.checked_add(1) {
                    Some(v) => {
                        self.b = v;
                    }
                    None => {
                        self.b = 0x00;
                        self.f = self.f |0b1000_0000; //Z up
                    }
                };

            },

            // TO DO : CHECK ADD SUB OF EXCEPTION 
            0x05 => { 

                //DEC B
                self.f = self.f | 0b0100_0000; //N UP 

                match self.b.checked_sub(1) {
                    Some(v) => {
                        self.b = v;
                    }
                    None => {
                        self.b = 0x00;
                        self.f += 0b1000_0000; //Z up
                    }
                };            
            },
            0x06 => { 
                //LD B,d8
                self.pc +=1;
                self.b = self.read(self.pc as usize);
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
                //LD C,d8
                self.pc +=1;
                self.c = self.read(self.pc as usize);
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
                //LD (DE),A
                let de:u16 = ((self.d as u16) << 4) + self.e as u16;
                self.write(de as usize,self.a);
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
                //LD D,d8
                self.pc +=1;
                self.d = self.read(self.pc as usize);
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
                //LD E,d8
                self.pc +=1;
                self.e = self.read(self.pc as usize);
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
                //LD (HL++),A
                let hl:u16 = ((self.h as u16) << 4) + (self.l as u16) + 1;
                self.write(hl as usize,self.a);
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
                //LD H,d8
                self.pc +=1;
                self.h = self.read(self.pc as usize);
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
                //LD L,d8
                self.pc +=1;
                self.l = self.read(self.pc as usize);
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
                //LD (HL--),A
                let hl:u16 = ((self.h as u16) << 4) + (self.l as u16) - 1;
                self.write(hl as usize,self.a);
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
                //SCF
                self.down_n();
                self.down_h();
                self.up_c();
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
                //LD A,d8
                self.pc +=1;
                self.a = self.read(self.pc as usize);
            },
            0x3F => { 

                println!("hey") 
            },

           
            //	8bit load/store/move instructions
            // How to cast H(0xF) with L(0xE) TO HL(0xFE) 
            // h = 0xF (as u16) => 0x0F (<<4) => 0xF0 (+0x0E) => 0XFE
            // impl : let hl:u16 = ((self.h as u16) << 4) + self.l as u16;


            //B register
            0x40 => { 
                //LD B,B
                self.b = self.b;
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
                //LD B,(HL)
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.b = self.read(hl as usize);
             },
            0x47 => { 
                //LD B,A
                self.b = self.a; 
            },


            //C register
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
                //LD C,(HL)
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.c = self.read(hl as usize); 
            },
            0x4F => { 
                 //LD C,A
                 self.c = self.a; 
            },


            //D register
            0x50 => { 
                //LD D,B
                self.d = self.b; 
            },
            0x51 => { 
                //LD D,C
                self.d = self.c; 
            },
            0x52 => { 
                //LD D,D
                self.d = self.d;               
            },
            0x53 => { 
                //LD D,E
                self.d = self.e; 
            },
            0x54 => { 
                //LD D,H
                self.d = self.h;               
            },
            0x55 => { 
                //LD D,L
                self.d = self.l;               
            },
            0x56 => { 
                //LD D,(HL)
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.d = self.read(hl as usize); 
            },
            0x57 => { 
                //LD D,A
                self.d = self.a;               
            },


            //E register
            0x58 => { 
                //LD E,B
                self.e = self.b;               
            },
            0x59 => { 
                //LD E,C
                self.e = self.c;               
            },
            0x5A => { 
                //LD E,D
                self.e = self.d;               
            },
            0x5B => { 
                //LD E,E
                self.e = self.e;               
            },
            0x5C => { 
                //LD E,H
                self.e = self.h;               
            },
            0x5D => { 
                //LD E,L
                self.e = self.l;               
            },
            0x5E => {
                //LD E,(HL)
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.e = self.read(hl as usize); 
            },
            0x5F => { 
                //LD E,A
                self.e = self.a;               
            },


            //H register
            0x60 => { 
                //LD H,B
                self.h = self.b;               
            },
            0x61 => { 
                //LD H,C
                self.h = self.c;               
            },
            0x62 => { 
                //LD H,D
                self.h = self.d;               
            },
            0x63 => { 
                //LD H,A
                self.h = self.e;               
            },
            0x64 => { 
                //LD H,A
                self.h = self.h;               
            },
            0x65 => { 
                //LD H,A
                self.h = self.l;               
            },
            0x66 => {
                //LD H,(HL)
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.h = self.read(hl as usize); 
            },
            0x67 => { 
                //LD H,A
                self.h = self.a;               
            },


            //L register
            0x68 => { 
                //LD L,B
                self.l = self.b;               
            },
            0x69 => { 
                //LD L,C
                self.l = self.c;               
            },
            0x6A => { 
                //LD L,D
                self.l = self.d;               
            },
            0x6B => { 
                //LD L,E
                self.l = self.e;               
            },
            0x6C => { 
                //LD L,H
                self.l = self.h;               
            },
            0x6D => { 
                //LD L,L
                self.l = self.l;               
            },
            0x6E => {
                //LD L,(HL)
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.l = self.read(hl as usize); 
            },
            0x6F => { 
                //LD L,A
                self.l = self.a;               
            },


            //HL register
            0x70 => {  
                //LD (HL),B
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.write(hl as usize, self.b);
            },
            0x71 => { 
                //LD (HL),C
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.write(hl as usize, self.c);
            },
            0x72 => { 
                //LD (HL),D
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.write(hl as usize, self.d);
            },
            0x73 => { 
                //LD (HL),E
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.write(hl as usize, self.e);
            },
            0x74 => { 
                //LD (HL),H
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.write(hl as usize, self.h);
            },
            0x75 => { 
                //LD (HL),L
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.write(hl as usize, self.l);
            },
            0x76 => { 
                //HALT
                //STOP clock               
            },
            0x77 => { 
                //LD (HL),A
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.write(hl as usize, self.a);
            },


            //A register
            0x78 => { 
                //LD A,B
                self.a = self.b;               
            },
            0x79 => { 
                //LD A,C
                self.a = self.c;               
            },
            0x7A => { 
                //LD A,C
                self.a = self.d;               
            },
            0x7B => { 
                //LD A,E
                self.a = self.e;               
            },
            0x7C => { 
                //LD A,H
                self.a = self.h;               
            },
            0x7D => { 
                //LD A,L
                self.a = self.l;               
            },
            0x7E => {
                //LD A,(HL)
                let hl:u16 = ((self.h as u16) << 4) + self.l as u16;
                self.a = self.read(hl as usize); 
            },
            0x7F => { 
                //LD A,A
                self.a = self.a;               
            },



            //ADD
            0x80 => { 
                //ADD A,B
                
                self.f = self.f & 0b1011_1111; //N down => AND to 0b1110_1111 so N gonna down

                if Self::halfcarry(self.a, self.b) {
                    self.f = self.f |0b0010_0000; //H up
                }
                
                match self.a.checked_add(self.b) {
                    Some(v) => {
                        self.a = v;
                    }
                    None => {
                        self.a = 0x00;
                        self.f = self.f |0b1000_0000; //Z up
                        self.f = self.f |0b0001_0000; //C up

                    }
                };
            },
            0x81 => { 
                //ADD A,C

                self.f = self.f & 0b1011_1111; //N down => AND to 0b1110_1111 so N gonna down

                if Self::halfcarry(self.a, self.c) {
                    self.f = self.f |0b0010_0000; //H up
                }
                
                match self.a.checked_add(self.c) {
                    Some(v) => {
                        self.a = v;
                    }
                    None => {
                        self.a = 0x00;
                        self.f = self.f |0b1000_0000; //Z up
                        self.f = self.f |0b0001_0000; //C up

                    }
                };
            },
            0x82 => {                
                //ADD A,D

                self.f = self.f & 0b1011_1111; //N down => AND to 0b1110_1111 so N gonna down

                if Self::halfcarry(self.a, 1) {
                    self.f = self.f |0b0010_0000; //H up
                }
                
                match self.a.checked_add(self.d) {
                    Some(v) => {
                        self.a = v;
                    }
                    None => {
                        self.a = 0x00;
                        self.f = self.f |0b1000_0000; //Z up
                        self.f = self.f |0b0001_0000; //C up

                    }
                };
            },
            0x83 => { 
                //ADD A,E

                self.f = self.f & 0b1011_1111; //N down => AND to 0b1110_1111 so N gonna down

                if Self::halfcarry(self.a, 1) {
                    self.f = self.f |0b0010_0000; //H up
                }
                
                match self.a.checked_add(self.e) {
                    Some(v) => {
                        self.a = v;
                    }
                    None => {
                        self.a = 0x00;
                        self.f = self.f |0b1000_0000; //Z up
                        self.f = self.f |0b0001_0000; //C up

                    }
                };
            },
            0x84 => { 
                //ADD A,H

                self.f = self.f & 0b1011_1111; //N down => AND to 0b1110_1111 so N gonna down

                if Self::halfcarry(self.a, 1) {
                    self.f = self.f |0b0010_0000; //H up
                }
                
                match self.a.checked_add(self.h) {
                    Some(v) => {
                        self.a = v;
                    }
                    None => {
                        self.a = 0x00;
                        self.f = self.f |0b1000_0000; //Z up
                        self.f = self.f |0b0001_0000; //C up

                    }
                };
            },
            0x85 => { 
                //ADD A,L

                self.f = self.f & 0b1011_1111; //N down => AND to 0b1110_1111 so N gonna down

                if Self::halfcarry(self.a, 1) {
                    self.f = self.f |0b0010_0000; //H up
                }
                
                match self.a.checked_add(self.l) {
                    Some(v) => {
                        self.a = v;
                    }
                    None => {
                        self.a = 0x00;
                        self.f = self.f |0b1000_0000; //Z up
                        self.f = self.f |0b0001_0000; //C up

                    }
                };
                        
            },
            0x86 => { 
                //ADD A,(HL)

                self.f = self.f & 0b1011_1111; //N down => AND to 0b1110_1111 so N gonna down

                let hl:u8 = self.read((((self.h as u16) << 4) + self.l as u16) as usize);

                if Self::halfcarry(self.a, hl) {
                    self.f = self.f |0b0010_0000; //H up
                }
                
                match self.a.checked_add(hl) {
                    Some(v) => {
                        self.a = v;
                    }
                    None => {
                        self.a = 0x00;
                        self.f = self.f |0b1000_0000; //Z up
                        self.f = self.f |0b0001_0000; //C up

                    }
                };

            },
            0x87 => {                 
                //ADD A,A

                self.f = self.f & 0b1011_1111; //N down => AND to 0b1110_1111 so N gonna down

                if Self::halfcarry(self.a, self.a) {
                    self.f = self.f |0b0010_0000; //H up
                }
                
                match self.a.checked_add(self.a) {
                    Some(v) => {
                        self.a = v;
                    }
                    None => {
                        self.a = 0x00;
                        self.f = self.f |0b1000_0000; //Z up
                        self.f = self.f |0b0001_0000; //C up

                    }
                };
            },
            0x88 => { 
                //ADC A,B

                self.f = self.f & 0b1011_1111; //N down => AND to 0b1110_1111 so N gonna down

                if Self::halfcarry(self.a, self.a) {
                    self.f = self.f |0b0010_0000; //H up
                }
                
                match self.a.checked_add(self.b + (self.f & 0b0001_0000)) {
                    Some(v) => {
                        self.a = v;
                    }
                    None => {
                        self.a = 0x00;
                        self.f = self.f |0b1000_0000; //Z up
                        self.f = self.f |0b0001_0000; //C up

                    }
                };            },
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


            //AND

            0xA0 => { 
                //AND B
                self.a = self.a & self.b;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.up_h();
            },
            0xA1 => { 
                //AND C
                self.a = self.a & self.c;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.up_h();
            },
            0xA2 => { 
                //AND D
                self.a = self.a & self.d;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.up_h();
            },
            0xA3 => { 
                //AND E
                self.a = self.a & self.e;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.up_h();
            },
            0xA4 => { 
                //AND H
                self.a = self.a & self.h;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.up_h();
            },
            0xA5 => { 
                //AND L
                self.a = self.a & self.l;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.up_h();
            },
            0xA6 => { 
                //AND (HL)
                println!("hey") 
            },
            0xA7 => { 
                //AND A
                self.a = self.a & self.a;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.up_h();
            },


            //XOR

            0xA8 => { 
                //XOR B
                self.a = self.a ^ self.b;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xA9 => { 
                //XOR C
                self.a = self.a ^ self.c;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xAA => { 
                //XOR D
                self.a = self.a ^ self.d;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xAB => { 
                //XOR E
                self.a = self.a ^ self.e;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xAC => { 
                //XOR H
                self.a = self.a ^ self.h;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xAD => { 
                //XOR L
                self.a = self.a ^ self.l;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xAE => { 
                //XOR (HL)
            },
            0xAF => { 
                //XOR A
                self.a = self.a ^ self.a;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },


            //OR

            0xB0 => { 
                //OR B
                self.a = self.a | self.b;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xB1 => { 
                //OR C
                self.a = self.a | self.c;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xB2 => { 
                //OR D
                self.a = self.a | self.d;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xB3 => { 
                //OR E
                self.a = self.a | self.e;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xB4 => { 
                //OR H
                self.a = self.a | self.h;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xB5 => { 
                //OR L
                self.a = self.a | self.l;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
            },
            0xB6 => { 
                //OR (HL)
            },
            0xB7 => { 
                //OR A
                self.a = self.a | self.a;
                if self.a == 0 {
                    self.up_z()
                }
                self.down_c();
                self.down_n();
                self.down_h();
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
                //RET NZ
                if !self.is_z() {
                    self.pc = Self::cast_xy(self.pop(), self.pop());
                }
            },
            0xC1 => { 
                //POP BC
                self.c = self.read(self.sp as usize);
                self.sp += 1;

                self.b = self.read(self.sp as usize);
                self.sp += 1;

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
                //PUSH BC 
                self.sp -= 1;
                self.write(self.sp as usize, self.b);

                self.sp -= 1;
                self.write(self.sp as usize, self.c);
            },
            0xC6 => { 

                println!("hey") 
            },
            0xC7 => { 

                println!("hey") 
            },
            0xC8 => { 
                //RET Z
                if self.is_z() {
                    self.pc = Self::cast_xy(self.pop(), self.pop());
                }
            },
            0xC9 => { 
                //RET
                self.pc = Self::cast_xy(self.pop(), self.pop());
            },
            0xCA => { 

                println!("hey") 
            },
            0xCB => { 
                // ALL CB operators finito pipo
                self.execute_CB();
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
                //RET NC
                if !self.is_c() {
                    self.pc = Self::cast_xy(self.pop(), self.pop());
                }

            },
            0xD1 => { 
                //POP DE
                self.d = self.read(self.sp as usize);
                self.sp += 1;

                self.e = self.read(self.sp as usize);
                self.sp += 1;
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
                //PUSH DE 
                self.sp -= 1;
                self.write(self.sp as usize, self.d);

                self.sp -= 1;
                self.write(self.sp as usize, self.e);
            },
            0xD6 => { 

                println!("hey") 
            },
            0xD7 => { 

                println!("hey") 
            },
            0xD8 => { 
                //RET C
                if self.is_c() {
                    self.pc = Self::cast_xy(self.pop(), self.pop());
                }
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
                //POP HL
                self.h = self.read(self.sp as usize);
                self.sp += 1;

                self.l = self.read(self.sp as usize);
                self.sp += 1;
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
                //PUSH HL 
                self.sp -= 1;
                self.write(self.sp as usize, self.h);

                self.sp -= 1;
                self.write(self.sp as usize, self.l);
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
                //JP HL
                self.pc = self.get_HL();

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
                //POP AF
                self.a = self.read(self.sp as usize);
                self.sp += 1;

                self.f = self.read(self.sp as usize);
                self.sp += 1;
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
                //PUSH AF
                self.sp -= 1;
                self.write(self.sp as usize, self.a);

                self.sp -= 1;
                self.write(self.sp as usize, self.f);
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

    let mut x:u8 = 0b1010_0000; //0b0101_0101 0b1010_1010


 
    cpu.print_reg();
    cpu.write(0x00, 0x04);//write slot 1 of ram 0x04 opcode => INC B
    cpu.write(0x01, 0x04);//=> INC B
    cpu.write(0x02, 0x60);//=> LD H,B
    cpu.write(0x03, 0x78);//=> LD A,B
    cpu.write(0x04, 0x80);//=> ADD B



    cpu.execute();
    cpu.print_reg();

    
    cpu.execute();
    cpu.print_reg();

    cpu.execute();
    cpu.print_reg();

    cpu.execute();
    cpu.print_reg();


    cpu.execute();
    cpu.print_reg();


}
