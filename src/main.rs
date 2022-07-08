extern crate regex;

// TODO: instead of manual .next(), consider .skip(1) or alike

pub mod bbu;
pub mod lexer;
pub mod parser;
pub mod platform;
pub mod eaf;

use std::str::FromStr;

use crate::bbu::ArchInstruction;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Need a platform for lexing, output
    let loc_plat: platform::Platform =
        platform::Platform::from_platform_info("c8r".to_string(), "bin".to_string());

    /*
    // Parser test
    let mut p: parser::Parser =
        parser::Parser::from_str("   .fakemacro    %rbx  ,    %rax  ; test ")?;
    println!("{:#?}", p.peek_queued());
    println!("{}", p.parse_line());
    println!("{:#?}", p.pop_parsed());
    p.drop_queued();
    println!("{:#?}", p.peek_queued());
    println!("{}", p.parse_line());

    // Chip8 test
    let p = parser::ParsedInstruction {
        instr: "0NNN".to_string(),
        args: Some(vec![0x300u16.to_string()]),
    };
    println!(
        "{:02x?}",
        crate::bbu::chip8_raw::get_instruction(p).get_output_bytes()
    );


    // Lexer test
    let mut vdq: std::collections::VecDeque<parser::ParsedOperation> =
        std::collections::VecDeque::new();
    vdq.push_back(parser::ParsedOperation::Macro(parser::ParsedMacro {
        mcr: "section".to_string(),
        args: Some(vec!["text".to_string()]),
    }));
    vdq.push_back(parser::ParsedOperation::Macro(parser::ParsedMacro {
        mcr: "L0:".to_string(),
        args: None,
    }));
    vdq.push_back(parser::ParsedOperation::Macro(parser::ParsedMacro {
        mcr: "text".to_string(),
        args: None,
    }));
    vdq.push_back(parser::ParsedOperation::Instruction(
        parser::ParsedInstruction {
            instr: "0NNN".to_string(),
            args: Some(vec![0x6f2u16.to_string()]),
        },
    ));
    let mut lx: lexer::Lexer = lexer::Lexer::from_vdq(vdq, loc_plat.clone());
    lx.lex_full_queue();
    println!("{:#?}\n\n\n", lx);
    */

    /*
    // Full test
    let mut p_full: parser::Parser = parser::Parser::from_str(include_str!("test.xs"))?;
    p_full.parse_all();
    // TODO: put cloning responsibilities on the callee, not the caller
    let mut lx_full: lexer::Lexer = lexer::Lexer::from_vdq(p_full.pop_vdq(), loc_plat.clone());
    lx_full.lex_full_queue();
    println!("{:#?}", lx_full);
    let mut nv: Vec<u8> = vec![];
    bbu::outs::run_output(lx_full.pop_vdq(), &mut nv, &loc_plat);
    println!("{:02x?}", nv);
    */

    // EAF test
    localprettyprint(&eaf::assemble_full_source(&include_str!("test.xs").to_string(),
                                               &platform::Platform::from_platform_info(
                                                   "chip8-raw".to_string(),
                                                   "rawbinary".to_string()
                                               )));

    Ok(())
}

fn localprettyprint(s: &Vec<u8>) {
    print!("Machine code produced: ");
    for n in s {
        print!("{:02x?} ", n);
    }
    println!("");
}
