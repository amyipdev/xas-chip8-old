pub mod rawbin;

// T is the architecture's pointer size
pub type LabelTree<T> = std::collections::HashMap<String, T>;

pub fn run_output(
    src: Vec<crate::lexer::LexSection>,
    dest: &mut Vec<u8>,
    plat: &crate::platform::Platform,
) -> Result<(), Box<dyn std::error::Error>> {
    // TODO: opt?
    match plat.target {
        crate::platform::PlatformTarget::RawBinary => rawbin::run_output(src, dest, plat),
    }
    Ok(())
}

// TODO: see run_output notes about dynamic pointer sizes
// NOTE: this is only for things which use automatic linking
pub fn get_offset(p: &crate::platform::Platform) -> u64 {
    match (&p.arch, &p.target) {
        (crate::platform::PlatformArch::ChipEightRaw, crate::platform::PlatformTarget::RawBinary) => 0x200,
    }
}
