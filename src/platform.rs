#[derive(Clone, Debug)]
pub enum PlatformArch {
    ChipEightRaw,
}

#[derive(Clone, Debug)]
pub enum PlatformTarget {
    RawBinary,
}

#[derive(Clone, Debug)]
pub struct Platform {
    pub arch: PlatformArch,
    pub target: PlatformTarget,
}

// TODO: proper error handling, return Result<Err> instead of panic
// TODO: overall since this is a library everything needs to be converted
// to error types, the library should NEVER panic - that's the responsibility
// of executables!
// TODO: general consideration CROSS-LIBRARY, change String to &String wherever
// possible. Allows passing string literals (&String = &str) and reduces call
// changes between str and String.
impl Platform {
    pub fn from_platform_info(arch: String, target: String) -> Self {
        Platform {
            // TODO: consider using some kind of lookup table?
            arch: match arch.to_lowercase().as_str() {
                "chipeightraw" | "chip8-raw" | "c8r" | "chip8r" => PlatformArch::ChipEightRaw,
                _ => panic!("unsupported arch"),
            },
            target: match target.to_lowercase().as_str() {
                "bin" | "binary" | "raw" | "rawbin" | "rawbinary" => PlatformTarget::RawBinary,
                _ => panic!("unsupported target"),
            },
        }
    }
}
