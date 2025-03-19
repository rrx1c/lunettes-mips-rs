use core::fmt;

pub enum LmErrorCode{
    FieldBadValue = 0x80000000, NoInstruction = 0x80000001, DevError = 0x80000002
}

pub struct LmError{
    address: u64,
    machine_code: u32,
    error_code: LmErrorCode,
}

impl LmError{
    pub fn throw_error(error_code: LmErrorCode, address: u64, machine_code: u32) -> LmError{
        LmError{error_code, address, machine_code}
    }
}

impl fmt::Display for LmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_code{
            LmErrorCode::FieldBadValue => write!(f, "[-]The field of this instruction has a bad value.\r\n\taddress: 0x{:08x}\r\n\tmachine code: 0x{:08x}", self.address, self.machine_code),
            LmErrorCode::NoInstruction =>write!(f, "[-]This machine code isn't an instruction.\r\n\taddress: 0x{:08x}\r\n\tmachine code: 0x{:08x}", self.address, self.machine_code),
            LmErrorCode::DevError =>write!(f, "[-]I did something wrong again.\r\n\taddress: {}\r\n\tmachine code: {}", self.address, self.machine_code),
        }
    }
}