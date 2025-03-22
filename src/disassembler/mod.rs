//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs

mod opcode_handlers;

use core::cmp::Ordering;
use super::instruction::*;
use super::LmAddressSize;
use super::operands::*;
use super::utils::string::*;
use super::error::*;

#[derive(Debug, Copy, Clone)]
pub struct LmDisassembler{
    pub address_size: LmAddressSize,
    pub _version: LmInstructionVersion
}

struct FieldInfos{
    mask: u32,                    //The mask of bits this field takes
    op_type: Option<LmOperandType>,         //Defines the type of this operand, if there's no type, the field 
                                    //reprsented by this struct should be skipped
    coprocessor: LmCoprocessor,     //Defines the coprocessor of the register if op_type a register
    blank: bool,                    //Means that the field is supposed to be 0x00
    operand_order: usize,           //Order of operand in the instruction string
}

impl FieldInfos{
    fn reg_field(operand_order: usize, coprocessor: LmCoprocessor, op_type: LmOperandType) -> FieldInfos{
        FieldInfos{
            mask: 0b11111, op_type: Some(op_type),
            coprocessor, blank: false,
            operand_order
        }
    }
    fn default_reg_field(operand_order: usize, coprocessor: LmCoprocessor) -> FieldInfos{
        FieldInfos{
            mask: 0b11111, op_type: Some(LmOperandType::Reg),
            coprocessor, blank: false,
            operand_order
        }
    }
    fn default_imm_field(operand_order: usize) -> FieldInfos{
        FieldInfos{
            mask: 0b1111111111111111, op_type: Some(LmOperandType::Imm),
            coprocessor: LmCoprocessor::Cpu, blank: false,
            operand_order
        }
    }
    fn imm_field(order: usize, mask: u32) -> FieldInfos{
        FieldInfos{
            mask: mask, op_type: Some(LmOperandType::Imm),
            coprocessor: LmCoprocessor::NoCoprocessor, blank: false,
            operand_order: order
        }
    }
    fn blank_field(mask: u32) -> FieldInfos{
        FieldInfos{
            mask: mask, op_type: None,
            coprocessor: LmCoprocessor::NoCoprocessor, blank: true,
            operand_order: 4
        }
    }
    fn default_blank_field() -> FieldInfos{
        FieldInfos{
            mask: 0b11111, op_type: None,
            coprocessor: LmCoprocessor::NoCoprocessor, blank: true,
            operand_order: 4
        }
    }

}

impl LmDisassembler{
    pub fn new_disassembler(address_size: LmAddressSize) -> LmDisassembler{
        LmDisassembler{
            address_size,
            _version: LmInstructionVersion::NoVersion,
        }
    }
    //Opcode handlers map
    pub fn disassemble(&self, memory: u32, address: u64) -> Result<LmInstruction, LmError>{
        //Une map qui réunit tous les handlers des opcodes, il y a d'autre map dans cette map
        const OPCODE_MAP: [fn (disass: &LmDisassembler, instruction: &mut LmInstruction) -> Option<LmError>; 64] = [
            LmDisassembler::special_opcode_map, LmDisassembler::regimm_opcode_map, LmDisassembler::j, LmDisassembler::jal, LmDisassembler::beq, LmDisassembler::bne,  LmDisassembler::blez,  LmDisassembler::bgtz,
            LmDisassembler::addi_addiu,  LmDisassembler::addi_addiu,  LmDisassembler::slti_sltiu,  LmDisassembler::slti_sltiu,  LmDisassembler::andi,  LmDisassembler::ori,  LmDisassembler::xori,  LmDisassembler::lui,
            LmDisassembler::cop0_opcode_map,  LmDisassembler::cop1_opcode_map,  LmDisassembler::cop2_opcode_map,  LmDisassembler::cop1x_opcode_map,  LmDisassembler::beql,  LmDisassembler::bnel,  LmDisassembler::blezl,  LmDisassembler::bgtzl,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::special2_opcode_map,  LmDisassembler::jalx,  LmDisassembler::no_instructions,  LmDisassembler::special3_opcode_map,
            LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::no_instructions,
            LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::cpu_loadstore,  LmDisassembler::cache_pref,
            LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::cache_pref,  LmDisassembler::no_instructions, LmDisassembler::cpu_loadstore, LmDisassembler::cpu_loadstore,  LmDisassembler::no_instructions,
            LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::cpu_loadstore,  LmDisassembler::cpu_loadstore,  LmDisassembler::no_instructions];

        let mut instruction: LmInstruction = LmInstruction{
            category: LmInstructionCategory::NoFunction,
            format: LmInstructionFormat::NoFormat,
            operand_num: 0,
            is_conditional: false,
            opcode: (memory >> 26) as u8,
            coprocessor: match memory >> 26{
                0x10 => LmCoprocessor::Cp0,
                0x11 => LmCoprocessor::Cp1,
                0x12 => LmCoprocessor::Cp2,
                0x13 => LmCoprocessor::Cp1x,
                _ => LmCoprocessor::Cpu,
            },
            machine_code: memory,
            operand: [None; 4],
            is_relative: false,
            exception: LmInstructionException::NoException,
            is_region: false,
            string: LmString::new_lmstring(),
            mnemonic: LM_MNE_NO_MNEMONIC,
            address,
            address_size: self.address_size,
            version: LmInstructionVersion::NoVersion
        };
        
        return match OPCODE_MAP[(memory >> 26) as usize](self, &mut instruction) {
            Some(e) => Err(e),
            None => Ok(instruction),
        }
    }
    fn reg_format(&self, instruction: &mut LmInstruction, rs: Option<FieldInfos>, rt: Option<FieldInfos>, rd: Option<FieldInfos>, sa: Option<FieldInfos>) -> Option<LmError>{
        let mut hex_num: LmString = LmString::new_lmstring();
        let comma: &str = ", ";

        instruction.format = LmInstructionFormat::Reg;

        //Rs field
        if let Some(field) = rs{
            let field_mask_result = instruction.machine_code >> 21 & field.mask;
            if field.blank == false{
                if let Some(op_type) = field.op_type {
                    instruction.operand[field.operand_order] = match op_type{
                        LmOperandType::Imm =>{
                            instruction.operand_num += 1;
                            Some(LmOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        LmOperandType::Reg => {
                            instruction.operand_num += 1;
                            Some(LmOpRegister::new_reg_opreand(field_mask_result as u8, field.coprocessor))
                        },        
                    }
                }
            }
            else if field_mask_result != 0{
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }
        //Rt field
        if let Some(field) = rt{
            let field_mask_result = instruction.machine_code >> 16 & field.mask;
            if field.blank == false{
                if let Some(op_type) = field.op_type {
                    instruction.operand[field.operand_order] = match op_type{
                        LmOperandType::Imm =>{
                            instruction.operand_num += 1;
                            Some(LmOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        LmOperandType::Reg => {
                            instruction.operand_num += 1;
                            Some(LmOpRegister::new_reg_opreand(field_mask_result as u8, field.coprocessor))
                        },        
                    }
                }
            }
            else if field_mask_result != 0{
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }
        //Rd field
        if let Some(field) = rd{
            let field_mask_result = instruction.machine_code >> 11 & field.mask;
            if field.blank == false{
                if let Some(op_type) = field.op_type {
                    match op_type{
                        LmOperandType::Imm =>{
                            instruction.operand_num += 1;
                            instruction.operand[field.operand_order] = Some(LmOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        LmOperandType::Reg => {
                            instruction.operand_num += 1;
                            instruction.operand[field.operand_order] = Some(LmOpRegister::new_reg_opreand(field_mask_result as u8, field.coprocessor))
                        },        
                    }
                }
            }
            else if field_mask_result != 0{
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }
        //Sa field
        if let Some(field) = sa{
            let field_mask_result = instruction.machine_code >> 6 & field.mask;
            if field.blank == false{
                if let Some(op_type) = field.op_type {
                    match op_type{
                        LmOperandType::Imm =>{
                            instruction.operand_num += 1;
                            instruction.operand[field.operand_order] = Some(LmOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        LmOperandType::Reg => {
                            instruction.operand_num += 1;
                            instruction.operand[field.operand_order] = Some(LmOpRegister::new_reg_opreand(field_mask_result as u8, field.coprocessor))
                        },        
                    }
                }
            }
            else if field_mask_result != 0{
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }

        instruction.string.append_str(instruction.mnemonic);
        instruction.string.append_char(' ');
        for i in 0..instruction.operand_num{
            if let Some(LmOperand::LmOpRegister(reg)) = instruction.operand[i]{
                instruction.string.append_str(reg.register);
            }
            else if let Some(LmOperand::LmOpImmediate(imm)) = instruction.operand[i]{
                hex_num.num_to_str(imm.value);
                instruction.string.append_string(&hex_num);
            }

            if instruction.operand_num - 1 > i{
                instruction.string.append_str(&comma);
            }
        }
        None
    }
    fn basic_str_format(instruction: &mut LmInstruction){
        let mut hex_num: LmString = LmString::new_lmstring();
        let comma: &str = ", ";

        instruction.string.append_str(instruction.mnemonic);
        instruction.string.append_char(' ');
        for i in 0..instruction.operand_num{
            if let Some(LmOperand::LmOpRegister(reg)) = instruction.operand[i]{
                instruction.string.append_str(reg.register);
            }
            else if let Some(LmOperand::LmOpImmediate(imm)) = instruction.operand[i]{
                hex_num.num_to_str(imm.value);
                instruction.string.append_string(&hex_num);
            }

            if instruction.operand_num - 1 > i{
                instruction.string.append_str(&comma);
            }
        }

    }
    fn cpx_cpu_transfer_format(&self, instruction: &mut LmInstruction, rt: FieldInfos, rd: FieldInfos) -> Option<LmError>{
        if (instruction.machine_code & 0b11111111111) != 0{
            return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
        }

        instruction.format = LmInstructionFormat::CpxCpuTransfer;

        instruction.operand_num = 2;
        instruction.operand[rd.operand_order] = Some(LmOpRegister::new_reg_opreand((instruction.machine_code >> 11 & rd.mask) as u8, rd.coprocessor));
        instruction.operand[rt.operand_order] = Some(LmOpRegister::new_reg_opreand((instruction.machine_code >> 16 & rt.mask) as u8, rt.coprocessor));

        LmDisassembler::basic_str_format(instruction);

        None
    }
    fn imm_format(&self, instruction: &mut LmInstruction, rs: Option<FieldInfos>, rt: Option<FieldInfos>, imm: FieldInfos) -> Option<LmError>{

        //Some attributes about the instruction and setting the operands
        instruction.format = LmInstructionFormat::Imm;
        instruction.operand_num =  1;
        //Rs field
        if let Some(field) = rs{
            let field_mask_result = instruction.machine_code >> 21 & field.mask;
            if field.blank == false{
                if let Some(op_type) = field.op_type {
                    match op_type{
                        LmOperandType::Imm =>{
                            instruction.operand_num += 1;
                            instruction.operand[field.operand_order] = Some(LmOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        LmOperandType::Reg => {
                            instruction.operand_num += 1;
                            instruction.operand[field.operand_order] = Some(LmOpRegister::new_reg_opreand(field_mask_result as u8, field.coprocessor))
                        },        
                    }
                }
            }
            else if field_mask_result != 0{
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }
        //Rt field
        if let Some(field) = rt{
            let field_mask_result = instruction.machine_code >> 16 & field.mask;
            if field.blank == false{
                if let Some(op_type) = field.op_type {
                    instruction.operand[field.operand_order] = match op_type{
                        LmOperandType::Imm =>{
                            instruction.operand_num += 1;
                            Some(LmOpImmediate::new_imm_opreand(field_mask_result as u64))
                        },
                        LmOperandType::Reg => {
                            instruction.operand_num += 1;
                            Some(LmOpRegister::new_reg_opreand(field_mask_result as u8, field.coprocessor))
                        },
                    }
                }
            }
            else if field_mask_result != 0{
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
            }
        }
        //Imm field
        instruction.operand[imm.operand_order] = Some(LmOpImmediate::new_imm_opreand((instruction.machine_code & 0b1111111111111111) as u64));
        
        if instruction.category == LmInstructionCategory::Load || instruction.category == LmInstructionCategory::Store
        || instruction.category == LmInstructionCategory::MemoryControl || instruction.mnemonic.cmp(LM_MNE_CACHE)  == Ordering::Equal{
            LmDisassembler::imm_loadstore_str_format(instruction);
        }
        else {
            LmDisassembler::imm_default_str_format(instruction);
        }

        None
    }
    fn imm_default_str_format(instruction: &mut LmInstruction){
        let mut hex_num: LmString = LmString::new_lmstring();
        let comma: &str = ", ";

        instruction.string.append_str(instruction.mnemonic);
        instruction.string.append_char(' ');
        for i in 0..instruction.operand_num{
            if let Some(LmOperand::LmOpRegister(reg)) = instruction.operand[i]{
                instruction.string.append_str(reg.register);
            }
            else if let Some(LmOperand::LmOpImmediate(imm)) = instruction.operand[i]{
                hex_num.num_to_str(imm.value);
                instruction.string.append_string(&hex_num);
            }

            if instruction.operand_num - 1 > i{
                instruction.string.append_str(&comma);
            }
        }
    }
    fn imm_loadstore_str_format(instruction: &mut LmInstruction){
        let mut hex_num: LmString = LmString::new_lmstring();
        let comma: &str = ", ";

        instruction.string.append_str(instruction.mnemonic);
        instruction.string.append_char(' ');
        for i in 0..instruction.operand_num - 1{
            if let Some(LmOperand::LmOpRegister(reg)) = instruction.operand[i]{
                instruction.string.append_str(reg.register);
            }
            else if let Some(LmOperand::LmOpImmediate(imm)) = instruction.operand[i]{
                hex_num.num_to_str(imm.value);
                instruction.string.append_string(&hex_num);
            }
            if instruction.operand_num - 2 > i{
                instruction.string.append_str(&comma);
            }
        }
        instruction.string.append_char('(');
        if let Some(LmOperand::LmOpRegister(reg)) = instruction.operand[instruction.operand_num - 1]{
            instruction.string.append_str(reg.register);
        }
        instruction.string.append_char(')');
    }
    fn jump_format(&self, instruction: &mut LmInstruction) -> (){
        let mut hex_num: LmString = LmString::new_lmstring();

        //Some attributes about the instruction
        instruction.format = LmInstructionFormat::Jump;
        instruction.operand_num = 1 ;
        instruction.is_region = true;
        instruction.category = LmInstructionCategory::BranchJump;
        instruction.operand[0] = Some(LmOpImmediate::new_imm_opreand((instruction.machine_code & 0x3FFFFFF) as u64));

        //Formatting the string
        //If the branch/jump is relative, the string will show it's destination address instead of the offset
        if let Some(LmOperand::LmOpImmediate(imm)) = instruction.operand[0]{
            hex_num.num_to_str(imm.value * 0x4 + instruction.address);
        }
        instruction.string.append_str(instruction.mnemonic);
        instruction.string.append_char(' ');
        instruction.string.append_string(&hex_num);

        return;
    }
}