//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs

#![no_std]
pub mod instruction;
pub mod disassembler;
pub mod operands;
pub mod error;
mod utils;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LmAddressSize{
    Lm32bits = 4, _Lm64ibts = 8
}