#![feature(mixed_integer_ops)]

use log::info;
use simple_logger::SimpleLogger;

mod cpu;
mod bus;
mod memory;

use bus::Bus;
use memory::{ROM, write_rom};
use cpu::cpu::CPU;


// https://web.archive.org/web/20210803073202/http://www.obelisk.me.uk/6502/architecture.html
// Zero page: 0x0000 - 0x00FF : is the focus of a number of special addressing modes that result in shorter (and quicker) instructions or allow indirect access to the memory (256 bytes of memory)
// Stack: 	  0x0100 - 0x01FF : is reserved for the system stack and which cannot be relocated. (256 bytes of stack!)
// Reserved memory: 0xFFFA - 0xFFFF (last 6 bytes) : must be programmed with the addresses of the non-maskable interrupt handler ($FFFA/B), the power on reset location ($FFFC/D) and the BRK/interrupt request handler ($FFFE/F) respectively.



/// This will populate the ROM with TOLOWER subroutine. \
/// The TOLOWER routine is described here: https://en.wikipedia.org/wiki/MOS_Technology_6502#Registers \
/// "which copies a null-terminated character string from one location to another, converting upper-case letter characters to lower-case letters."
/// Returns the amount of assembly lines / code written in ROM.
fn program_tolower(rom_memory: &mut [u8;65_536]) -> u8 {
	rom_memory[0] 	= 0xA0; //LDY #$00
	rom_memory[1] 	= 0x00;
	rom_memory[2] 	= 0xB1;	//LDA (SRC),Y
	rom_memory[3] 	= 0x80;
	rom_memory[4] 	= 0xF0;	//BEQ DONE
	rom_memory[5] 	= 0x11;
	rom_memory[6] 	= 0xC9;	//CMP #'A'
	rom_memory[7] 	= 0x41;
	rom_memory[8] 	= 0x90;	//BCC SKIP
	rom_memory[9] 	= 0x06;
	rom_memory[10] 	= 0xC9;	//CMP #'Z'+1
	rom_memory[11] 	= 0x58;
	rom_memory[12] 	= 0xB0; //BCS SKIP
	rom_memory[13] 	= 0x02;
	rom_memory[14] 	= 0x09; //ORA #%00100000
	rom_memory[15] 	= 0x20;
	rom_memory[16] 	= 0x91; //STA (DST),Y
	rom_memory[17] 	= 0x82;
	rom_memory[18] 	= 0xC8; //INY
	rom_memory[19] 	= 0xD0; //BNE LOOP
	rom_memory[20] 	= 0xED;
	rom_memory[21] 	= 0x38; //SEC
	rom_memory[22] 	= 0x60; //RTS
	rom_memory[23] 	= 0x91; //STA (DST),Y
	rom_memory[24] 	= 0x82;
	rom_memory[25] 	= 0x18; //CLC
	rom_memory[26] 	= 0x60; //RTS

	16
}

/// Program is taken from the first example here: https://skilldrick.github.io/easy6502/#first-program
/// Returns the amount of assembly lines / code written in ROM.
fn program_helloworld(rom_memory: &mut [u8;65_536]) -> u8 {
	// LDA #$01
	// STA $0200
	// LDA #$05
	// STA $0201
	// LDA #$08
	// STA $0202

	write_rom(rom_memory, "a9 01 8d 00 02 a9 05 8d 01 02 a9 08 8d 02 02");
	6
}

fn load_program_stack_operations(rom: &mut [u8;65_536]) -> u8 {
	// Push "8c ab" onto stack and get it
	/*
		LDA #$8C
		PHA
		LDA #$AB
		PHA
		PLA
		PLA
		NOP
	*/
	write_rom(rom, "A9 8C 48 A9 AB 48 68 68 EA");
	7
}

fn load_program_ADC(rom: &mut [u8;65_536]) -> u8 {
	/*
	CLD
	LDA #$09
	CLC
	ADC #$02 	; A will be 0x0B, as expected (0x9 + 0x2 = 0xB)

	SED
	LDA #$09
	CLC
	ADC #$02 	; A will be 0x11, because decimal bitflag is enabled (i.e. represent the sum in 'decimal' (11) form, not hex (0xB) form)
	*/
	write_rom(rom, "D8 A9 09 18 69 02 F8 A9 09 18 69 02");
	8
}

fn main() {
	SimpleLogger::new().init().unwrap();

	let mut rom_memory: [u8; 65_536] = [0;65_536];

	let assembly_lines_amount = load_program_ADC(&mut rom_memory);
	
	let rom: ROM = ROM {
		rom: Box::new(rom_memory)
	};
	
	let bus = Box::new(Bus::new(rom));
	let mut cpu = CPU::new(bus);

	for _ in 0..assembly_lines_amount {
		cpu.clock_tick();
	}

	info!("Finished running NES");
}

// #[cfg(test)]
// mod tests {
//     use super::*;

// 	fn load_test_program(rom: &mut [u8;65_536]) {
// 		/*
// 			LDA #$8C
// 			PHA
// 			NOP
// 		*/
// 		write_rom(rom, "A9 8C 48 EA");
// 	}

//     #[test]
// 	fn test_cpu_component() {
// 		SimpleLogger::new().init().unwrap();

// 		let mut rom_memory: [u8; 65_536] = [0;65_536];
// 		load_test_program(&mut rom_memory);
		
// 		let rom: ROM = ROM {
// 			rom: Box::new(rom_memory)
// 		};

// 		let mut bus = Box::new(Bus::new(rom));
// 		let mut cpu = CPU::new(bus);

// 		for _ in 0..3 {
// 			cpu.clock_tick();
// 		}
		
// 		let x = bus.ram.read(0x1FF);
// 		assert_eq!(x, 0x8C);
// 	}
// }