// TODO: make all arches (including chip8_raw) under features
// same with a lot of other parts

// TODO: better error handling
// TODO: reduce repetition of this
use crate::bbu::ArchInstruction;

pub fn get_instruction(i: crate::parser::ParsedInstruction) -> impl crate::bbu::ArchInstruction {
    match i.instr.as_str() {
        "0NNN" | "0nnn" => Chip8_0NNN::get_lex(i.args),
        _ => panic!("unknown instruction error"),
    }
}

// chip8 "call" function
pub struct Chip8_0NNN {
    args: (u8, u8),
}

impl crate::bbu::ArchInstruction for Chip8_0NNN {
    fn get_output_bytes(&self) -> Vec<u8> {
        vec![self.args.0 & 0x0F, self.args.1]
    }
    // TODO: fix all this
    fn get_lex(a: Option<Vec<String>>) -> Self {
        // TODO: better number checking, this is proof-of-concept
        if let Some(i) = a {
            if i.len() != 1 {
                panic!("Improper number of arguments")
            }
            // TODO: proper error handling if number is out of sizescope
            let v: u16 = crate::bbu::parse_unknown_radix_u16(&i[0]).unwrap();
            // instruction is only valid when >0x200, <0x1000
            if v < 0x200 || v > 0xfff {
                panic!("Call out of range")
            }
            Chip8_0NNN {
                args: (((v & 0xf00) >> 8) as u8, v as u8 & 0xff),
            }
        } else {
            panic!("No arguments provided, error")
        }
    }
}
