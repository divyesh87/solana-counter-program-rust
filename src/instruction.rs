use solana_program::program_error::ProgramError;

pub enum UserInstruction {
    Increment,
    Decrement,
    Set(u32),
}

impl UserInstruction {
    pub fn unpack(data: &[u8]) -> Result<Self, ProgramError> {
        let (&operation, rest) = data
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match operation {
            0 => Ok(UserInstruction::Increment),
            1 => Ok(UserInstruction::Decrement),
            2 => {
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                };

                let val: Result<[u8; 4], _> = rest.try_into();

                match val {
                    Ok(bytes) => Ok(UserInstruction::Set(u32::from_le_bytes(bytes))),
                    _ => Err(ProgramError::InvalidInstructionData),
                }
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
