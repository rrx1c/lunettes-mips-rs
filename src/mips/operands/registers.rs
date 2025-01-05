//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs

/*
    On peut s'en servir en tant qu'index dans l'array qui regroupe tous les registres 
    pour trouver le bon registre en précisant bien sûr le coprocessuer ou juste pour reconnaître un certain registre.
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum LmRegisters {
    ZERO, AT, V0, V1, A0, A1, A2, A3,
    T0, T1, T2, T3, T4, T5, T6, T7,
    S0, S1, S2, S3, S4, S5, S6, S7,
    T8, T9, K0, K1, GP, SP, FP, RA,
}

pub const LM_REG_ZERO: &str = "$zero"; pub const LM_REG_AT: &str = "$at"; pub const LM_REG_V0: &str = "$v0";
pub const LM_REG_V1: &str = "$v1"; pub const LM_REG_A0: &str = "$a0"; pub const LM_REG_A1: &str = "$a1";
pub const LM_REG_A2: &str = "$a2"; pub const LM_REG_A3: &str = "$a3"; pub const LM_REG_T0: &str = "$t0";
pub const LM_REG_T1: &str = "$t1"; pub const LM_REG_T2: &str = "$t2"; pub const LM_REG_T3: &str = "$t3";
pub const LM_REG_T4: &str = "$t4"; pub const LM_REG_T5: &str = "$t5"; pub const LM_REG_T6: &str = "$t6";
pub const LM_REG_T7: &str = "$t7"; pub const LM_REG_S0: &str = "$s0"; pub const LM_REG_S1: &str = "$s1";
pub const LM_REG_S2: &str = "$s2"; pub const LM_REG_S3: &str = "$s3"; pub const LM_REG_S4: &str = "$s4";
pub const LM_REG_S5: &str = "$s5"; pub const LM_REG_S6: &str = "$s6"; pub const LM_REG_S7: &str = "$s7";
pub const LM_REG_T8: &str = "$t8"; pub const LM_REG_T9: &str = "$t9"; pub const LM_REG_K0: &str = "$k0";
pub const LM_REG_K1: &str = "$k1"; pub const LM_REG_GP: &str = "$gp"; pub const LM_REG_SP: &str = "$sp";
pub const LM_REG_FP: &str = "$fp"; pub const LM_REG_RA: &str = "$ra"; pub const LM_REG_F0: &str = "$f0";
pub const LM_REG_F1: &str = "$f1"; pub const LM_REG_F2: &str = "$f2"; pub const LM_REG_F3: &str = "$f3";
pub const LM_REG_F4: &str = "$f4"; pub const LM_REG_F5: &str = "$f5"; pub const LM_REG_F6: &str = "$f6";
pub const LM_REG_F7: &str = "$f7"; pub const LM_REG_F8: &str = "$f8"; pub const LM_REG_F9: &str = "$f9";
pub const LM_REG_F10: &str = "$f10"; pub const LM_REG_F11: &str = "$f11"; pub const LM_REG_F12: &str = "$f12";
pub const LM_REG_F13: &str = "$f13"; pub const LM_REG_F14: &str = "$f14"; pub const LM_REG_F15: &str = "$f15";
pub const LM_REG_F16: &str = "$f16"; pub const LM_REG_F17: &str = "$f17"; pub const LM_REG_F18: &str = "$f18";
pub const LM_REG_F19: &str = "$f19"; pub const LM_REG_F20: &str = "$f20"; pub const LM_REG_F21: &str = "$f21";
pub const LM_REG_F22: &str = "$f22"; pub const LM_REG_F23: &str = "$f23"; pub const LM_REG_F24: &str = "$f24";
pub const LM_REG_F25: &str = "$f25"; pub const LM_REG_F26: &str = "$f26"; pub const LM_REG_F27: &str = "$f27";
pub const LM_REG_F28: &str = "$f28"; pub const LM_REG_F29: &str = "$f29"; pub const LM_REG_F30: &str = "$f30";
pub const LM_REG_F31: &str = "$f31"; pub const LM_REG_0: &str = "$0"; pub const LM_REG_1: &str = "$1";
pub const LM_REG_2: &str = "$2"; pub const LM_REG_3: &str = "$3"; pub const LM_REG_4: &str = "$4";
pub const LM_REG_5: &str = "$5"; pub const LM_REG_6: &str = "$6"; pub const LM_REG_7: &str = "$7";
pub const LM_REG_8: &str = "$8"; pub const LM_REG_9: &str = "$9"; pub const LM_REG_10: &str = "$10";
pub const LM_REG_11: &str = "$11"; pub const LM_REG_12: &str = "$12"; pub const LM_REG_13: &str = "$13";
pub const LM_REG_14: &str = "$14"; pub const LM_REG_15: &str = "$15"; pub const LM_REG_16: &str = "$16";
pub const LM_REG_17: &str = "$17"; pub const LM_REG_18: &str = "$18"; pub const LM_REG_19: &str = "$19";
pub const LM_REG_20: &str = "$20"; pub const LM_REG_21: &str = "$21"; pub const LM_REG_22: &str = "$22";
pub const LM_REG_23: &str = "$23"; pub const LM_REG_24: &str = "$24"; pub const LM_REG_25: &str = "$25";
pub const LM_REG_26: &str = "$26"; pub const LM_REG_27: &str = "$27"; pub const LM_REG_28: &str = "$28";
pub const LM_REG_29: &str = "$29"; pub const LM_REG_30: &str = "$30"; pub const LM_REG_31: &str = "$31";