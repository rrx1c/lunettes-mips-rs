//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs

use crate::instruction::*;
use crate::operands::*;
use crate::disassembler::*;
use registers::*;
use FieldInfos;

//TODO: Je n'ai pas envie de debugger ce truc
//TODO: Je dois mettre les bonnes exceptions
//TODO: Dans le Release1 mfmc0 avait une autre exception, je dois rajouter les versions pour ça

//Opcode handlers

//Default opcode field handlers
pub fn j(instruction: &mut LmInstruction) -> Option<LmError>{
    instruction.mnemonic = LM_MNE_J;
    LmDisassembler::jump_format(instruction);
    None
}
pub fn jal(instruction: &mut LmInstruction) -> Option<LmError>{
    instruction.mnemonic = LM_MNE_JAL;
    LmDisassembler::jump_format(instruction);
    None
}
pub fn beq(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    let rt: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    

    instruction.is_relative = true;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.mnemonic = LM_MNE_BEQ;
    instruction.is_conditional = true;
    
    return LmDisassembler::imm_format(instruction, Some(rs), Some(rt), FieldInfos::default_imm_field(2));
}
pub fn bne(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    let rt: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    

    instruction.is_relative = true;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.mnemonic = LM_MNE_BNE;
    instruction.is_conditional = true;
    
    return LmDisassembler::imm_format(instruction, Some(rs), Some(rt), FieldInfos::default_imm_field(2));
}
pub fn blez(instruction: &mut LmInstruction) -> Option<LmError>{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BLEZ;
    instruction.is_conditional = true;
    instruction.category = LmInstructionCategory::BranchJump;
    let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    return LmDisassembler::imm_format(instruction, Some(rs), Some(FieldInfos::default_blank_field()), FieldInfos::default_imm_field(1));
}
pub fn bgtz(instruction: &mut LmInstruction) -> Option<LmError>{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BGTZ;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.is_conditional = true;
    let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    return LmDisassembler::imm_format(instruction, Some(rs), Some(FieldInfos::default_blank_field()), FieldInfos::default_imm_field(1));
}
pub fn addi_addiu(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
    let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    let sa: FieldInfos = FieldInfos::default_imm_field(2);

    instruction.mnemonic = match instruction.machine_code >> 26 & 1 {
        1 => LM_MNE_ADDIU,
        0 => {
            instruction.exception = LmInstructionException::LmIntOverflowExcept;
            LM_MNE_ADDI
        }
        _ => LM_MNE_NO_MNEMONIC
    };
    instruction.category = LmInstructionCategory::Arithmetic;
    return LmDisassembler::imm_format(instruction, Some(rs), Some(rt), sa);
}
pub fn slti_sltiu(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
    let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    let sa: FieldInfos = FieldInfos::default_imm_field(2);

    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = match instruction.machine_code >> 26 & 1 {
        1 => LM_MNE_SLTIU,
        0 => {
            instruction.exception = LmInstructionException::LmIntOverflowExcept;
            LM_MNE_SLTI
        }
        _ => LM_MNE_NO_MNEMONIC
    };


    return LmDisassembler::imm_format(instruction, Some(rs), Some(rt), sa);
}
pub fn andi(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
    let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    let sa: FieldInfos = FieldInfos::default_imm_field(2);

    instruction.mnemonic = LM_MNE_ANDI;
    instruction.category = LmInstructionCategory::Logical;

    return LmDisassembler::imm_format(instruction, Some(rs), Some(rt), sa);
}
pub fn ori(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
    let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    let sa: FieldInfos = FieldInfos::default_imm_field(2);

    instruction.mnemonic = LM_MNE_ORI;
    instruction.category = LmInstructionCategory::Logical;
    
    return LmDisassembler::imm_format(instruction, Some(rs), Some(rt), sa);
}
pub fn xori(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
    let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    let sa: FieldInfos = FieldInfos::default_imm_field(2);

    instruction.mnemonic = LM_MNE_XORI;
    instruction.category = LmInstructionCategory::Logical;
    
    return LmDisassembler::imm_format(instruction, Some(rs), Some(rt), sa);
}
pub fn lui(instruction: &mut LmInstruction) -> Option<LmError>{
    let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    let sa: FieldInfos = FieldInfos::default_imm_field(1);

    instruction.mnemonic = LM_MNE_LUI;
    instruction.category = LmInstructionCategory::Logical;

    return LmDisassembler::imm_format(instruction, Some(FieldInfos::default_blank_field()), Some(rt), sa);
}
pub fn beql(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    let rt: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
    let imm: FieldInfos = FieldInfos::default_imm_field(2);

    instruction.is_relative = true;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.mnemonic = LM_MNE_BEQL;
    instruction.is_conditional = true;
    
    return LmDisassembler::imm_format(instruction, Some(rs), Some(rt), imm);
}
pub fn bnel(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    let rt: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
    let imm: FieldInfos = FieldInfos::default_imm_field(2);

    instruction.is_relative = true;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.mnemonic = LM_MNE_BNEL;
    instruction.is_conditional = true;
    
    return LmDisassembler::imm_format(instruction, Some(rs), Some(rt), imm);
}
pub fn blezl(instruction: &mut LmInstruction) -> Option<LmError>{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BLEZL;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.is_conditional = true;
    let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    return LmDisassembler::imm_format(instruction, Some(rs), Some(FieldInfos::default_blank_field()), FieldInfos::default_imm_field(1));
}
pub fn bgtzl(instruction: &mut LmInstruction) -> Option<LmError>{
    instruction.is_relative = true;
    instruction.mnemonic = LM_MNE_BGTZL;
    instruction.category = LmInstructionCategory::BranchJump;
    instruction.is_conditional = true;
    let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
    return LmDisassembler::imm_format(instruction, Some(rs), Some(FieldInfos::default_blank_field()), FieldInfos::default_imm_field(1));
}
pub fn jalx(instruction: &mut LmInstruction) -> Option<LmError>{
    instruction.mnemonic = LM_MNE_JALX;
    LmDisassembler::jump_format(instruction);
    None
}
pub fn cpu_loadstore(instruction: &mut LmInstruction) -> Option<LmError>{
    let base: FieldInfos = FieldInfos::default_reg_field(2, LmCoprocessor::Cpu);
    let rt: FieldInfos;
    let mnemonics: [[&str; 7]; 4] = [
        [LM_MNE_LB, LM_MNE_LH, LM_MNE_LWL, LM_MNE_LW, LM_MNE_LBU, LM_MNE_LHU, LM_MNE_LWR],
        [LM_MNE_SB, LM_MNE_SH, LM_MNE_SWL, LM_MNE_SW, LM_MNE_NO_MNEMONIC, LM_MNE_NO_MNEMONIC, LM_MNE_SWR],
        [LM_MNE_LL, LM_MNE_LWC1, LM_MNE_LWC2, LM_MNE_NO_MNEMONIC, LM_MNE_NO_MNEMONIC, LM_MNE_LDC1, LM_MNE_LDC2],
        [LM_MNE_SC, LM_MNE_SWC1, LM_MNE_SWC2, LM_MNE_NO_MNEMONIC, LM_MNE_NO_MNEMONIC, LM_MNE_SDC1, LM_MNE_SDC2]
    ];

    instruction.mnemonic = mnemonics[(instruction.machine_code >> 29 & 3) as usize][(instruction.machine_code >> 26 & 7) as usize];

    if (instruction.machine_code >> 29 & 3) == 6 
    || (instruction.machine_code >> 29 & 3) == 7
    && (instruction.machine_code >> 27 & 3) == 1{
        rt = FieldInfos::default_reg_field(0, LmCoprocessor::Cp2);
    }
    else if (instruction.machine_code >> 29 & 3) == 6 
    || (instruction.machine_code >> 29 & 3) == 7
    && (instruction.machine_code >> 27 & 3) == 0{
        rt = FieldInfos::default_reg_field(0, LmCoprocessor::Cp1);
    }
    else {
        rt = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);
    }

    instruction.category = match instruction.machine_code & 1{
        0 => LmInstructionCategory::Load,
        1 => LmInstructionCategory::Store,
        _ => return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
    };

    return LmDisassembler::imm_format(instruction, Some(base), Some(rt), FieldInfos::default_imm_field(1))
}
pub fn cache_pref(instruction: &mut LmInstruction) -> Option<LmError>{
    let base: FieldInfos = FieldInfos::default_reg_field(2, LmCoprocessor::Cpu);
    let op: FieldInfos = FieldInfos::imm_field(0, 0b11111);
    
    instruction.mnemonic = match instruction.machine_code >> 26 & 4{
        4 =>     {
            instruction.category = LmInstructionCategory::MemoryControl;
            LM_MNE_PREF
        },
        0 => {
            instruction.category = LmInstructionCategory::Priviledge;
            LM_MNE_CACHE
        },
        _ => LM_MNE_NO_MNEMONIC
    };
    return LmDisassembler::imm_format(instruction, Some(base), Some(op), FieldInfos::default_imm_field(1));
}

//Special
pub fn sll(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::default_blank_field();
    let rt: FieldInfos;
    let rd: FieldInfos;
    let sa: FieldInfos;

    if instruction.machine_code >> 11 & 0b111111111111111 == 0{
        instruction.mnemonic = match instruction.machine_code >> 6 & 0b11111{
            0 => LM_MNE_NOP,
            1 => LM_MNE_SSNOP,
            3 => LM_MNE_EHB,
            5 => LM_MNE_PAUSE,
            _ => return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
        };
        rt = FieldInfos::default_blank_field();
        rd = FieldInfos::default_blank_field();
        sa = FieldInfos::default_blank_field();

        instruction.category = LmInstructionCategory::Control;

    }
    else{
        instruction.mnemonic = LM_MNE_SLL;
        instruction.category = LmInstructionCategory::Shift;

        rt = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        rd = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        sa = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Imm);
    }
    
    LmDisassembler::reg_format(instruction, Some(rs), Some(rt), Some(rd), Some(sa))
}
pub fn movci(instruction: &mut LmInstruction) -> Option<LmError>{
    //Reserved Instruction, Coprocessor Unusable
    if (instruction.machine_code >> 6 & 0b11111) != 0
    ||(instruction.machine_code >> 17 & 1) != 0{
        return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
    }
    let mnemonics = [LM_MNE_MOVF, LM_MNE_MOVT];
    let mut hex_num: LmString = LmString::new_lmstring();
    let comma: &str = ", ";
    let registers: [&str; 8] = [ LM_REG_FCC0, LM_REG_FCC1, LM_REG_FCC2, LM_REG_FCC3, LM_REG_FCC4, LM_REG_FCC5, LM_REG_FCC6, LM_REG_FCC7,];
    
    instruction.format = LmInstructionFormat::CoditionCodeFpu;
    instruction.category = LmInstructionCategory::Move;
    instruction.mnemonic = mnemonics[(instruction.machine_code >> 16 & 1) as usize];

    instruction.operand_num = 3;
    instruction.operand[0] = Some(LmOpRegister::new_reg_opreand((instruction.machine_code >> 11 & 0b11111) as u8, LmCoprocessor::Cpu));
    instruction.operand[1] = Some(LmOpRegister::new_reg_opreand((instruction.machine_code >> 21 & 0b11111) as u8, LmCoprocessor::Cpu));
    instruction.operand[2] = Some(LmOperand::LmOpRegister(LmOpRegister{register: registers[(instruction.machine_code >> 18 & 0b111) as usize], coprocessor: LmCoprocessor::Cp1}));

    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    for i in 0..instruction.operand_num{
        if let Some(op) = instruction.operand[i]{
            match op{
                LmOperand::LmOpRegister(reg) => _= instruction.string.append_str(reg.register),
                LmOperand::LmOpImmediate(imm) => {
                    hex_num.num_to_str(imm.value);
                    instruction.string.append_string(&hex_num);
                },
            };
            if instruction.operand_num - 1 > i{
                instruction.string.append_str(&comma);
            }
        }
    }
    None
}
pub fn srl_sra(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::blank_field(0b1111);
    let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let sa: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Imm);

    instruction.mnemonic = match instruction.machine_code & 1{
        1 => LM_MNE_SRA,
        0 => {
            match instruction.machine_code >> 6 & 1 {
                1 => LM_MNE_ROTR,
                0 => LM_MNE_SRL,
                _ => LM_MNE_NO_MNEMONIC
            }
        },
        _ => LM_MNE_NO_MNEMONIC
    };

    instruction.category = LmInstructionCategory::Shift;
    return LmDisassembler::reg_format(instruction, Some(rs), Some(rt), Some(rd), Some(sa))
}
pub fn sllv(instruction: &mut LmInstruction) -> Option<LmError>{
    instruction.mnemonic = LM_MNE_SLLV;
    instruction.category = LmInstructionCategory::Shift;

    let sa: FieldInfos = FieldInfos::default_blank_field();
    let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rs: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Reg);

    return LmDisassembler::reg_format(instruction, Some(rs), Some(rt), Some(rd), Some(sa))
}
pub fn srlv_srav(instruction: &mut LmInstruction) -> Option<LmError>{
    let sa: FieldInfos = FieldInfos::blank_field(0b1111);
    let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rs: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Imm);

    instruction.mnemonic = match instruction.machine_code & 1{
        1 => LM_MNE_SRAV,
        0 => {
            match instruction.machine_code >> 6 & 1 {
                1 => LM_MNE_ROTRV,
                0 => LM_MNE_SRLV,
                _ => LM_MNE_NO_MNEMONIC
            }
        },
        _ => LM_MNE_NO_MNEMONIC
    };

    instruction.category = LmInstructionCategory::Shift;
    return LmDisassembler::reg_format(instruction, Some(rs), Some(rt), Some(rd), Some(sa))
}
pub fn jr(instruction: &mut LmInstruction) -> Option<LmError>{
    let rd: FieldInfos = FieldInfos::blank_field(0b1111111111);
    let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);

    instruction.category = LmInstructionCategory::BranchJump;

    if (instruction.machine_code >> 6 & 0b11111) == 0b10000{
        instruction.mnemonic = LM_MNE_JRHB;
    }
    else{
        instruction.mnemonic = LM_MNE_JR;
    }

    LmDisassembler::reg_format(instruction, Some(rs), None, Some(rd), None)
}
pub fn jalr(instruction: &mut LmInstruction) -> Option<LmError>{
    let rt: FieldInfos = FieldInfos::default_blank_field();
    let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    
    instruction.category = LmInstructionCategory::BranchJump;

    if (instruction.machine_code >> 6 & 0b11111) == 0b10000{
        instruction.mnemonic = LM_MNE_JALRHB
    }
    else{
        instruction.mnemonic = LM_MNE_JALR
    }

    LmDisassembler::reg_format(instruction, Some(rs), Some(rt), Some(rd), None)
}
pub fn movn_movz(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rt: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);

    instruction.category = LmInstructionCategory::Move;
    instruction.is_conditional = true;

    if instruction.machine_code & 0b111111 == 0b001010{
        instruction.mnemonic = LM_MNE_MOVZ;
    }
    else{
        instruction.mnemonic = LM_MNE_MOVN;
    }
    return LmDisassembler::reg_format(instruction, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
}
pub fn syscall_break(instruction: &mut LmInstruction) -> Option<LmError>{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.mnemonic = match instruction.machine_code & 1{
        1 => LM_MNE_BREAK,
        0 => LM_MNE_SYSCALL,
        _ => LM_MNE_NO_MNEMONIC
    };
    instruction.category = LmInstructionCategory::Trap;
    instruction.format = LmInstructionFormat::Other;
    instruction.operand[0] = Some(LmOpImmediate::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64));

    if let Some(LmOperand::LmOpImmediate(imm)) = instruction.operand[0]{
        hex_num.num_to_str(imm.value);
    };
    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    instruction.string.append_string(&hex_num);
    None
}
pub fn sync(instruction: &mut LmInstruction) -> Option<LmError>{
    let rd: FieldInfos = FieldInfos::blank_field(0b111111111111111);
    let sa: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Imm);

    //Setting the attributes
    instruction.mnemonic = LM_MNE_SYNC;
    instruction.category = LmInstructionCategory::MemoryControl;
    LmDisassembler::reg_format(instruction, None, None, Some(rd), Some(sa))
}
pub fn mfhi_mflo(instruction: &mut LmInstruction) -> Option<LmError>{
    let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let mnemonics: [&str; 2] = [LM_MNE_MFHI, LM_MNE_MFLO];

    instruction.mnemonic = mnemonics[(instruction.machine_code >> 1 & 1) as usize];
    instruction.category = LmInstructionCategory::Move;

    LmDisassembler::reg_format(instruction, None, Some(FieldInfos::blank_field(0b1111111111)), Some(rd), Some(FieldInfos::default_blank_field()))
}
pub fn mthi_mtlo(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let mnemonics: [&str; 2] = [LM_MNE_MTHI, LM_MNE_MTLO];
    
    instruction.mnemonic = mnemonics[(instruction.machine_code >> 1 & 1) as usize];
    instruction.category = LmInstructionCategory::Move;

    LmDisassembler::reg_format(instruction, Some(rs), None, None, Some(FieldInfos::blank_field(0b111111111111111)))
}
pub fn mult_multu_div_divu(instruction: &mut LmInstruction) -> Option<LmError>{
    let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let mnemonics: [[&str; 2]; 2] = [[LM_MNE_MULT, LM_MNE_MULTU], [LM_MNE_DIV, LM_MNE_DIVU]];

    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = mnemonics[(instruction.machine_code >> 1 & 1) as usize][(instruction.machine_code & 1) as usize];

    LmDisassembler::reg_format(instruction, Some(rs), Some(rt), None, Some(FieldInfos::blank_field(0b1111111111)))
}
pub fn add_addu_sub_subu_and_or_xor_nor(instruction: &mut LmInstruction) -> Option<LmError>{
    let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rt: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let mnemonics: [[[&str; 2]; 2]; 2] = [[[LM_MNE_ADD, LM_MNE_ADDU], [LM_MNE_SUB, LM_MNE_SUBU]], [[LM_MNE_AND, LM_MNE_OR], [LM_MNE_XOR, LM_MNE_NOR]]];

    instruction.mnemonic = mnemonics[(instruction.machine_code >> 2 & 1) as usize][(instruction.machine_code >> 1 & 1) as usize][(instruction.machine_code & 1) as usize];
    if (instruction.machine_code >> 2 & 1) == 1{
        instruction.category = LmInstructionCategory::Logical;
    }
    else{
        instruction.category = LmInstructionCategory::Arithmetic;
        if (instruction.machine_code & 1) == 0{
            instruction.exception = LmInstructionException::LmIntOverflowExcept;
        }
    }

    LmDisassembler::reg_format(instruction, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
}
pub fn slt_sltu(instruction: &mut LmInstruction) -> Option<LmError>{
    let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rt: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let mnemonics: [&str; 2] = [LM_MNE_SLT, LM_MNE_SLTU];

    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.is_conditional = true;
    instruction.mnemonic = mnemonics[(instruction.machine_code & 1) as usize];

    LmDisassembler::reg_format(instruction, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
}
pub fn tge_tgeu_tlt_tltu(instruction: &mut LmInstruction) -> Option<LmError>{
    let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let mnemonics: [[&str; 2]; 2] = [[LM_MNE_TGE, LM_MNE_TGEU], [LM_MNE_TLT, LM_MNE_TLTU]];
    
    instruction.mnemonic = mnemonics[(instruction.machine_code >> 1 & 1) as usize][(instruction.machine_code & 1) as usize];
    instruction.category = LmInstructionCategory::Trap;
    instruction.exception = LmInstructionException::LmTrapExcept;

    LmDisassembler::reg_format(instruction, Some(rs), Some(rt), None, Some(FieldInfos::imm_field(2, 0b1111111111)))
}
pub fn teq_tne(instruction: &mut LmInstruction) -> Option<LmError>{
    let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    
    instruction.category = LmInstructionCategory::Trap;
    instruction.mnemonic = match instruction.machine_code >> 1 & 1{
        1 => LM_MNE_TEQ,
        0 => LM_MNE_TNE,
        _ => LM_MNE_NO_MNEMONIC
    };

    LmDisassembler::reg_format(instruction, Some(rs), Some(rt), None, Some(FieldInfos::imm_field(2, 0b1111111111)))
}

//Special2
pub fn madd_maddu(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);

    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.exception = LmInstructionException::LmIntOverflowExcept;
    instruction.mnemonic = match instruction.machine_code & 1{
        0 => LM_MNE_MADD,
        1 => LM_MNE_MADDU,
        _ => LM_MNE_NO_MNEMONIC
    };

    LmDisassembler::reg_format(instruction, Some(rs), Some(rt), None, Some(FieldInfos::blank_field(0b1111111111)))
}
pub fn mul(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rt: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);

    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.exception = LmInstructionException::LmIntOverflowExcept;
    instruction.mnemonic = LM_MNE_MUL;

    LmDisassembler::reg_format(instruction, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
}
pub fn msub_msubu(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);

    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.exception = LmInstructionException::LmIntOverflowExcept;
    instruction.mnemonic = match instruction.machine_code & 1{
        0 => LM_MNE_MSUB,
        1 => LM_MNE_MSUBU,
        _ => LM_MNE_NO_MNEMONIC
    };

    LmDisassembler::reg_format(instruction, Some(rs), Some(rt), None, Some(FieldInfos::blank_field(0b1111111111)))
}
pub fn clz_clo(instruction: &mut LmInstruction) -> Option<LmError>{
    let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);

    instruction.category = LmInstructionCategory::Arithmetic;
    instruction.mnemonic = match instruction.machine_code & 1{
        0 => LM_MNE_CLZ,
        1 => LM_MNE_CLO,
        _ => LM_MNE_NO_MNEMONIC
    };
    let success = LmDisassembler::reg_format(instruction, Some(rs), None, Some(rd), Some(FieldInfos::default_blank_field()));
    
    return success
}
pub fn sdbbp(instruction: &mut LmInstruction) -> Option<LmError>{
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.mnemonic = LM_MNE_SDBBP;
    instruction.category = LmInstructionCategory::Trap;
    instruction.format = LmInstructionFormat::Other;
    instruction.operand[0] = Some(LmOpImmediate::new_imm_opreand(((instruction.machine_code >> 6) & 0xFFFFF) as u64));

    if let Some(LmOperand::LmOpImmediate(imm)) = instruction.operand[0]{
        hex_num.num_to_str(imm.value);
    };
    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    instruction.string.append_string(&hex_num);
    None
}

//Special3 They need some testing
pub fn ext(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.mnemonic = LM_MNE_EXT;
    instruction.category = LmInstructionCategory::InsertExtract;
    instruction.exception = LmInstructionException::LmReservedInstructionException;

    let success = LmDisassembler::reg_format(instruction, Some(rs), Some(rt), None, None);

    instruction.operand_num = 4;
    instruction.operand[2] = Some(LmOpImmediate::new_imm_opreand((instruction.machine_code >> 6 & 0b11111) as u64));
    instruction.operand[3] = Some(LmOpImmediate::new_imm_opreand((instruction.machine_code >> 11 & 0b11111) as u64));
    
    instruction.string.append_str(", ");
    if let Some(LmOperand::LmOpImmediate(imm)) = instruction.operand[2]{
        hex_num.num_to_str(imm.value);
        instruction.string.append_string(&hex_num);
    }
    instruction.string.append_str(", ");
    if let Some(LmOperand::LmOpImmediate(imm)) = instruction.operand[3]{
        hex_num.num_to_str(imm.value);
        instruction.string.append_string(&hex_num);
    }
    return success
}
pub fn ins(instruction: &mut LmInstruction) -> Option<LmError>{
    let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    let mut hex_num: LmString = LmString::new_lmstring();

    instruction.mnemonic = LM_MNE_INS;
    instruction.category = LmInstructionCategory::InsertExtract;
    instruction.exception = LmInstructionException::LmReservedInstructionException;

    let success = LmDisassembler::reg_format(instruction, Some(rs), Some(rt), None, None);

    instruction.operand_num = 4;
    instruction.operand[2] = Some(LmOpImmediate::new_imm_opreand((instruction.machine_code >> 6 & 0b11111) as u64));
    instruction.operand[3] = Some(LmOpImmediate::new_imm_opreand((instruction.machine_code >> 11 & 0b11111) as u64));
    
    instruction.string.append_str(", ");
    if let Some(LmOperand::LmOpImmediate(imm)) = instruction.operand[3]{
        if let Some(LmOperand::LmOpImmediate(imm1)) = instruction.operand[2]{
            hex_num.num_to_str(imm.value - imm1.value + 1);
            instruction.string.append_string(&hex_num);
        }
    }
    instruction.string.append_str(", ");
    if let Some(LmOperand::LmOpImmediate(imm)) = instruction.operand[3]{
        hex_num.num_to_str(imm.value);
        instruction.string.append_string(&hex_num);
    }
    return success
}
pub fn bshfl(instruction: &mut LmInstruction) -> Option<LmError>{
    let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);

    instruction.mnemonic = match instruction.machine_code >> 6 & 0b11111{
        0b00010 => {
            instruction.category = LmInstructionCategory::InsertExtract;
            LM_MNE_WSBH},
        0b10000 => {
            instruction.category = LmInstructionCategory::Arithmetic;
            LM_MNE_SEB},
        0b11000 => {
            instruction.category = LmInstructionCategory::Arithmetic;
            LM_MNE_SEH},
        _ => return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
    };
    
    LmDisassembler::reg_format(instruction, Some(FieldInfos::default_blank_field()), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
}
pub fn rdhwr(instruction: &mut LmInstruction) -> Option<LmError>{
    let rt: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
    let rd: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
    
    instruction.category = LmInstructionCategory::Move;
    instruction.mnemonic = LM_MNE_RDHWR;

    LmDisassembler::reg_format(instruction, Some(FieldInfos::default_blank_field()), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
}

//CP0
pub fn mov_cp0(instruction: &mut LmInstruction) -> Option<LmError>{
    let mnemonics = [LM_MNE_MFC0, LM_MNE_MTC0];
    if (instruction.machine_code >> 3 & 0b11111111) != 0{
        return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
    }

    instruction.category = LmInstructionCategory::Priviledge;
    instruction.format = LmInstructionFormat::Other;
    instruction.mnemonic = mnemonics[(instruction.machine_code >> 23 & 1) as usize];
    instruction.operand_num = 3;

    instruction.operand[0] = Some(LmOpRegister::new_reg_opreand((instruction.machine_code >> 16 & 0b11111) as u8, LmCoprocessor::Cpu));
    instruction.operand[1] = Some(LmOpRegister::new_reg_opreand((instruction.machine_code >> 11 & 0b11111) as u8, LmCoprocessor::Cpu));
    instruction.operand[2] = Some(LmOpImmediate::new_imm_opreand((instruction.machine_code & 7) as u64));

    LmDisassembler::basic_str_format(instruction);

    None
}
pub fn gpr_shadowset(instruction: &mut LmInstruction) -> Option<LmError>{
    let mnemonics = [LM_MNE_RDPGPR, LM_MNE_WRPGPR];

    instruction.category = LmInstructionCategory::Priviledge;
    instruction.mnemonic = mnemonics[(instruction.machine_code >> 23 & 1) as usize];
    LmDisassembler::cpx_cpu_transfer_format(instruction, FieldInfos::default_reg_field(1, LmCoprocessor::Cpu), FieldInfos::default_reg_field(0, LmCoprocessor::Cpu))
}
pub fn mfmc0(instruction: &mut LmInstruction) -> Option<LmError>{
    let mnemonics = [LM_MNE_DI, LM_MNE_EI];

    if instruction.machine_code & 0b11111 != 0 ||
    (instruction.machine_code >> 6 & 0b11111) != 0 || 
    (instruction.machine_code >> 11 & 0b01100) != 0b01100 {
        return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
    }
    
    instruction.exception = LmInstructionException::LmCoprocessorUnusableException;
    instruction.category = LmInstructionCategory::Priviledge;
    instruction.format = LmInstructionFormat::Mfmc0;
    instruction.mnemonic = mnemonics[(instruction.machine_code >> 5 & 1) as usize];
    instruction.operand_num = 1;
    instruction.operand[0] = Some(LmOpRegister::new_reg_opreand((instruction.machine_code >> 16 & 0b11111) as u8, LmCoprocessor::Cpu));

    instruction.string.append_str(instruction.mnemonic);
    instruction.string.append_char(' ');
    if let Some(LmOperand::LmOpRegister(reg)) = instruction.operand[0]{
        instruction.string.append_str(reg.register);
    }
    None
}
pub fn c0(instruction: &mut LmInstruction) -> Option<LmError>{
    let mnemonics: [[&str; 8]; 8] = [
        [LM_MNE_NO_MNEMONIC,  LM_MNE_TLBR,  LM_MNE_TLBWI,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_TLBWR,  LM_MNE_NO_MNEMONIC],
        [LM_MNE_TLBP,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC],
        [LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC],
        [LM_MNE_ERET,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_DERET], 
        [LM_MNE_WAIT,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_SYNCI],
        [LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_SYNCI],
        [LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_SYNCI],
        [LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_NO_MNEMONIC,  LM_MNE_SYNCI]
    ];
    if (instruction.machine_code >> 6 & 0b1111111111111111111) != 0 ||
    (instruction.machine_code >> 25 & 1) != 1{
        return Some(LmError::throw_error(LmErrorCode::FieldBadValue, instruction.opcode, instruction.address, instruction.machine_code))
    }

    instruction.category = LmInstructionCategory::Priviledge;
    instruction.format = LmInstructionFormat::Other;
    instruction.mnemonic = mnemonics[(instruction.machine_code >> 3 & 0b111) as usize][(instruction.machine_code & 0b111) as usize];
    instruction.string.append_str(instruction.mnemonic);

    assert_ne!(instruction.mnemonic.cmp(LM_MNE_NO_MNEMONIC), Ordering::Equal);
    None
}
