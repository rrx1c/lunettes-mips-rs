//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs
use crate::mips::LmAddressSize;
use crate::mips::operands::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LmCpuInstructionFormat{
    NoFormat, Imm, Reg, Jump, Other
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LmInstructionFunction{
    NoFunction, Computational, BranchJump,
    LoadStore, Miscellaneous,
    Coprocessor
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LmCoprocessor{
    NoCoprocessor,
    CPU, CP0, CP1, CP2, CP1X
}

/*
    On peut s'en servir en tant qu'index dans l'array qui regroupe tous les mnemonics pour trouver le bon mnemonic,
    peut aussi servir pour reconnaître l'instruction sans avoir à comparer le mnemonics 
    avec une chaîne de caractère ce qui peut ralentir la recherche d'une instruction précise.
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LmMnemonicId {
    NOMNEMONIC, J, JAL, BEQ, BNE, BLEZ, BGTZ, ADDI, ADDIU, SLTI, SLTIU, ANDI,
    ORI, XORI, LUI, BEQL, BNEL, BLEZL, BGTZL, JALX, LB, LH, LWL, LW, LBU, LHU,
    LWR, SB, SH, SWL, SW, SWR, CACHE, LL, LWC1, LWC2, PREF, LDC1, LDC2, SC,
    SWC1, SWC2, SDC1, SDC2, SLL, SRA, SLLV, SRAV, JR, JRHB, JALR, JALRHB, MOVZ, MOVN,
    SYSCALL, BREAK, SYNC
}

#[derive(Clone, Debug, PartialEq)]
pub struct LmInstruction{
    address: u64,
    to_string_callack: fn(mnemonic_id: LmMnemonicId, operands: &Vec<LmOperand>) -> String,
    operands: Vec<LmOperand>,
    mnemonic_id: LmMnemonicId,
    function: LmInstructionFunction,
    format: LmCpuInstructionFormat,
    address_size: LmAddressSize,
    coprocessor: LmCoprocessor,
    machine_code: u32,
    is_conditional: bool,
    is_relative: bool,
    is_region: bool,
}

impl LmInstruction{
    pub fn new_instruction(to_string: fn(mnemonic_id: LmMnemonicId, operands: &Vec<LmOperand>) -> String, address_size: LmAddressSize, is_conditional: bool, coprocessor: LmCoprocessor, address: u64, operands: Vec<LmOperand>, mnemonic_id: LmMnemonicId, machine_code: u32, function: LmInstructionFunction, format: LmCpuInstructionFormat, relative: bool, region: bool) -> Option<LmInstruction>{
        if  mnemonic_id == LmMnemonicId::NOMNEMONIC ||
            format == LmCpuInstructionFormat::NoFormat ||
            function == LmInstructionFunction::NoFunction{
                println!("[-]new_instruction function: Something is missing or wrong");
                return None
        }
        return Some(LmInstruction{
            // operands: operands,
            address: address,
            to_string_callack: to_string,
            is_conditional: is_conditional,
            coprocessor: coprocessor,
            function: function,
            format: format,
            machine_code: machine_code,
            address_size: address_size,
            is_region: region,
            is_relative: relative,
            mnemonic_id: mnemonic_id,
            operands: operands
        })
    }
    pub fn get_memonic(mnemonic_id: LmMnemonicId) -> &'static str{
        static MNEMONIC_TABLE: [&str; 57] = [
            LM_MNE_NO_MNEMONIC, LM_MNE_J, LM_MNE_JAL, LM_MNE_BEQ, LM_MNE_BNE, LM_MNE_BLEZ, LM_MNE_BGTZ, LM_MNE_ADDI, 
            LM_MNE_ADDIU, LM_MNE_SLTI, LM_MNE_SLTIU, LM_MNE_ANDI, LM_MNE_ORI, LM_MNE_XORI, LM_MNE_LUI, LM_MNE_BEQL, 
            LM_MNE_BNEL, LM_MNE_BLEZL, LM_MNE_BGTZL, LM_MNE_JALX, LM_MNE_LB, LM_MNE_LH, LM_MNE_LWL, LM_MNE_LW, 
            LM_MNE_LBU, LM_MNE_LHU, LM_MNE_LWR, LM_MNE_SB, LM_MNE_SH, LM_MNE_SWL, LM_MNE_SW, LM_MNE_SWR, 
            LM_MNE_CACHE, LM_MNE_LL, LM_MNE_LWC1, LM_MNE_LWC2, LM_MNE_PREF, LM_MNE_LDC1, LM_MNE_LDC2, LM_MNE_SC, 
            LM_MNE_SWC1, LM_MNE_SWC2, LM_MNE_SDC1, LM_MNE_SDC2, LM_MNE_SLL, LM_MNE_SRA, LM_MNE_SLLV,LM_MNE_SRAV,
            LM_MNE_JR, LM_MNE_JRHB, LM_MNE_JALR, LM_MNE_JALRHB, LM_MNE_MOVZ, LM_MNE_MOVN, LM_MNE_SYSCALL, LM_MNE_BREAK,
            LM_MNE_SYNC
        ];
        MNEMONIC_TABLE[mnemonic_id as usize]
    }

    pub fn get_mnemonicid(&self) -> LmMnemonicId{
        self.mnemonic_id
    }
    pub fn _get_opcode(&self) -> u8{
        (self.machine_code >> 26) as u8
    }
    pub fn temp_to_string(&self) -> String{
        (self.to_string_callack)(self.mnemonic_id, &self.operands)
    }
    pub fn to_string(&self) -> String{
        match self.format {
            LmCpuInstructionFormat::Jump => {
                return format!("{} {}", LmInstruction::get_memonic(self.mnemonic_id), self.operands[0].imm_to_string().unwrap());
            },
            LmCpuInstructionFormat::Imm => {
                let base: &str;
                let rt: &str;

                match self.function {
                    LmInstructionFunction::LoadStore =>{
                        if self.mnemonic_id == LmMnemonicId::CACHE || self.mnemonic_id == LmMnemonicId::PREF  {
                            return format!("{} {}, {}({})", LmInstruction::get_memonic(self.mnemonic_id), self.operands[0].imm_to_string().unwrap(), self.operands[2].imm_to_string().unwrap(), LmOperand::reg_to_string(self.operands[1].get_register().unwrap(), LmCoprocessor::CPU))
                        }
                        return format!("{} {}, {}({})", LmInstruction::get_memonic(self.mnemonic_id), LmOperand::reg_to_string(self.operands[0].get_register().unwrap(), self.operands[0].get_coprocessor()), self.operands[2].imm_to_string().unwrap(), LmOperand::reg_to_string(self.operands[1].get_register().unwrap(), LmCoprocessor::CPU))
                    }
                    LmInstructionFunction::BranchJump =>{
                        if self.operands.len() == 2{
                            return format!("{} {}, {}", LmInstruction::get_memonic(self.mnemonic_id), LmOperand::reg_to_string(self.operands[0].get_register().unwrap(), LmCoprocessor::CPU), self.operands[1].imm_to_string().unwrap())
                        }
                        rt = LmOperand::reg_to_string(self.operands[1].get_register().unwrap(), LmCoprocessor::CPU);
                        base = LmOperand::reg_to_string(self.operands[0].get_register().unwrap(), LmCoprocessor::CPU);
                        return format!("{} {}, {}, {}", LmInstruction::get_memonic(self.mnemonic_id), base, rt, self.operands[2].imm_to_string().unwrap())
                    }
                    _ => {
                        if self.operands.len() == 2{
                            return format!("{} {}, {}", LmInstruction::get_memonic(self.mnemonic_id), LmOperand::reg_to_string(self.operands[0].get_register().unwrap(), LmCoprocessor::CPU), self.operands[1].imm_to_string().unwrap());
                        }

                        base = LmOperand::reg_to_string(self.operands[1].get_register().unwrap(), LmCoprocessor::CPU);
                        rt = LmOperand::reg_to_string(self.operands[0].get_register().unwrap(), LmCoprocessor::CPU);

                        return format!("{} {}, {}, {}", LmInstruction::get_memonic(self.mnemonic_id), rt, base, self.operands[2].imm_to_string().unwrap());
                    }
                }
            }
            LmCpuInstructionFormat::Reg =>{
                if self.operands.len() == 0{
                    return format!("{}", LmInstruction::get_memonic(self.mnemonic_id));
                }
                else{
                    if self.operands[2].get_operand_type() == LmOperandType::IMM{
                        return format!("{} {}, {}, {}", LmInstruction::get_memonic(self.mnemonic_id), LmOperand::reg_to_string(self.operands[0].get_register().unwrap(), self.operands[0].get_coprocessor()), LmOperand::reg_to_string(self.operands[1].get_register().unwrap(), self.operands[1].get_coprocessor()), self.operands[2].imm_to_string().unwrap())
                    }
                    else{
                        return format!("{} {}, {}, {}", LmInstruction::get_memonic(self.mnemonic_id), LmOperand::reg_to_string(self.operands[0].get_register().unwrap(), self.operands[0].get_coprocessor()), LmOperand::reg_to_string(self.operands[1].get_register().unwrap(), self.operands[1].get_coprocessor()), LmOperand::reg_to_string(self.operands[2].get_register().unwrap(), self.operands[2].get_coprocessor()))
                    }
                }
            },
            _ => return String::from("[-]Print instruction as string: instruction format not implemented yet"),
        };
    }
    pub fn get_address(&self) -> u64{
        self.address
    }
    pub fn _get_format(&self) -> LmCpuInstructionFormat{
        self.format
    }
    pub fn _get_address_size(&self) -> LmAddressSize{
        return self.address_size
    }
    pub fn _get_function(&self) -> LmInstructionFunction{
        self.function
    }
    pub fn _get_imm_imm(&self)-> Option<u16>{
        if self.format != LmCpuInstructionFormat::Imm{
            return None
        }
        return Some((self.machine_code & 0xffff) as u16)
    }
    pub fn _get_imm_rt(&self)-> Option<u8>{
        if self.format != LmCpuInstructionFormat::Imm{
            return None
        }
        return Some((self.machine_code  >> 16 & 0b11111) as u8)
    }
    pub fn _get_imm_rs(&self)-> Option<u8>{
        if self.format != LmCpuInstructionFormat::Imm{
            return None
        }
        return Some((self.machine_code >> 21 & 0b11111) as u8)
    }
    pub fn _get_jump_instr_index(&self) -> Option<u32>{
        if self.format != LmCpuInstructionFormat::Jump{
            return None
        }
        Some((self.machine_code & 26) as u32)
    }
    pub fn _is_relative(&self) -> bool{
        self.is_relative
    }
    pub fn _is_region(&self) -> bool{
        self.is_region
    }
    pub fn _get_operand_num(&self) -> usize{
        self.operands.len()
    }
    pub fn _get_machine_code(&self) -> u32{
        self.machine_code
    }
    pub fn _get_coprocessor(&self) -> LmCoprocessor{
        return match self._get_opcode(){
            0x20 => LmCoprocessor::CP0,
            0x21 => LmCoprocessor::CP1,
            0x22 => LmCoprocessor::CP2,
            0x23 => LmCoprocessor::CP1X,
            _ => LmCoprocessor::CPU,
        }
    }
}

pub fn jr_jalr(mnemonic_id: LmMnemonicId, operands: &Vec<LmOperand>) ->String{
    if operands.len() == 1{
        return format!("{} {}", LmInstruction::get_memonic(mnemonic_id), LmOperand::reg_to_string(operands[0].get_register().unwrap(), operands[0].get_coprocessor()))
    }
    else{
        return format!("{} {}, {}", LmInstruction::get_memonic(mnemonic_id), LmOperand::reg_to_string(operands[0].get_register().unwrap(), operands[0].get_coprocessor()), LmOperand::reg_to_string(operands[1].get_register().unwrap(), operands[1].get_coprocessor()))
    }
}
pub fn one_operand_to_string(mnemonic_id: LmMnemonicId, operands: &Vec<LmOperand>) ->String{
    if operands[0].get_operand_type() == LmOperandType::REG{
        return format!("{} {}", LmInstruction::get_memonic(mnemonic_id), LmOperand::reg_to_string(operands[0].get_register().unwrap(), operands[0].get_coprocessor()))
    }
    else{
        return format!("{} {}", LmInstruction::get_memonic(mnemonic_id), operands[0].imm_to_string().unwrap())
    }
}
pub fn no_operand_to_string(mnemonic_id: LmMnemonicId, _operands: &Vec<LmOperand>) ->String{
    return format!("{}", LmInstruction::get_memonic(mnemonic_id))
}

pub const LM_MNE_NO_MNEMONIC: &str = "error"; pub const LM_MNE_J: &str = "j"; pub const LM_MNE_JAL: &str = "jal";
pub const LM_MNE_BEQ: &str = "beq"; pub const LM_MNE_BNE: &str = "bne"; pub const LM_MNE_BLEZ: &str = "blez";
pub const LM_MNE_BGTZ: &str = "bgtz"; pub const LM_MNE_ADDI: &str = "addi"; pub const LM_MNE_ADDIU: &str = "addiu";
pub const LM_MNE_SLTI: &str = "slti"; pub const LM_MNE_SLTIU: &str = "sltiu"; pub const LM_MNE_ANDI: &str = "andi";
pub const LM_MNE_ORI: &str = "ori"; pub const LM_MNE_XORI: &str = "xori"; pub const LM_MNE_LUI: &str = "lui";
pub const LM_MNE_BEQL: &str = "beql"; pub const LM_MNE_BNEL: &str = "bnel"; pub const LM_MNE_BLEZL: &str = "blezl";
pub const LM_MNE_BGTZL: &str = "bgtzl"; pub const LM_MNE_JALX: &str = "jalx"; pub const LM_MNE_LB: &str = "lb";
pub const LM_MNE_LH: &str = "lh"; pub const LM_MNE_LWL: &str = "lwl"; pub const LM_MNE_LW: &str = "lw";
pub const LM_MNE_LBU: &str = "lbu"; pub const LM_MNE_LHU: &str = "lhu"; pub const LM_MNE_LWR: &str = "lwr";
pub const LM_MNE_SB: &str = "sb"; pub const LM_MNE_SH: &str = "sh"; pub const LM_MNE_SWL: &str = "swl";
pub const LM_MNE_SW: &str = "sw"; pub const LM_MNE_SWR: &str = "swr"; pub const LM_MNE_CACHE: &str = "cache";
pub const LM_MNE_LL: &str = "ll"; pub const LM_MNE_LWC1: &str = "lwc1"; pub const LM_MNE_LWC2: &str = "lwc2";
pub const LM_MNE_PREF: &str = "pref"; pub const LM_MNE_LDC1: &str = "ldc1"; pub const LM_MNE_LDC2: &str = "ldc2";
pub const LM_MNE_SC: &str = "sc"; pub const LM_MNE_SWC1: &str = "swc1"; pub const LM_MNE_SWC2: &str = "swc2";
pub const LM_MNE_SDC1: &str = "sdc1"; pub const LM_MNE_SDC2: &str = "sdc2"; pub const LM_MNE_SLL: &str = "sll";
pub const LM_MNE_SRA: &str = "sra"; pub const LM_MNE_SLLV: &str = "sllv"; pub const LM_MNE_SRAV: &str = "srav";
pub const LM_MNE_JR: &str = "jr"; pub const LM_MNE_JRHB: &str = "jr.hb"; pub const LM_MNE_JALR: &str = "jalr"; 
pub const LM_MNE_JALRHB: &str = "jalr.hb"; pub const LM_MNE_MOVZ: &str = "movz"; pub const LM_MNE_MOVN: &str = "movn";
pub const LM_MNE_SYSCALL: &str = "syscall"; pub const LM_MNE_BREAK: &str = "break"; pub const LM_MNE_SYNC: &str = "syn";