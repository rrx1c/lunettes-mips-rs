//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs

pub mod registers;
use super::instruction::LmCoprocessor;
use super::operands::registers::*;
use super::instruction;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LmOperandType{
    Imm, Reg
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct LmOpRegister{
    register: &'static str,
    coprocessor: LmCoprocessor,
}
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct LmOpImmediate{
    value: u64,
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum LmOperand{
    LmOpRegister(LmOpRegister), LmOpImmediate(LmOpImmediate)
}

impl LmOpImmediate{
    pub fn new_imm_opreand(value: u64) -> LmOperand{
        LmOperand::LmOpImmediate(LmOpImmediate{
            value,
        })
    }
    pub fn get_value(&self) -> u64{
        self.value
    }
}
impl LmOpRegister{
    fn get_reg_str(register: u8, coprocessor: instruction::LmCoprocessor) -> &'static str{
        static CPU_REGISTER_TABLE: [&str; 32] = [
            LM_REG_ZERO, LM_REG_AT, LM_REG_V0, LM_REG_V1, LM_REG_A0, LM_REG_A1, LM_REG_A2, LM_REG_A3,
            LM_REG_T0, LM_REG_T1, LM_REG_T2, LM_REG_T3, LM_REG_T4, LM_REG_T5, LM_REG_T6, LM_REG_T7,
            LM_REG_S0, LM_REG_S1, LM_REG_S2, LM_REG_S3, LM_REG_S4, LM_REG_S5, LM_REG_S6, LM_REG_S7,
            LM_REG_T8, LM_REG_T9, LM_REG_K0, LM_REG_K1, LM_REG_GP, LM_REG_SP, LM_REG_FP, LM_REG_RA,
        ];
        static FPU_REGISTER_TABLE: [&str; 32] = [
            LM_REG_F0, LM_REG_F1, LM_REG_F2, LM_REG_F3, LM_REG_F4, LM_REG_F5, LM_REG_F6, LM_REG_F7,
            LM_REG_F8, LM_REG_F9, LM_REG_F10, LM_REG_F11, LM_REG_F12, LM_REG_F13, LM_REG_F14, LM_REG_F15,
            LM_REG_F16, LM_REG_F17, LM_REG_F18, LM_REG_F19, LM_REG_F20, LM_REG_F21, LM_REG_F22, LM_REG_F23,
            LM_REG_F24, LM_REG_F25, LM_REG_F26, LM_REG_F27, LM_REG_F28, LM_REG_F29, LM_REG_F30, LM_REG_F31,
        ];
        static LM_DEFAULT_REG_TABLE: [&str; 32] = [
            LM_REG_0, LM_REG_1, LM_REG_2, LM_REG_3, LM_REG_4, LM_REG_5, LM_REG_6, LM_REG_7,
            LM_REG_8, LM_REG_9, LM_REG_10, LM_REG_11, LM_REG_12, LM_REG_13, LM_REG_14, LM_REG_15,
            LM_REG_16, LM_REG_17, LM_REG_18, LM_REG_19, LM_REG_20, LM_REG_21, LM_REG_22, LM_REG_23,
            LM_REG_24, LM_REG_25, LM_REG_26, LM_REG_27, LM_REG_28, LM_REG_29, LM_REG_30, LM_REG_31,
        ];
        
        return match coprocessor{
            instruction::LmCoprocessor::Cp1 => FPU_REGISTER_TABLE[register as usize],
            instruction::LmCoprocessor::Cpu => CPU_REGISTER_TABLE[register as usize],
            _ => LM_DEFAULT_REG_TABLE[register as usize],
        }
    }
    pub fn new_reg_opreand(register: u8, coprocessor: LmCoprocessor) -> LmOperand{
        LmOperand::LmOpRegister(LmOpRegister{
            coprocessor,
            register: LmOpRegister::get_reg_str(register, coprocessor),
        })
    }
    pub fn new_reg_operand_str(reg_str: &'static str, coprocessor: instruction::LmCoprocessor) -> LmOperand{
        LmOperand::LmOpRegister(LmOpRegister{
            coprocessor,
            register: reg_str,
        })
    }
    pub fn get_register(&self) -> &'static str{
        self.register
    }
    pub fn get_coprocessor(&self) -> LmCoprocessor{
        self.coprocessor
    }

}