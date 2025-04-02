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
impl LmDisassembler{
    pub (super) fn no_instructions(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        Err(LmError::throw_error(LmErrorCode::NoInstruction, context.opcode, context.address, context.machine_code))
    }
    pub (super) fn special_opcode_map(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        static SPECIAL_MAP: [fn(disassembler: &LmDisassembler, &mut LmInstructionContext) -> Result<(), LmError>; 64] = [
        LmDisassembler::sll,  LmDisassembler::movci,  LmDisassembler::srl_sra,  LmDisassembler::srl_sra,  LmDisassembler::sllv,  LmDisassembler::no_instructions,  LmDisassembler::srlv_srav,  LmDisassembler::srlv_srav,
        LmDisassembler::jr,  LmDisassembler::jalr,  LmDisassembler::movn_movz,  LmDisassembler::movn_movz,  LmDisassembler::syscall_break,  LmDisassembler::syscall_break,  LmDisassembler::no_instructions,  LmDisassembler::sync,
        LmDisassembler::mfhi_mflo,  LmDisassembler::mthi_mtlo,  LmDisassembler::mfhi_mflo,  LmDisassembler::mthi_mtlo,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
        LmDisassembler::mult_multu_div_divu,  LmDisassembler::mult_multu_div_divu,  LmDisassembler::mult_multu_div_divu,  LmDisassembler::mult_multu_div_divu,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
        LmDisassembler::add_addu_sub_subu_and_or_xor_nor,  LmDisassembler::add_addu_sub_subu_and_or_xor_nor,  LmDisassembler::add_addu_sub_subu_and_or_xor_nor,  LmDisassembler::add_addu_sub_subu_and_or_xor_nor,  LmDisassembler::add_addu_sub_subu_and_or_xor_nor,  LmDisassembler::add_addu_sub_subu_and_or_xor_nor,  LmDisassembler::add_addu_sub_subu_and_or_xor_nor,  LmDisassembler::add_addu_sub_subu_and_or_xor_nor,
        LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::slt_sltu,  LmDisassembler::slt_sltu,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
        LmDisassembler::tge_tgeu_tlt_tltu,  LmDisassembler::tge_tgeu_tlt_tltu,  LmDisassembler::tge_tgeu_tlt_tltu,  LmDisassembler::tge_tgeu_tlt_tltu,  LmDisassembler::teq_tne,  LmDisassembler::no_instructions,  LmDisassembler::teq_tne,  LmDisassembler::no_instructions,
        LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions ];

        SPECIAL_MAP[(context.machine_code & 0b111111) as usize](self, context)
    }
    pub (super) fn regimm_opcode_map(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let imm_order: usize;
        let rs: Option<FieldInfos>;
        static MENMONICS: [[Option<&str>; 8]; 4] =
        [   [Some(LM_MNE_BLTZ),  Some(LM_MNE_BGEZ),  Some(LM_MNE_BLTZL),  Some(LM_MNE_BGEZL),  None,  None,  None,  None],
            [Some(LM_MNE_TGEI),  Some(LM_MNE_TGEIU),  Some(LM_MNE_TLTI),  Some(LM_MNE_TLTIU),  Some(LM_MNE_TEQI),  None,  Some(LM_MNE_TNEI),  None],
            [Some(LM_MNE_BLTZAL),  Some(LM_MNE_BGEZAL),  Some(LM_MNE_BLTZALL),  Some(LM_MNE_BGEZALL),  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  Some(LM_MNE_SYNCI)] ];
        
        context.mnemonic = MENMONICS[(context.machine_code >> 19 & 0b11) as usize][(context.machine_code >> 16 & 0b111) as usize];
        context.category = Some(match context.machine_code >> 19 & 3{
            3 => LmInstructionCategory::MemoryControl,
            1 => {
                context.is_conditional = true;
                LmInstructionCategory::Trap
            },
            _ => {
                context.is_relative = true;
                context.is_conditional = true;
                LmInstructionCategory::BranchJump
            },
        });

        if (context.machine_code >> 16 & 0b111111) == 0x11
        && (context.machine_code >> 21 & 0b11111) == 0{
            context.mnemonic = Some(LM_MNE_BAL);
            rs = None;
            imm_order = 0;
            context.is_conditional = false;
        }
        else if (context.machine_code >> 16 & 0b111111) == 0x1f{
            imm_order = 0;
            rs = Some(FieldInfos::default_reg_field(1, LmCoprocessor::Cpu));
        }
        else{
            imm_order = 1;
            rs = Some(FieldInfos::default_reg_field(0, LmCoprocessor::Cpu));
        }

        return LmDisassembler::imm_format(self, context, rs, None, FieldInfos::default_imm_field(imm_order))
    }
    pub (super) fn special2_opcode_map(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        static SPECIAL2_MAP: [fn(disassembler: &LmDisassembler, &mut LmInstructionContext) -> Result<(), LmError>; 64] = 
            [   LmDisassembler::madd_maddu,  LmDisassembler::madd_maddu,  LmDisassembler::mul,  LmDisassembler::no_instructions,  LmDisassembler::msub_msubu,  LmDisassembler::msub_msubu,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::clz_clo,  LmDisassembler::clz_clo,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::sdbbp ];
        SPECIAL2_MAP[(context.machine_code & 0b111111) as usize](self, context)
    }
    pub (super) fn special3_opcode_map(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        static SPECIAL3_MAP: [fn(disassembler: &LmDisassembler, &mut LmInstructionContext) -> Result<(), LmError>; 64] = 
            [   LmDisassembler::ext,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::ins,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::bshfl,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::rdhwr,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions ];
        
        SPECIAL3_MAP[(context.machine_code & 0b111111) as usize](self, context)
    }
    pub (super) fn cop0_opcode_map(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        static COP0_MAP: [fn(disassembler: &LmDisassembler, &mut LmInstructionContext) -> Result<(), LmError>; 32] =
            [   LmDisassembler::mov_cp0,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::mov_cp0,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
                LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::gpr_shadowset,  LmDisassembler::mfmc0,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::gpr_shadowset,  LmDisassembler::no_instructions,
                LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0,
                LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0,  LmDisassembler::c0];
        // unimplemented!("[-]Opcode map isn't implemented yet!");
        // context.coprocessor = LmCoprocessor::Cp0;
        COP0_MAP[(context.machine_code >> 21 & 0b11111) as usize](self, context)
    }
    pub (super) fn cop1_opcode_map(&self, _instruction: &mut LmInstructionContext) -> Result<(), LmError>{
        static _COP1_MAP: [fn(disassembler: &LmDisassembler, &mut LmInstructionContext) -> Result<(), LmError>; 64] =
        [   LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions ];
        unimplemented!("[-]Opcode map isn't implemented yet!");

        // COP1_MAP[(context.machine_code >> 26) as usize](context)
    }
    pub (super) fn cop2_opcode_map(&self, _instruction: &mut LmInstructionContext) -> Result<(), LmError>{
        static _COP2_MAP: [fn(disassembler: &LmDisassembler, &mut LmInstructionContext) -> Result<(), LmError>; 64] = 
        [   LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions ];
        unimplemented!("[-]Opcode map isn't implemented yet!");

        // context.coprocessor = LmCoprocessor::Cp2;
        // COP2_MAP[(context.machine_code >> 26) as usize](context)
    }
    pub (super) fn cop1x_opcode_map(&self, _instruction: &mut LmInstructionContext) -> Result<(), LmError>{
        static _COP1X_MAP: [fn(disassembler: &LmDisassembler, &mut LmInstructionContext) -> Result<(), LmError>; 64] = 
        [   LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,
            LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions,  LmDisassembler::no_instructions ];
        unimplemented!("[-]Opcode map isn't implemented yet!");

        // context.coprocessor = LmCoprocessor::Cp1x;
        // _COP1X_MAP[(context.machine_code >> 26) as usize](context)
    }
    pub (super) fn pcrel_opcode_map(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        static PCREL_MAP: [fn(disassembler: &LmDisassembler, &mut LmInstructionContext) -> Result<(), LmError>; 32] =[   
            LmDisassembler::addiupc,  LmDisassembler::addiupc,  LmDisassembler::lwpc,  LmDisassembler::lwpc,  LmDisassembler::lwupc,  LmDisassembler::lwupc,  LmDisassembler::ldpc,  LmDisassembler::no_instructions,
            LmDisassembler::addiupc,  LmDisassembler::addiupc,  LmDisassembler::lwpc,  LmDisassembler::lwpc,  LmDisassembler::lwupc,  LmDisassembler::lwupc,  LmDisassembler::ldpc,  LmDisassembler::no_instructions,
            LmDisassembler::addiupc,  LmDisassembler::addiupc,  LmDisassembler::lwpc,  LmDisassembler::lwpc,  LmDisassembler::lwupc,  LmDisassembler::lwupc,  LmDisassembler::ldpc,  LmDisassembler::auipc,
            LmDisassembler::addiupc,  LmDisassembler::addiupc,  LmDisassembler::lwpc,  LmDisassembler::lwpc,  LmDisassembler::lwupc,  LmDisassembler::lwupc,  LmDisassembler::ldpc,  LmDisassembler::aluipc
        ];

        context.is_relative = true;
        context.format = Some(LmInstructionFormat::Imm);
        let imm = FieldInfos::imm_field(1, 0b1111111111111111);
        let rs = Some(FieldInfos::default_reg_field(0, LmCoprocessor::Cpu));
        if let Err(e) = PCREL_MAP[(context.machine_code >> 16 & 0b11111) as usize](self, context){
            return Err(e)
        }
        self.imm_format(context, rs, None, imm)
    }

    //Opcode handlers

    //Default opcode field handlers
    pub(super) fn j(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.mnemonic = Some(LM_MNE_J);
        LmDisassembler::jump_format(self, context)
    }
    pub(super) fn jal(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.mnemonic = Some(LM_MNE_JAL);
        LmDisassembler::jump_format(self, context)
    }
    pub(super) fn beq(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    

        context.is_relative = true;
        context.category = Some(LmInstructionCategory::BranchJump);
        context.mnemonic = Some(LM_MNE_BEQ);
        context.is_conditional = true;
        
        return LmDisassembler::imm_format(self, context, Some(rs), Some(rt), FieldInfos::default_imm_field(2));
    }
    pub(super) fn bne(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    

        context.is_relative = true;
        context.category = Some(LmInstructionCategory::BranchJump);
        context.mnemonic = Some(LM_MNE_BNE);
        context.is_conditional = true;
        
        return LmDisassembler::imm_format(self, context, Some(rs), Some(rt), FieldInfos::default_imm_field(2));
    }
    pub(super) fn blez(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.is_relative = true;
        context.mnemonic = Some(LM_MNE_BLEZ);
        context.is_conditional = true;
        context.category = Some(LmInstructionCategory::BranchJump);
        let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        return LmDisassembler::imm_format(self, context, Some(rs), Some(FieldInfos::default_blank_field()), FieldInfos::default_imm_field(1));
    }
    pub(super) fn bgtz(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.is_relative = true;
        context.mnemonic = Some(LM_MNE_BGTZ);
        context.category = Some(LmInstructionCategory::BranchJump);
        context.is_conditional = true;
        let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        return LmDisassembler::imm_format(self, context, Some(rs), Some(FieldInfos::default_blank_field()), FieldInfos::default_imm_field(1));
    }
    pub(super) fn addi_addiu(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        context.mnemonic = match context.machine_code >> 26 & 1 {
            1 => Some(LM_MNE_ADDIU),
            0 => {
                Some(LM_MNE_ADDI)
            }
            _ => None
        };
        context.category = Some(LmInstructionCategory::Arithmetic);
        return LmDisassembler::imm_format(self, context, Some(rs), Some(rt), sa);
    }
    pub(super) fn slti_sltiu(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        context.category = Some(LmInstructionCategory::Arithmetic);
        context.mnemonic = match context.machine_code >> 26 & 1 {
            1 => Some(LM_MNE_SLTIU),
            0 => {
                Some(LM_MNE_SLTI)
            }
            _ => None
        };


        return LmDisassembler::imm_format(self, context, Some(rs), Some(rt), sa);
    }
    pub(super) fn andi(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        context.mnemonic = Some(LM_MNE_ANDI);
        context.category = Some(LmInstructionCategory::Logical);

        return LmDisassembler::imm_format(self, context, Some(rs), Some(rt), sa);
    }
    pub(super) fn ori(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        context.mnemonic = Some(LM_MNE_ORI);
        context.category = Some(LmInstructionCategory::Logical);
        
        return LmDisassembler::imm_format(self, context, Some(rs), Some(rt), sa);
    }
    pub(super) fn xori(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(2);

        context.mnemonic = Some(LM_MNE_XORI);
        context.category = Some(LmInstructionCategory::Logical);
        
        return LmDisassembler::imm_format(self, context, Some(rs), Some(rt), sa);
    }
    pub(super) fn lui(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rt: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        let sa: FieldInfos = FieldInfos::default_imm_field(1);

        context.mnemonic = Some(LM_MNE_LUI);
        context.category = Some(LmInstructionCategory::Logical);

        return LmDisassembler::imm_format(self, context, Some(FieldInfos::default_blank_field()), Some(rt), sa);
    }
    pub(super) fn beql(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
        let imm: FieldInfos = FieldInfos::default_imm_field(2);

        context.is_relative = true;
        context.category = Some(LmInstructionCategory::BranchJump);
        context.mnemonic = Some(LM_MNE_BEQL);
        context.is_conditional = true;
        
        return LmDisassembler::imm_format(self, context, Some(rs), Some(rt), imm);
    }
    pub(super) fn bnel(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        let rt: FieldInfos = FieldInfos::default_reg_field(1, LmCoprocessor::Cpu);    
        let imm: FieldInfos = FieldInfos::default_imm_field(2);

        context.is_relative = true;
        context.category = Some(LmInstructionCategory::BranchJump);
        context.mnemonic = Some(LM_MNE_BNEL);
        context.is_conditional = true;
        
        return LmDisassembler::imm_format(self, context, Some(rs), Some(rt), imm);
    }
    pub(super) fn blezl(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.is_relative = true;
        context.mnemonic = Some(LM_MNE_BLEZL);
        context.category = Some(LmInstructionCategory::BranchJump);
        context.is_conditional = true;
        let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        return LmDisassembler::imm_format(self, context, Some(rs), Some(FieldInfos::default_blank_field()), FieldInfos::default_imm_field(1));
    }
    pub(super) fn bgtzl(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.is_relative = true;
        context.mnemonic = Some(LM_MNE_BGTZL);
        context.category = Some(LmInstructionCategory::BranchJump);
        context.is_conditional = true;
        let rs: FieldInfos = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);    
        return LmDisassembler::imm_format(self, context, Some(rs), Some(FieldInfos::default_blank_field()), FieldInfos::default_imm_field(1));
    }
    pub(super) fn jalx(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.mnemonic = Some(LM_MNE_JALX);
        LmDisassembler::jump_format(self, context)
    }
    pub(super) fn cpu_loadstore(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, LmCoprocessor::Cpu);
        let rt: FieldInfos;
        let mnemonics: [[Option<&str>; 7]; 4] = [
            [Some(LM_MNE_LB), Some(LM_MNE_LH), Some(LM_MNE_LWL), Some(LM_MNE_LW), Some(LM_MNE_LBU), Some(LM_MNE_LHU), Some(LM_MNE_LWR)],
            [Some(LM_MNE_SB), Some(LM_MNE_SH), Some(LM_MNE_SWL), Some(LM_MNE_SW), None, None, Some(LM_MNE_SWR)],
            [Some(LM_MNE_LL), Some(LM_MNE_LWC1), Some(LM_MNE_LWC2), None, None, Some(LM_MNE_LDC1), Some(LM_MNE_LDC2)],
            [Some(LM_MNE_SC), Some(LM_MNE_SWC1), Some(LM_MNE_SWC2), None, None, Some(LM_MNE_SDC1), Some(LM_MNE_SDC2)]
        ];

        context.mnemonic = mnemonics[(context.machine_code >> 29 & 3) as usize][(context.machine_code >> 26 & 7) as usize];

        if (context.machine_code >> 29 & 3) == 6 
        || (context.machine_code >> 29 & 3) == 7
        && (context.machine_code >> 27 & 3) == 1{
            rt = FieldInfos::default_reg_field(0, LmCoprocessor::Cp2);
        }
        else if (context.machine_code >> 29 & 3) == 6 
        || (context.machine_code >> 29 & 3) == 7
        && (context.machine_code >> 27 & 3) == 0{
            rt = FieldInfos::default_reg_field(0, LmCoprocessor::Cp1);
        }
        else {
            rt = FieldInfos::default_reg_field(0, LmCoprocessor::Cpu);
        }

        context.category = Some(match context.machine_code & 1{
            0 => LmInstructionCategory::Load,
            1 => LmInstructionCategory::Store,
            _ => return Err(LmError::throw_error(LmErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        });

        return LmDisassembler::imm_format(self, context, Some(base), Some(rt), FieldInfos::default_imm_field(1))
    }
    pub(super) fn cache_pref(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let base: FieldInfos = FieldInfos::default_reg_field(2, LmCoprocessor::Cpu);
        let op: FieldInfos = FieldInfos::imm_field(0, 0b11111);
        
        context.mnemonic = match context.machine_code >> 26 & 4{
            4 =>     {
                context.category = Some(LmInstructionCategory::MemoryControl);
                Some(LM_MNE_PREF)
            },
            0 => {
                context.category = Some(LmInstructionCategory::Priviledge);
                Some(LM_MNE_CACHE)
            },
            _ => None
        };
        return LmDisassembler::imm_format(self, context, Some(base), Some(op), FieldInfos::default_imm_field(1));
    }

    //Special
    pub(super) fn sll(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::default_blank_field();
        let rt: FieldInfos;
        let rd: FieldInfos;
        let sa: FieldInfos;

        if context.machine_code >> 11 & 0b111111111111111 == 0{
            context.mnemonic = match context.machine_code >> 6 & 0b11111{
                0 => Some(LM_MNE_NOP),
                1 => Some(LM_MNE_SSNOP),
                3 => Some(LM_MNE_EHB),
                5 => Some(LM_MNE_PAUSE),
                _ => return Err(LmError::throw_error(LmErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
            };
            rt = FieldInfos::default_blank_field();
            rd = FieldInfos::default_blank_field();
            sa = FieldInfos::default_blank_field();

            context.category = Some(LmInstructionCategory::Control);

        }
        else{
            context.mnemonic = Some(LM_MNE_SLL);
            context.category = Some(LmInstructionCategory::Shift);

            rt = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
            rd = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
            sa = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Imm);
        }
        
        LmDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn movci(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        //Reserved Instruction, Coprocessor Unusable
        if (context.machine_code >> 6 & 0b11111) != 0
        ||(context.machine_code >> 17 & 1) != 0{
            return Err(LmError::throw_error(LmErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        }
        let mnemonics = [Some(LM_MNE_MOVF), Some(LM_MNE_MOVT)];
        let mut hex_num: LmString = LmString::new_lmstring();
        let comma: &str = ", ";
        let registers: [&str; 8] = [ LM_REG_FCC0, LM_REG_FCC1, LM_REG_FCC2, LM_REG_FCC3, LM_REG_FCC4, LM_REG_FCC5, LM_REG_FCC6, LM_REG_FCC7,];
        
        context.format = Some(LmInstructionFormat::CoditionCodeFpu);
        context.category = Some(LmInstructionCategory::Move);
        context.mnemonic = mnemonics[(context.machine_code >> 16 & 1) as usize];

        context.operand_num = 3;
        context.operand[0] = Some(LmOpRegister::new_reg_opreand((context.machine_code >> 11 & 0b11111) as u8, LmCoprocessor::Cpu));
        context.operand[1] = Some(LmOpRegister::new_reg_opreand((context.machine_code >> 21 & 0b11111) as u8, LmCoprocessor::Cpu));
        context.operand[2] = Some(LmOpRegister::new_reg_operand_str(registers[(context.machine_code >> 18 & 0b111) as usize], LmCoprocessor::Cp1));

        if let Some(mne) = context.mnemonic{
            context.string.append_str(mne);
        }
        context.string.append_char(' ');
        for i in 0..context.operand_num{
            if let Some(op) = context.operand[i]{
                match op{
                    LmOperand::LmOpRegister(reg) => _= context.string.append_str(reg.get_register()),
                    LmOperand::LmOpImmediate(imm) => {
                        hex_num.num_to_str(imm.get_value());
                        context.string.append_string(&hex_num);
                    },
                };
                if context.operand_num - 1 > i{
                    context.string.append_str(&comma);
                }
            }
        }
        Ok(())
    }
    pub(super) fn srl_sra(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::blank_field(0b1111);
        let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let sa: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Imm);

        context.mnemonic = match context.machine_code & 1{
            1 => Some(LM_MNE_SRA),
            0 => {
                match context.machine_code >> 6 & 1 {
                    1 => Some(LM_MNE_ROTR),
                    0 => Some(LM_MNE_SRL),
                    _ => None
                }
            },
            _ => None
        };

        context.category = Some(LmInstructionCategory::Shift);
        return LmDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn sllv(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.mnemonic = Some(LM_MNE_SLLV);
        context.category = Some(LmInstructionCategory::Shift);

        let sa: FieldInfos = FieldInfos::default_blank_field();
        let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Reg);

        return LmDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn srlv_srav(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let sa: FieldInfos = FieldInfos::blank_field(0b1111);
        let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Imm);

        context.mnemonic = match context.machine_code & 1{
            1 => Some(LM_MNE_SRAV),
            0 => {
                match context.machine_code >> 6 & 1 {
                    1 => Some(LM_MNE_ROTRV),
                    0 => Some(LM_MNE_SRLV),
                    _ => None
                }
            },
            _ => None
        };

        context.category = Some(LmInstructionCategory::Shift);
        return LmDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(sa))
    }
    pub(super) fn jr(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rd: FieldInfos = FieldInfos::blank_field(0b1111111111);
        let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);

        context.category = Some(LmInstructionCategory::BranchJump);

        if (context.machine_code >> 6 & 0b10000) != 0{
            context.mnemonic = Some(LM_MNE_JRHB);
        }
        else{
            context.mnemonic = Some(LM_MNE_JR);
        }

        LmDisassembler::reg_format(self, context, Some(rs), None, Some(rd), None)
    }
    pub(super) fn jalr(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rt: FieldInfos = FieldInfos::default_blank_field();
        let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        
        context.category = Some(LmInstructionCategory::BranchJump);

        if (context.machine_code >> 6 & 0b10000) != 0{
            context.mnemonic = Some(LM_MNE_JALRHB)
        }
        else{
            context.mnemonic = Some(LM_MNE_JALR)
        }

        LmDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), None)
    }
    pub(super) fn movn_movz(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);

        context.category = Some(LmInstructionCategory::Move);
        context.is_conditional = true;

        if context.machine_code & 0b111111 == 0b001010{
            context.mnemonic = Some(LM_MNE_MOVZ);
        }
        else{
            context.mnemonic = Some(LM_MNE_MOVN);
        }
        return LmDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
    }
    pub(super) fn syscall_break(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let mut hex_num: LmString = LmString::new_lmstring();

        context.mnemonic = match context.machine_code & 1{
            1 => Some(LM_MNE_BREAK),
            0 => Some(LM_MNE_SYSCALL),
            _ => None
        };
        context.category = Some(LmInstructionCategory::Trap);
        context.format = Some(LmInstructionFormat::Other);
        context.operand[0] = Some(LmOpImmediate::new_imm_opreand(((context.machine_code >> 6) & 0xFFFFF) as u64));

        if let Some(LmOperand::LmOpImmediate(imm)) = context.operand[0]{
            hex_num.num_to_str(imm.get_value());
        };
        if let Some(mne) = context.mnemonic{
            context.string.append_str(mne);
            context.string.append_char(' ');
            context.string.append_string(&hex_num);
        }
        Ok(())
    }
    pub(super) fn sync(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rd: FieldInfos = FieldInfos::blank_field(0b111111111111111);
        let sa: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Imm);

        //Setting the attributes
        context.mnemonic = Some(LM_MNE_SYNC);
        context.category = Some(LmInstructionCategory::MemoryControl);
        LmDisassembler::reg_format(self, context, None, None, Some(rd), Some(sa))
    }
    pub(super) fn mfhi_mflo(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let mnemonics = [Some(LM_MNE_MFHI), Some(LM_MNE_MFLO)];

        context.mnemonic = mnemonics[(context.machine_code >> 1 & 1) as usize];
        context.category = Some(LmInstructionCategory::Move);

        LmDisassembler::reg_format(self, context, None, Some(FieldInfos::blank_field(0b1111111111)), Some(rd), Some(FieldInfos::default_blank_field()))
    }
    pub(super) fn mthi_mtlo(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let mnemonics = [Some(LM_MNE_MTHI), Some(LM_MNE_MTLO)];
        
        context.mnemonic = mnemonics[(context.machine_code >> 1 & 1) as usize];
        context.category = Some(LmInstructionCategory::Move);

        LmDisassembler::reg_format(self, context, Some(rs), None, None, Some(FieldInfos::blank_field(0b111111111111111)))
    }
    pub(super) fn mult_multu_div_divu(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let mnemonics = [[Some(LM_MNE_MULT), Some(LM_MNE_MULTU)], [Some(LM_MNE_DIV), Some(LM_MNE_DIVU)]];

        context.category = Some(LmInstructionCategory::Arithmetic);
        context.mnemonic = mnemonics[(context.machine_code >> 1 & 1) as usize][(context.machine_code & 1) as usize];

        LmDisassembler::reg_format(self, context, Some(rs), Some(rt), None, Some(FieldInfos::blank_field(0b1111111111)))
    }
    pub(super) fn add_addu_sub_subu_and_or_xor_nor(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let mnemonics = [[[Some(LM_MNE_ADD), Some(LM_MNE_ADDU)], [Some(LM_MNE_SUB), Some(LM_MNE_SUBU)]], [[Some(LM_MNE_AND), Some(LM_MNE_OR)], [Some(LM_MNE_XOR), Some(LM_MNE_NOR)]]];

        context.mnemonic = mnemonics[(context.machine_code >> 2 & 1) as usize][(context.machine_code >> 1 & 1) as usize][(context.machine_code & 1) as usize];
        if (context.machine_code >> 2 & 1) == 1{
            context.category = Some(LmInstructionCategory::Logical);
        }
        else{
            context.category = Some(LmInstructionCategory::Arithmetic);
            if (context.machine_code & 1) == 0{
            }
        }

        LmDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
    }
    pub(super) fn slt_sltu(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let mnemonics = [Some(LM_MNE_SLT), Some(LM_MNE_SLTU)];

        context.category = Some(LmInstructionCategory::Arithmetic);
        context.is_conditional = true;
        context.mnemonic = mnemonics[(context.machine_code & 1) as usize];

        LmDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
    }
    pub(super) fn tge_tgeu_tlt_tltu(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let mnemonics = [[Some(LM_MNE_TGE), Some(LM_MNE_TGEU)], [Some(LM_MNE_TLT), Some(LM_MNE_TLTU)]];
        
        context.mnemonic = mnemonics[(context.machine_code >> 1 & 1) as usize][(context.machine_code & 1) as usize];
        context.category = Some(LmInstructionCategory::Trap);

        LmDisassembler::reg_format(self, context, Some(rs), Some(rt), None, Some(FieldInfos::imm_field(2, 0b1111111111)))
    }
    pub(super) fn teq_tne(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        
        context.category = Some(LmInstructionCategory::Trap);
        context.mnemonic = match context.machine_code >> 1 & 1{
            1 => Some(LM_MNE_TEQ),
            0 => Some(LM_MNE_TNE),
            _ => None
        };

        LmDisassembler::reg_format(self, context, Some(rs), Some(rt), None, Some(FieldInfos::imm_field(2, 0b1111111111)))
    }

    //Special2
    pub(super) fn madd_maddu(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);

        context.category = Some(LmInstructionCategory::Arithmetic);
        context.mnemonic = match context.machine_code & 1{
            0 => Some(LM_MNE_MADD),
            1 => Some(LM_MNE_MADDU),
            _ => None
        };

        LmDisassembler::reg_format(self, context, Some(rs), Some(rt), None, Some(FieldInfos::blank_field(0b1111111111)))
    }
    pub(super) fn mul(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(2, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);

        context.category = Some(LmInstructionCategory::Arithmetic);
        context.mnemonic = Some(LM_MNE_MUL);

        LmDisassembler::reg_format(self, context, Some(rs), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
    }
    pub(super) fn msub_msubu(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);

        context.category = Some(LmInstructionCategory::Arithmetic);
        context.mnemonic = match context.machine_code & 1{
            0 => Some(LM_MNE_MSUB),
            1 => Some(LM_MNE_MSUBU),
            _ => None
        };

        LmDisassembler::reg_format(self, context, Some(rs), Some(rt), None, Some(FieldInfos::blank_field(0b1111111111)))
    }
    pub(super) fn clz_clo(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rs: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);

        context.category = Some(LmInstructionCategory::Arithmetic);
        context.mnemonic = match context.machine_code & 1{
            0 => Some(LM_MNE_CLZ),
            1 => Some(LM_MNE_CLO),
            _ => None
        };
        let success = LmDisassembler::reg_format(self, context, Some(rs), None, Some(rd), Some(FieldInfos::default_blank_field()));
        
        return success
    }
    pub(super) fn sdbbp(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let mut hex_num: LmString = LmString::new_lmstring();

        context.mnemonic = Some(LM_MNE_SDBBP);
        context.category = Some(LmInstructionCategory::Trap);
        context.format = Some(LmInstructionFormat::Other);
        context.operand[0] = Some(LmOpImmediate::new_imm_opreand(((context.machine_code >> 6) & 0xFFFFF) as u64));

        if let Some(LmOperand::LmOpImmediate(imm)) = context.operand[0]{
            hex_num.num_to_str(imm.get_value());
        };
        if let Some(mne) = context.mnemonic{
            context.string.append_str(mne);
            context.string.append_char(' ');
            context.string.append_string(&hex_num);
        }
        Ok(())
    }

    //Special3 They need some testing
    pub(super) fn ext(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let mut hex_num: LmString = LmString::new_lmstring();

        context.mnemonic = Some(LM_MNE_EXT);
        context.category = Some(LmInstructionCategory::InsertExtract);

        let success = LmDisassembler::reg_format(self, context, Some(rs), Some(rt), None, None);

        context.operand_num = 4;
        context.operand[2] = Some(LmOpImmediate::new_imm_opreand((context.machine_code >> 6 & 0b11111) as u64));
        context.operand[3] = Some(LmOpImmediate::new_imm_opreand((context.machine_code >> 11 & 0b11111) as u64));
        
        context.string.append_str(", ");
        if let Some(LmOperand::LmOpImmediate(imm)) = context.operand[2]{
            hex_num.num_to_str(imm.get_value());
            context.string.append_string(&hex_num);
        }
        context.string.append_str(", ");
        if let Some(LmOperand::LmOpImmediate(imm)) = context.operand[3]{
            hex_num.num_to_str(imm.get_value());
            context.string.append_string(&hex_num);
        }
        return success
    }
    pub(super) fn ins(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rs: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        let mut hex_num: LmString = LmString::new_lmstring();

        context.mnemonic = Some(LM_MNE_INS);
        context.category = Some(LmInstructionCategory::InsertExtract);

        let success = LmDisassembler::reg_format(self, context, Some(rs), Some(rt), None, None);

        context.operand_num = 4;
        context.operand[2] = Some(LmOpImmediate::new_imm_opreand((context.machine_code >> 6 & 0b11111) as u64));
        context.operand[3] = Some(LmOpImmediate::new_imm_opreand((context.machine_code >> 11 & 0b11111) as u64));
        
        context.string.append_str(", ");
        if let Some(LmOperand::LmOpImmediate(imm)) = context.operand[3]{
            if let Some(LmOperand::LmOpImmediate(imm1)) = context.operand[2]{
                hex_num.num_to_str(imm.get_value() - imm1.get_value() + 1);
                context.string.append_string(&hex_num);
            }
        }
        context.string.append_str(", ");
        if let Some(LmOperand::LmOpImmediate(imm)) = context.operand[3]{
            hex_num.num_to_str(imm.get_value());
            context.string.append_string(&hex_num);
        }
        return success
    }
    pub(super) fn bshfl(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rd: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rt: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);

        context.mnemonic = match context.machine_code >> 6 & 0b11111{
            0b00010 => {
                context.category = Some(LmInstructionCategory::InsertExtract);
                Some(LM_MNE_WSBH)},
            0b10000 => {
                context.category = Some(LmInstructionCategory::Arithmetic);
                Some(LM_MNE_SEB)},
            0b11000 => {
                context.category = Some(LmInstructionCategory::Arithmetic);
                Some(LM_MNE_SEH)},
            _ => return Err(LmError::throw_error(LmErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        };
        
        LmDisassembler::reg_format(self, context, Some(FieldInfos::default_blank_field()), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
    }
    pub(super) fn rdhwr(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let rt: FieldInfos = FieldInfos::reg_field(0, LmCoprocessor::Cpu, LmOperandType::Reg);
        let rd: FieldInfos = FieldInfos::reg_field(1, LmCoprocessor::Cpu, LmOperandType::Reg);
        
        context.category = Some(LmInstructionCategory::Move);
        context.mnemonic = Some(LM_MNE_RDHWR);

        LmDisassembler::reg_format(self, context, Some(FieldInfos::default_blank_field()), Some(rt), Some(rd), Some(FieldInfos::default_blank_field()))
    }

    //CP0
    pub(super) fn mov_cp0(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let mnemonics = [Some(LM_MNE_MFC0), Some(LM_MNE_MTC0)];
        if (context.machine_code >> 3 & 0b11111111) != 0{
            return Err(LmError::throw_error(LmErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        }

        context.category = Some(LmInstructionCategory::Priviledge);
        context.format = Some(LmInstructionFormat::Other);
        context.mnemonic = mnemonics[(context.machine_code >> 23 & 1) as usize];
        context.operand_num = 3;

        context.operand[0] = Some(LmOpRegister::new_reg_opreand((context.machine_code >> 16 & 0b11111) as u8, LmCoprocessor::Cpu));
        context.operand[1] = Some(LmOpRegister::new_reg_opreand((context.machine_code >> 11 & 0b11111) as u8, LmCoprocessor::Cpu));
        context.operand[2] = Some(LmOpImmediate::new_imm_opreand((context.machine_code & 7) as u64));

        LmDisassembler::basic_str_format(context)
    }
    pub(super) fn gpr_shadowset(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let mnemonics = [Some(LM_MNE_RDPGPR), Some(LM_MNE_WRPGPR)];

        context.category = Some(LmInstructionCategory::Priviledge);
        context.mnemonic = mnemonics[(context.machine_code >> 23 & 1) as usize];
        LmDisassembler::cpx_cpu_transfer_format(self, context, FieldInfos::default_reg_field(1, LmCoprocessor::Cpu), FieldInfos::default_reg_field(0, LmCoprocessor::Cpu))
    }
    pub(super) fn mfmc0(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let mnemonics = [Some(LM_MNE_DI), Some(LM_MNE_EI)];

        if context.machine_code & 0b11111 != 0 ||
        (context.machine_code >> 6 & 0b11111) != 0 || 
        (context.machine_code >> 11 & 0b01100) != 0b01100 {
            return Err(LmError::throw_error(LmErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        }
        
        context.category = Some(LmInstructionCategory::Priviledge);
        context.format = Some(LmInstructionFormat::Mfmc0);
        context.mnemonic = mnemonics[(context.machine_code >> 5 & 1) as usize];
        context.operand_num = 1;
        context.operand[0] = Some(LmOpRegister::new_reg_opreand((context.machine_code >> 16 & 0b11111) as u8, LmCoprocessor::Cpu));

        if let Some(mne) = context.mnemonic{
            context.string.append_str(mne);
            context.string.append_char(' ');
            if let Some(LmOperand::LmOpRegister(reg)) = context.operand[0]{
                context.string.append_str(reg.get_register());
            }
        }
        Ok(())
    }
    pub(super) fn c0(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        let mnemonics: [[Option<&str>; 8]; 8] = [
            [None,  Some(LM_MNE_TLBR),  Some(LM_MNE_TLBWI),  None,  None,  None,  Some(LM_MNE_TLBWR),  None],
            [Some(LM_MNE_TLBP),  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None],
            [Some(LM_MNE_ERET),  None,  None,  None,  None,  None,  None,  Some(LM_MNE_DERET)], 
            [Some(LM_MNE_WAIT),  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None],
            [None,  None,  None,  None,  None,  None,  None,  None]
        ];
        if (context.machine_code >> 6 & 0b1111111111111111111) != 0 ||
        (context.machine_code >> 25 & 1) != 1{
            return Err(LmError::throw_error(LmErrorCode::FieldBadValue, context.opcode, context.address, context.machine_code))
        }

        context.category = Some(LmInstructionCategory::Priviledge);
        context.format = Some(LmInstructionFormat::Other);
        context.mnemonic = mnemonics[(context.machine_code >> 3 & 0b111) as usize][(context.machine_code & 0b111) as usize];
        let Some(mne) = context.mnemonic else {
            return Err(LmError::throw_error(LmErrorCode::DevError, context.opcode, context.address, context.machine_code))
        };
        context.string.append_str(mne);

        assert_ne!(context.mnemonic, None);
        Ok(())
    }

    //pcrel
    pub (super) fn addiupc(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.mnemonic = Some(LM_MNE_ADDIUPC);
        context.category = Some(LmInstructionCategory::Arithmetic);
        Ok(())
    }
    pub (super) fn lwpc(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.mnemonic = Some(LM_MNE_LWPC);
        context.category = Some(LmInstructionCategory::Load);
        Ok(())
    }
    pub (super) fn lwupc(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.mnemonic = Some(LM_MNE_LWUPC);
        context.category = Some(LmInstructionCategory::Load);
        Ok(())
    }
    pub (super) fn aluipc(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.mnemonic = Some(LM_MNE_ALUIPC);
        context.category = Some(LmInstructionCategory::Logical);
        Ok(())
    }
    pub (super) fn ldpc(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.mnemonic = Some(LM_MNE_LDPC);
        context.category = Some(LmInstructionCategory::Load);
        Ok(())
    }
    pub (super) fn auipc(&self, context: &mut LmInstructionContext) -> Result<(), LmError>{
        context.mnemonic = Some(LM_MNE_AUIPC);
        context.category = Some(LmInstructionCategory::Logical);
        Ok(())
    }
}