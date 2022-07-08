// TODO: specify BBU = "better binutils" or "basic binary units"
// TODO: clarify vec bits are in exact order, v[0] = binary[0]
// TODO: move CPU targets into new module

// TODO: generic argument types (direct, label, memory, register, etc)
// TODO: general symbol implementation

pub mod chip8_raw;
pub mod outs;

pub trait ArchInstruction {
    fn get_output_bytes(&self) -> Vec<u8>;
    fn get_lex(a: Option<Vec<String>>) -> Self
    where
        Self: Sized;
}

pub trait ArchMacro {
    fn get_output_bytes(&self) -> Vec<u8>;
    fn get_lex(a: Option<Vec<String>>) -> Self
    where
        Self: Sized;
}

// TODO: optimize, minimize code deduplication
// TODO: should doubled patterns (0x0x for example) be allowed?
// trim_start_matches might allow that
// TODO: proper error handling, return None instead of .unwrap()ing
// TODO: generally better suite of argument parsing
pub fn parse_unknown_radix_u16(s: &String) -> Option<u16> {
    // TODO: is making a copy of the string really necessary?
    // TODO: would if-elseif be faster?
    // TODO: when unwrap is handled correctly below this becomes
    // less necessary
    match s.len() {
        0 => return None,
        // this also might help do away with the last TODO
        // consider rewriting this whole function later
        // also, we need a TODO type generic version
        // something easily replicable across all sizes
        // more reason to TODO rewrite all of this
        // this just works as patchwork for now
        1 => {
            if s.chars().nth(0) == Some('0') {
                return Some(0);
            }
        }
        _ => {}
    }
    // TODO: dozenal support (XE-based maybe?)
    // TODO: explore how heavily features should be made
    Some(match s.chars().nth(0).unwrap() {
        '0' => match s.chars().nth(1).unwrap() {
            // lots of duplication here
            'x' => u16::from_str_radix(s.trim_start_matches("0x"), 16).unwrap(),
            'b' => u16::from_str_radix(s.trim_start_matches("0b"), 2).unwrap(),
            // octal is autotrimmed
            _ => u16::from_str_radix(s, 8).unwrap(),
        },
        // more duplication here
        '-' => u16::from_str_radix(s.trim_start_matches("-"), 10)
            .unwrap()
            .wrapping_neg(),
        _ => u16::from_str_radix(s, 10).unwrap(),
    })
}
