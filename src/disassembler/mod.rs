//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs

mod opcode_handlers;

use core::cmp::Ordering;
use opcode_handlers::*;
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
    pub fn disassemble(&self, memory: u32, address: u64) -> Result<LmInstruction, LmError>{
        //Une map qui réunit tous les handlers des opcodes, il y a d'autre map dans cette map
        const OPCODE_MAP: [fn (instruction: &mut LmInstruction) -> Option<LmError>; 64] = [
            special_opcode_map, regimm_opcode_map, j, jal, beq, bne,  blez,  bgtz,
            addi_addiu,  addi_addiu,  slti_sltiu,  slti_sltiu,  andi,  ori,  xori,  lui,
            cop0_opcode_map,  cop1_opcode_map,  cop2_opcode_map,  cop1x_opcode_map,  beql,  bnel,  blezl,  bgtzl,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  special2_opcode_map,  jalx,  no_instructions,  special3_opcode_map,
            cpu_loadstore,  cpu_loadstore,  cpu_loadstore,  cpu_loadstore,  cpu_loadstore,  cpu_loadstore,  cpu_loadstore,  no_instructions,
            cpu_loadstore,  cpu_loadstore,  cpu_loadstore,  cpu_loadstore,  no_instructions,  no_instructions,  cpu_loadstore,  cache_pref,
            cpu_loadstore,  cpu_loadstore,  cpu_loadstore,  cache_pref,  no_instructions, cpu_loadstore, cpu_loadstore,  no_instructions,
            cpu_loadstore,  cpu_loadstore,  cpu_loadstore,  no_instructions,  no_instructions,  cpu_loadstore,  cpu_loadstore,  no_instructions];

        let mut instruction: LmInstruction = LmInstruction{
            category: LmInstructionCategory::NoFunction,
            format: LmInstructionFormat::NoFormat,
            operand_num: 0,
            is_conditional: false,
            coprocessor: match memory >> 26{
                0x20 => LmCoprocessor::Cp0,
                0x21 => LmCoprocessor::Cp1,
                0x22 => LmCoprocessor::Cp2,
                0x23 => LmCoprocessor::Cp1x,
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
        
        return match OPCODE_MAP[(memory >> 26) as usize](&mut instruction) {
            Some(e) => Err(e),
            None => Ok(instruction),
        }
    }
    fn reg_format(instruction: &mut LmInstruction, rs: Option<FieldInfos>, rt: Option<FieldInfos>, rd: Option<FieldInfos>, sa: Option<FieldInfos>) -> Option<LmError>{
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
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.address, instruction.machine_code))
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
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.address, instruction.machine_code))
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
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.address, instruction.machine_code))
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
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.address, instruction.machine_code))
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
    fn imm_format(instruction: &mut LmInstruction, rs: Option<FieldInfos>, rt: Option<FieldInfos>, imm: FieldInfos) -> Option<LmError>{

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
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.address, instruction.machine_code))
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
                return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.address, instruction.machine_code))
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
    fn jump_format(instruction: &mut LmInstruction) -> (){
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

//Opcode handlers map
fn no_instructions(instruction: &mut LmInstruction) -> Option<LmError>{
    Some(LmError::throw_error(LmErrorCode::NoInstruction, instruction.address, instruction.machine_code))
}
fn special_opcode_map(instruction: &mut LmInstruction) -> Option<LmError>{
    static SPECIAL_MAP: [fn(&mut LmInstruction) -> Option<LmError>; 64] = [
    sll,  movci,  srl_sra,  srl_sra,  sllv,  no_instructions,  srlv_srav,  srlv_srav,
    jr,  jalr,  movn_movz,  movn_movz,  syscall_break,  syscall_break,  no_instructions,  sync,
    mfhi_mflo,  mthi_mtlo,  mfhi_mflo,  mthi_mtlo,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
    mult_multu_div_divu,  mult_multu_div_divu,  mult_multu_div_divu,  mult_multu_div_divu,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
    add_addu_sub_subu_and_or_xor_nor,  add_addu_sub_subu_and_or_xor_nor,  add_addu_sub_subu_and_or_xor_nor,  add_addu_sub_subu_and_or_xor_nor,  add_addu_sub_subu_and_or_xor_nor,  add_addu_sub_subu_and_or_xor_nor,  add_addu_sub_subu_and_or_xor_nor,  add_addu_sub_subu_and_or_xor_nor,
    no_instructions,  no_instructions,  slt_sltu,  slt_sltu,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
    tge_tgeu_tlt_tltu,  tge_tgeu_tlt_tltu,  tge_tgeu_tlt_tltu,  tge_tgeu_tlt_tltu,  teq_tne,  no_instructions,  teq_tne,  no_instructions,
    no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];

    SPECIAL_MAP[(instruction.machine_code & 0b111111) as usize](instruction)
}
fn regimm_opcode_map(instruction: &mut LmInstruction) -> Option<LmError>{
    let imm_order: usize;
    let rs: Option<FieldInfos>;
    static MENMONICS: [[&str; 8]; 4] =
    [   [LM_MNE_BLTZL,  LM_MNE_BGEZ,  LM_MNE_BLTZL,  LM_MNE_BGEZL,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC],
        [LM_MNE_TGEI,  LM_MNE_TGEIU,  LM_MNE_TLTI,  LM_MNE_TLTIU,  LM_MNE_TEQI,  LM_MNE_NO_MNEMONIC,  LM_MNE_TNEI,  LM_MNE_NO_MNEMONIC],
        [LM_MNE_BLTZAL,  LM_MNE_BGEZAL,  LM_MNE_BLTZALL,  LM_MNE_BGEZALL,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC],
        [LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_SYNCI] ];
    
    instruction.mnemonic = MENMONICS[(instruction.machine_code >> 19 & 0b11) as usize][(instruction.machine_code >> 16 & 0b111) as usize];
    instruction.category = match instruction.machine_code >> 19 & 3{
        3 => LmInstructionCategory::MemoryControl,
        1 => {
            instruction.exception = LmInstructionException::LmTrapExcept;
            instruction.is_conditional = true;
            LmInstructionCategory::Trap
        },
        _ => {
            instruction.is_relative = true;
            instruction.is_conditional = true;
            LmInstructionCategory::BranchJump
        },
    };

    if (instruction.machine_code >> 16 & 0b111111) == 0x11
    && (instruction.machine_code >> 21 & 0b11111) == 0{
        instruction.mnemonic = LM_MNE_BAL;
        rs = None;
        imm_order = 0;
        instruction.is_conditional = false;
    }
    else if (instruction.machine_code >> 16 & 0b111111) == 0x1f{
        imm_order = 0;
        rs = Some(FieldInfos::default_reg_field(1, LmCoprocessor::Cpu));
    }
    else{
        imm_order = 1;
        rs = Some(FieldInfos::default_reg_field(0, LmCoprocessor::Cpu));
    }

    return LmDisassembler::imm_format(instruction, rs, None, FieldInfos::default_imm_field(imm_order))
}
fn special2_opcode_map(instruction: &mut LmInstruction) -> Option<LmError>{
    static SPECIAL2_MAP: [fn(&mut LmInstruction) -> Option<LmError>; 64] = 
        [   madd_maddu,  madd_maddu,  mul,  no_instructions,  msub_msubu,  msub_msubu,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            clz_clo,  clz_clo,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  sdbbp ];
    SPECIAL2_MAP[(instruction.machine_code & 0b111111) as usize](instruction)
}
fn special3_opcode_map(instruction: &mut LmInstruction) -> Option<LmError>{
    static SPECIAL3_MAP: [fn(&mut LmInstruction) -> Option<LmError>; 64] = 
        [   ext,  no_instructions,  no_instructions,  no_instructions,  ins,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            bshfl,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  rdhwr,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];
    
    SPECIAL3_MAP[(instruction.machine_code & 0b111111) as usize](instruction)
}
fn cop0_opcode_map(_instruction: &mut LmInstruction) -> Option<LmError>{
    static _COP0_MAP: [fn(&mut LmInstruction) -> Option<LmError>; 64] =
        [   no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
            no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];
    unimplemented!("[-]Opcode map isn't implemented yet!");
    // instruction.coprocessor = LmCoprocessor::Cp0;
    // COP0_MAP[(instruction.machine_code >> 26) as usize](instruction)
}
fn cop1_opcode_map(_instruction: &mut LmInstruction) -> Option<LmError>{
    static _COP1_MAP: [fn(&mut LmInstruction) -> Option<LmError>; 64] =
    [   no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];
    unimplemented!("[-]Opcode map isn't implemented yet!");

    // instruction.coprocessor = LmCoprocessor::Cp1;
    // COP1_MAP[(instruction.machine_code >> 26) as usize](instruction)
}
fn cop2_opcode_map(_instruction: &mut LmInstruction) -> Option<LmError>{
    static _COP2_MAP: [fn(&mut LmInstruction) -> Option<LmError>; 64] = 
    [   no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];
    unimplemented!("[-]Opcode map isn't implemented yet!");

    // instruction.coprocessor = LmCoprocessor::Cp2;
    // COP2_MAP[(instruction.machine_code >> 26) as usize](instruction)
}
fn cop1x_opcode_map(_instruction: &mut LmInstruction) -> Option<LmError>{
    static _COP1X_MAP: [fn(&mut LmInstruction) -> Option<LmError>; 64] = 
    [   no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,
        no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions,  no_instructions ];
    unimplemented!("[-]Opcode map isn't implemented yet!");

    // instruction.coprocessor = LmCoprocessor::Cp1x;
    // _COP1X_MAP[(instruction.machine_code >> 26) as usize](instruction)
}