//Author: RR28
//Discord: rrx1c
//Jabber: rrx1c@jabber.fr
//Github profile: https://github.com/RRx1C
//Link to repo: https://github.com/RRx1C/lunettes-mips-rs

mod mips;
// use mips;
use mips::disassembler::*;
use mips::instruction::*;

fn main() {
    let machine_codes =
    [   0x343259C8, 0x382059C8, 0x3C0059C8, 0x5001D671, 0x5401D671, 0x5820D671, 0x5C20D671, 0x74001672,
        0x1001D671, 0x1401D671, 0xA00159C8, 0xA76159C8, 0xAB6159C8, 0xAF6159C8, 0xBB6159C8, 0xBF6459C8,
        0xC00159C8, 0xC7E059C8, 0xCfE059C8, 0xC8C059C8, 0xE3E259C8, 0xE7E059C8, 0xEBE059C8, 0xF7E059C8,
        0xFBE059C8, 0x00000040, 0x000000C0, 0x001F10C3, 0x033F1004, 0x031F1007, 0x0040FC09, 0x0040F809, 
        0x00400408, 0x00400008, 0x03FE100A, 0x03FE100B, 0x0000000C, 0x03FFFFCC, 0x0000000D, 0x0000030F];
    let disassembler: LmDisassembler = new_disassembler(mips::LmAddressSize::Lm32bits);
    println!("Mnemonic SLL: {}", LmInstruction::get_memonic(LmMnemonicId::SLL));
    println!("Mnemonic SRA: {}", LmInstruction::get_memonic(LmMnemonicId::SRA));

    for i in 0..machine_codes.len(){
        match disassembler.disassemble(machine_codes[i], (0x00400000 + i * 4) as u64){
            Some(instruction) => {
                let instruction_machine_code = machine_codes[i].to_le_bytes();
                if  instruction.get_mnemonicid() == LmMnemonicId::JR || instruction.get_mnemonicid() == LmMnemonicId::JRHB ||
                instruction.get_mnemonicid() == LmMnemonicId::JALR || instruction.get_mnemonicid() == LmMnemonicId::JALRHB
                || instruction.get_mnemonicid() == LmMnemonicId::SYSCALL|| instruction.get_mnemonicid() == LmMnemonicId::BREAK
                || instruction.get_mnemonicid() == LmMnemonicId::SYNC{
                    println!("{:08x}  {:02x} {:02x} {:02x} {:02x}  {}", instruction.get_address(),
                    instruction_machine_code[0], instruction_machine_code[1], instruction_machine_code[2], instruction_machine_code[3],
                    instruction.temp_to_string());
                }
                else{
                    println!("{:08x}  {:02x} {:02x} {:02x} {:02x}  {}", instruction.get_address(),
                    instruction_machine_code[0], instruction_machine_code[1], instruction_machine_code[2], instruction_machine_code[3],
                    instruction.to_string());
                }
            },
            None => eprintln!("Instruction is probably not implemented yet or something went wrong"),
        };
    }
}