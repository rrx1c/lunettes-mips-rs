//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs
use crate::mips::instruction::*;
use crate::mips::operands::*;
use crate::mips::operands::registers::*;
use crate::mips::LmAddressSize;

//#TODO: Spend more time with JR and JALR
//#TODO: Spend more time with Break
//#TODO: Pref has the Miscellaneous function and I don't know about cache, I guess it's the same as pref

fn u32_to_register(register: u32) -> Option<LmRegisters>{
    return match register{
        0 => Some(LmRegisters::ZERO), 1 => Some(LmRegisters::AT), 2 => Some(LmRegisters::V0), 3 => Some(LmRegisters::V1), 4 => Some(LmRegisters::A0), 5 => Some(LmRegisters::A1), 6 => Some(LmRegisters::A2), 7 => Some(LmRegisters::A3),
        8 => Some(LmRegisters::T0), 9 => Some(LmRegisters::T1), 10 => Some(LmRegisters::T2), 11 => Some(LmRegisters::T3), 12 => Some(LmRegisters::T4), 13 => Some(LmRegisters::T5), 14 => Some(LmRegisters::T6), 15 => Some(LmRegisters::T7),
        16 => Some(LmRegisters::S0), 17 => Some(LmRegisters::S1), 18 => Some(LmRegisters::S2), 19 => Some(LmRegisters::S3), 20 => Some(LmRegisters::S4), 21 => Some(LmRegisters::S5), 22 => Some(LmRegisters::S6), 23 => Some(LmRegisters::S7),
        24 => Some(LmRegisters::T8), 25 => Some(LmRegisters::T9), 26 => Some(LmRegisters::K0), 27 => Some(LmRegisters::K1), 28 => Some(LmRegisters::GP), 29 => Some(LmRegisters::SP), 30 => Some(LmRegisters::FP), 31 => Some(LmRegisters::RA),
        _ => None,
    }
}

struct LmInstructionContext{
    pub function: LmInstructionFunction,
    pub format: LmCpuInstructionFormat,
    pub machine_code: u32,
    pub is_relative: bool,
    pub is_region: bool,
    pub is_conditional: bool,
    pub operands: Vec<LmOperand>,
    pub mnemonic_id: LmMnemonicId,
    pub coprocessor: LmCoprocessor,
    pub to_string: fn(mnemonic_id: LmMnemonicId, operands: &Vec<LmOperand>) -> String
}

#[derive(Debug, Copy, Clone)]
pub struct LmDisassembler{
    address_size: LmAddressSize,
}

fn no_to_string_callback(_mnemonic_id: LmMnemonicId, _operands: &Vec<LmOperand>) -> String{
    String::from("")
}

impl LmDisassembler{
    pub fn disassemble(&self, memory: u32, address: u64) -> Option<LmInstruction>{
        const OPCODE_TABLE: [fn (context: &mut LmInstructionContext) -> bool; 64] = [
            special_opcode_table, regimm_opcode_table, j, jal, beq, bne,  blez,  bgtz,
            addi,  addiu,  slti,  sltiu,  andi,  ori,  xori,  lui,
            cop0_opcode_table,  cop1_opcode_table,  cop2_opcode_table,  cop1x_opcode_table,  beql,  bnel,  blezl,  bgtzl,
            empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  jalx,  empty_opcode,  special3_opcode_table,
            lb,  lh,  lwl,  lw,  lbu,  lhu,  lwr,  empty_opcode,
            sb,  sh,  swl,  sw,  empty_opcode,  empty_opcode,  swr,  cache,
            ll,  lwc1,  lwc2,  pref,  empty_opcode, ldc1, ldc2,  empty_opcode,
            sc,  swc1,  swc2,  empty_opcode,  empty_opcode,  sdc1,  sdc2,  empty_opcode];

        let mut context: LmInstructionContext = LmInstructionContext{
            function: LmInstructionFunction::NoFunction,
            format: LmCpuInstructionFormat::NoFormat,
            to_string: no_to_string_callback,
            is_conditional: false,
            coprocessor: match memory >> 26{
                0x20 => LmCoprocessor::CP0,
                0x21 => LmCoprocessor::CP1,
                0x22 => LmCoprocessor::CP2,
                0x23 => LmCoprocessor::CP1X,
                _ => LmCoprocessor::CPU,
            },
            machine_code: memory,
            is_relative: false,
            is_region: false,
            mnemonic_id: LmMnemonicId::NOMNEMONIC,
            operands: Vec::new()
        };
        
        if !OPCODE_TABLE[(memory >> 26) as usize](&mut context){
            return None
        }
        if context.operands.len() == 1 {
            context.to_string = one_operand_to_string;
        }
        else if context.operands.len() == 0{
            context.to_string = no_operand_to_string;
        }
        return match LmInstruction::new_instruction(context.to_string, self.address_size, context.is_conditional, context.coprocessor, address, context.operands, context.mnemonic_id, memory, context.function, context.format, context.is_relative, context.is_region){
            Some(instruction) => Some(instruction),
            None => None,
        };
    }
    pub fn _get_address_size(&self) -> LmAddressSize{
        self.address_size
    }
    fn imm_format(context: &mut LmInstructionContext, coprocessor: LmCoprocessor) -> (){
        context.format = LmCpuInstructionFormat::Imm;
        context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 0b11111).unwrap(), coprocessor));
        context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::CPU));
        context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));    
    }
    fn _reg_format(context: &mut LmInstructionContext) -> (){
        context.format = LmCpuInstructionFormat::Reg;
    }
    fn jump_format(context: &mut LmInstructionContext) -> (){
        context.format = LmCpuInstructionFormat::Jump;
        context.is_region = true;
        context.function = LmInstructionFunction::BranchJump;
        context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0x3FFFFFF) as u64));
    }
}

pub fn new_disassembler(address_size: LmAddressSize) -> LmDisassembler{
    LmDisassembler{
        address_size: address_size
    }
}

//Opcode handlers tables
fn empty_opcode(_context: &mut LmInstructionContext) -> bool{
    false
}
fn special_opcode_table(context: &mut LmInstructionContext) -> bool{
    static SPECIAL_TABLE: [fn(&mut LmInstructionContext) -> bool; 64] = [
    sll,  empty_opcode,  empty_opcode,  sra,  sllv,  empty_opcode,  empty_opcode,  srav,
    jr,  jalr,  movz,  movn,  syscall,  break_inst,  empty_opcode,  sync,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];

    SPECIAL_TABLE[(context.machine_code & 0b11111) as usize](context)
}
fn regimm_opcode_table(context: &mut LmInstructionContext) -> bool{
    static REGIMM_TABLE: [fn(&mut LmInstructionContext) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];

    REGIMM_TABLE[(context.machine_code >> 26) as usize](context)
}
fn special3_opcode_table(context: &mut LmInstructionContext) -> bool{
    static SPECIAL3_TABLE: [fn(&mut LmInstructionContext) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];

    SPECIAL3_TABLE[(context.machine_code >> 26) as usize](context)
}
fn cop0_opcode_table(context: &mut LmInstructionContext) -> bool{
    static COP0_TABLE: [fn(&mut LmInstructionContext) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];
    context.coprocessor = LmCoprocessor::CP0;
    COP0_TABLE[(context.machine_code >> 26) as usize](context)
}
fn cop1_opcode_table(context: &mut LmInstructionContext) -> bool{
    static COP1_TABLE: [fn(&mut LmInstructionContext) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];
    context.coprocessor = LmCoprocessor::CP1;

    COP1_TABLE[(context.machine_code >> 26) as usize](context)
}
fn cop2_opcode_table(context: &mut LmInstructionContext) -> bool{
    static COP2_TABLE: [fn(&mut LmInstructionContext) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];
    context.coprocessor = LmCoprocessor::CP2;

    COP2_TABLE[(context.machine_code >> 26) as usize](context)
}
fn cop1x_opcode_table(context: &mut LmInstructionContext) -> bool{
    static COP1X_TABLE: [fn(&mut LmInstructionContext) -> bool; 64] = [
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,
    empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode,  empty_opcode ];
    context.coprocessor = LmCoprocessor::CP1X;

    COP1X_TABLE[(context.machine_code >> 26) as usize](context)
}

//Opcode handlers
fn j(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::J;
    LmDisassembler::jump_format(context);
    true
}
fn jal(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::J;
    LmDisassembler::jump_format(context);
    true
}
fn beq(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::BEQ;
    context.format = LmCpuInstructionFormat::Imm;
    context.function = LmInstructionFunction::BranchJump;
    context.is_relative = true;
    context.is_conditional = true;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));
    true
}
fn bne(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::BNE;
    context.is_conditional = true;
    context.is_relative = true;
    context.format = LmCpuInstructionFormat::Imm;
    context.function = LmInstructionFunction::BranchJump;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));
    true
}
fn blez(context: &mut LmInstructionContext) -> bool{
    if (context.machine_code >> 16 & 5) != 0{
        return false
    }
    context.is_relative = true;
    context.mnemonic_id = LmMnemonicId::BLEZ;
    context.format = LmCpuInstructionFormat::Imm;
    context.is_conditional = true;
    context.function = LmInstructionFunction::BranchJump;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));
    true
}
fn bgtz(context: &mut LmInstructionContext) -> bool{
    if (context.machine_code >> 16 & 5) != 0{
        return false
    }
    context.is_relative = true;
    context.mnemonic_id = LmMnemonicId::BGTZ;
    context.format = LmCpuInstructionFormat::Imm;
    context.function = LmInstructionFunction::BranchJump;
    context.is_conditional = true;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));
    true
}
fn addi(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::ADDI;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::Computational;
    true
}
fn addiu(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::ADDIU;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::Computational;
    true
}
fn slti(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SLTI;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.is_conditional = true;
    context.function = LmInstructionFunction::Computational;
    true
}
fn sltiu(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SLTIU;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.is_conditional = true;
    context.function = LmInstructionFunction::Computational;
    true
}
fn andi(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::ANDI;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::Computational;
    true
}
fn ori(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::ORI;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::Computational;
    true
}
fn xori(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::XORI;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::Computational;
    true
}
fn lui(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LUI;
    context.format = LmCpuInstructionFormat::Imm;
    context.function = LmInstructionFunction::Computational;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 5).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));
    true
}
fn beql(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::BEQL;
    context.format = LmCpuInstructionFormat::Imm;
    context.is_relative = true;
    context.function = LmInstructionFunction::BranchJump;
    context.is_conditional = true;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));
    true
}
fn bnel(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::BNEL;
    context.format = LmCpuInstructionFormat::Imm;
    context.function = LmInstructionFunction::BranchJump;
    context.is_conditional = true;
    context.is_relative = true;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));
    true
}
fn blezl(context: &mut LmInstructionContext) -> bool{
    if (context.machine_code >> 16 & 5) != 0{
        return false
    }
    context.is_relative = true;
    context.mnemonic_id = LmMnemonicId::BLEZL;
    context.format = LmCpuInstructionFormat::Imm;
    context.function = LmInstructionFunction::BranchJump;
    context.is_conditional = true;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));
    true
}
fn bgtzl(context: &mut LmInstructionContext) -> bool{
    if (context.machine_code >> 16 & 5) != 0{
        return false
    }
    context.is_relative = true;
    context.mnemonic_id = LmMnemonicId::BGTZL;
    context.format = LmCpuInstructionFormat::Imm;
    context.function = LmInstructionFunction::BranchJump;
    context.is_conditional = true;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));
    true
}
fn jalx(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::JALX;
    LmDisassembler::jump_format(context);
    true
}
fn lb(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LB;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn lh(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LH;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn lwl(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LWL;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn lw(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LW;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn lbu(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LBU;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn lhu(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LHU;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn lwr(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LWR;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn sb(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SB;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn sh(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SH;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn swl(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SWL;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn sw(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SW;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn swr(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SWR;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn cache(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::CACHE;
    context.format = LmCpuInstructionFormat::Imm;
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code >> 16 & 0b11111) as u64));    
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::NoCoprocessor));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));  
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn ll(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LL;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn lwc1(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LWC1;
    LmDisassembler::imm_format(context, LmCoprocessor::CP1);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn lwc2(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LWC2;
    LmDisassembler::imm_format(context, LmCoprocessor::CP2);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn pref(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::PREF;
    context.format = LmCpuInstructionFormat::Imm;
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code >> 16 & 0b11111) as u64));    
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111).unwrap(), LmCoprocessor::NoCoprocessor));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code & 0xffff) as u64));  
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn ldc1(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LDC1;
    LmDisassembler::imm_format(context, LmCoprocessor::CP1);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn ldc2(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::LDC2;
    LmDisassembler::imm_format(context, LmCoprocessor::CP2);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn sc(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SC;
    LmDisassembler::imm_format(context, LmCoprocessor::CPU);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn swc1(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SWC1;
    LmDisassembler::imm_format(context, LmCoprocessor::CP1);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn swc2(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SWC2;
    LmDisassembler::imm_format(context, LmCoprocessor::CP2);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn sdc1(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SDC1;
    LmDisassembler::imm_format(context, LmCoprocessor::CP1);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn sdc2(context: &mut LmInstructionContext) -> bool{
    context.mnemonic_id = LmMnemonicId::SDC2;
    LmDisassembler::imm_format(context, LmCoprocessor::CP2);
    context.function = LmInstructionFunction::LoadStore;
    true
}
fn sll(context: &mut LmInstructionContext) -> bool{
    if context.machine_code >> 21 & 0b11111 != 0{
        return false
    }
    context.format = LmCpuInstructionFormat::Reg;
    context.mnemonic_id = LmMnemonicId::SLL;
    context.function = LmInstructionFunction::Computational;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code >> 6 & 0b11111) as u64));
    true
}
fn sra(context: &mut LmInstructionContext) -> bool{
    if context.machine_code >> 21 & 0b11111 != 0{
        return false
    }
    context.format = LmCpuInstructionFormat::Reg;
    context.mnemonic_id = LmMnemonicId::SRA;
    context.function = LmInstructionFunction::Computational;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_imm_opreand((context.machine_code >> 6 & 0b11111) as u64));
    true
}
fn sllv(context: &mut LmInstructionContext) -> bool{
    if context.machine_code >> 6 & 0b11111 != 0{
        return false
    }
    context.format = LmCpuInstructionFormat::Reg;
    context.mnemonic_id = LmMnemonicId::SLLV;
    context.function = LmInstructionFunction::Computational;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::CPU));
    true
}
fn srav(context: &mut LmInstructionContext) -> bool{
    if context.machine_code >> 6 & 0b11111 != 0{
        return false
    }
    context.format = LmCpuInstructionFormat::Reg;
    context.mnemonic_id = LmMnemonicId::SRAV;
    context.function = LmInstructionFunction::Computational;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::CPU));
    true
}
fn jr(context: &mut LmInstructionContext) -> bool{
    if context.machine_code >> 11 & 0b1111111111 != 0{
        return false
    }
    context.to_string = jr_jalr;
    context.function = LmInstructionFunction::BranchJump;
    context.format = LmCpuInstructionFormat::Reg;
    if (context.machine_code >> 6 & 0b11111) == 0b10000{
        context.mnemonic_id = LmMnemonicId::JRHB;
    }
    else{
        context.mnemonic_id = LmMnemonicId::JR;
    }
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::CPU));
    true
}
fn jalr(context: &mut LmInstructionContext) -> bool{
    if context.machine_code >> 16 & 0b11111 != 0{
        return false
    }
    context.to_string = jr_jalr;
    context.function = LmInstructionFunction::BranchJump;
    context.format = LmCpuInstructionFormat::Reg;
    if (context.machine_code >> 6 & 0b11111) == 0b10000{
        context.mnemonic_id = LmMnemonicId::JALRHB;
    }
    else{
        context.mnemonic_id = LmMnemonicId::JALR;
    }
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::CPU));
    true
}
fn movz(context: &mut LmInstructionContext) -> bool{
    if context.machine_code >> 6 & 0b11111 != 0{
        return false
    }
    context.function = LmInstructionFunction::Miscellaneous;
    context.is_conditional = true;
    context.format = LmCpuInstructionFormat::Reg;
    context.mnemonic_id = LmMnemonicId::MOVZ;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::CPU));
    true
}
fn movn(context: &mut LmInstructionContext) -> bool{
    if context.machine_code >> 6 & 0b11111 != 0{
        return false
    }
    context.function = LmInstructionFunction::Miscellaneous;
    context.is_conditional = true;
    context.format = LmCpuInstructionFormat::Reg;
    context.mnemonic_id = LmMnemonicId::MOVN;
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 11 & 0b11111).unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 21 & 0b11111) .unwrap(), LmCoprocessor::CPU));
    context.operands.push(LmOperand::new_reg_opreand(u32_to_register(context.machine_code >> 16 & 0b11111).unwrap(), LmCoprocessor::CPU));
    true
}
fn syscall(context: &mut LmInstructionContext) -> bool{
    context.function = LmInstructionFunction::Miscellaneous;
    context.format = LmCpuInstructionFormat::Other;
    context.mnemonic_id = LmMnemonicId::SYSCALL;
    context.operands.push(LmOperand::new_imm_opreand(((context.machine_code >> 6) & 0xFFFFF) as u64));
    true
}
fn break_inst(context: &mut LmInstructionContext) -> bool{
    context.function = LmInstructionFunction::Miscellaneous;
    context.format = LmCpuInstructionFormat::Other;
    context.mnemonic_id = LmMnemonicId::BREAK;
    // context.operands.push(LmOperand::new_imm_opreand(((context.machine_code >> 6) & 0xFFFFF) as u64));
    true
}
fn sync(context: &mut LmInstructionContext) -> bool{
    if (context.machine_code >> 11 & 0xffff) != 0{
        return false
    }
    context.function = LmInstructionFunction::Miscellaneous;
    context.format = LmCpuInstructionFormat::Other;
    context.mnemonic_id = LmMnemonicId::SYNC;
    context.operands.push(LmOperand::new_imm_opreand(((context.machine_code >> 6) & 0xFFFFF) as u64));
    true
}
