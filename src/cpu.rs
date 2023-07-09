pub struct CPU {
	program_ctr: u16,
	stack_ptr: u8,
	accumulater: u8,
	index_reg_x: u8,
	index_reg_y: u8,
	proc_status: u8,
	memory: [u8; 0xFFFF]
}

pub enum AddressingMode {
	Absolute,
	AbsoluteX,
	AbsoluteY,
	ZeroPage,
	ZeroPageX,
	ZeroPageY,
	Immediate,
	Relative,
	Implicit,
	IndirectAddressing,
	IndexedIndirect,
	IndirectIndexed
}

impl CPU {
	/// Gets the address specified by an assembly instruction. The `program_ctr`
	/// parameter is assumed to be on the instruction.
	fn get_op_operand(&self, program_ctr: u16, addr_mode: AddressingMode) -> u16 {
		match addr_mode {
			AddressingMode::Absolute => self.read_u16_addr_in_mem(program_ctr + 1),
			AddressingMode::AbsoluteX => self.read_u16_addr_in_mem(program_ctr + 1).wrapping_add(self.index_reg_x as u16),
			AddressingMode::AbsoluteY => self.read_u16_addr_in_mem(program_ctr + 1).wrapping_add(self.index_reg_y as u16),
			AddressingMode::ZeroPage => self.read_mem(program_ctr + 1) as u16,
			AddressingMode::ZeroPageX => self.read_mem(program_ctr + 1).wrapping_add(self.index_reg_x) as u16,
			AddressingMode::ZeroPageY => self.read_mem(program_ctr + 1).wrapping_add(self.index_reg_y) as u16,
			AddressingMode::Immediate => program_ctr + 1,
			AddressingMode::Relative => (program_ctr + 1).wrapping_add_signed(self.read_mem(program_ctr + 1) as i16),
			AddressingMode::Implicit => panic!("no address is specified by an opcode using an implicit addressing mode"),
			AddressingMode::IndirectAddressing => {
				let addr = self.read_u16_addr_in_mem(program_ctr + 1);
				self.read_u16_addr_in_mem(addr)
			},
			AddressingMode::IndexedIndirect => {
				let addr = self.read_mem(program_ctr + 1).wrapping_add(self.index_reg_x) as u16;
				self.read_u16_addr_in_mem(addr)
			}
			AddressingMode::IndirectIndexed => {
				let addr = self.read_mem(program_ctr + 1) as u16;
				self.read_u16_addr_in_mem(addr).wrapping_add(self.index_reg_y as u16)
			}
		}
	}

	fn read_mem(&self, addr: u16) -> u8 {
		self.memory[addr as usize]
	}

	fn write_mem(&mut self, addr: u16, data: u8) {
		self.memory[addr as usize] = data;
	}

	/// Gets the big-endian version of a 16-bit memory address stored in
	/// memory at `addr`. The NES CPU is little-endian, so memory addresses
	/// would have their lowest significant bits stored first before the
	/// most significant bits.
	fn read_u16_addr_in_mem(&self, addr: u16) -> u16 {
		let lsb: u8 = self.read_mem(addr);
		let msb: u8 = self.read_mem(addr.wrapping_add(1));

		((msb as u16) << 8) | (lsb as u16)
	}
}
