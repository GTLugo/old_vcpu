use crate::error::InstructionError;

#[derive(Debug)]
pub enum Instruction {
  Null,
  SetLo { opcode: u16, register: u8, immediate: u64 },
  SetHi { opcode: u16, register: u8, immediate: u64 },
  Load { opcode: u16, register: u8, address: u64 },
  Store { opcode: u16, register: u8, address: u64 },
}

pub struct Fields {
  opcode: u16,
  operand_register_a: u8,
  operand_register_b: u8,
  target_register: u8,
  shift_amount: u8,
  immediate: u64,
}

impl Instruction {
  pub fn decode(instruction: u128) -> Result<Instruction, InstructionError> {
    Instruction::try_from(instruction)
  }

  fn extract_fields(instruction: u128) -> Fields {
    const SHIFT_OFFSET    : u32 = 64 + 16; // immediate + reserved
    const TARGET_OFFSET   : u32 = SHIFT_OFFSET + 8;
    const OPERAND_B_OFFSET: u32 = TARGET_OFFSET + 8;
    const OPERAND_A_OFFSET: u32 = OPERAND_B_OFFSET + 8;
    const OPCODE_OFFSET   : u32 = OPERAND_A_OFFSET + 8;

    let immediate         : u64 = instruction as u64;
    let shift_amount      : u8  = (instruction >> SHIFT_OFFSET) as u8;
    let target_register   : u8  = (instruction >> TARGET_OFFSET) as u8;
    let operand_register_b: u8  = (instruction >> OPERAND_B_OFFSET) as u8;
    let operand_register_a: u8  = (instruction >> OPERAND_A_OFFSET) as u8;
    let opcode            : u16 = (instruction >> OPCODE_OFFSET) as u16;

    Fields {
      opcode,
      operand_register_a,
      operand_register_b,
      target_register,
      shift_amount,
      immediate,
    }
  }
}

impl TryFrom<u128> for Instruction {
  type Error = InstructionError;

  fn try_from(instruction: u128) -> Result<Self, Self::Error> {
    let fields = Self::extract_fields(instruction);
    match fields.opcode {
      0x0000 => Ok(Self::Null),
      0xF010 => Ok(Self::SetLo {
        opcode: fields.opcode,
        register: fields.target_register,
        immediate: fields.immediate,
      }),
      0xF011 => Ok(Self::SetHi {
        opcode: fields.opcode,
        register: fields.target_register,
        immediate: fields.immediate,
      }),
      0xF013 => Ok(Self::Load {
        opcode: fields.opcode,
        register: fields.target_register,
        address: fields.immediate,
      }),
      0xF020 => Ok(Self::Store {
        opcode: fields.opcode,
        register: fields.target_register,
        address: fields.immediate,
      }),
      _ => Err(InstructionError::InvalidOpCode(fields.opcode))
    }
  }
}