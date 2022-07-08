pub fn run_output(
    src: Vec<crate::lexer::LexSection>,
    dest: &mut Vec<u8>,
    plat: &crate::platform::Platform
) -> () {
    // TODO: unresolved symbols! while we build the label tree, we don't link them yet
    // TODO: dynamically size LabelTree entries... we want architecture pointer sizes
    // overall TODO fix pointer sizes so that they are correct and dynamic to architecture
    let mut lt: crate::bbu::outs::LabelTree<u64> = crate::bbu::outs::LabelTree::new();
    // code duplication issue - redone across many files
    // perhaps we should store the LT, offset, and position in a pre-generated structure?
    // after all run_output calls do run through the global run_output which can pass a struct
    // could actually pass everything as just one structure pointer... TODO
    // also, NOTE offset is only used for implicit binary linking
    let offset: u64 = crate::bbu::outs::get_offset(plat);
    let mut pos: u64 = 0;

    // TODO: list a label
    for section in src {
        for label_t in &section.labels {
            let label: (&crate::lexer::LexIdLabel, Option<&String>) = label_t.extract();
            if let Some(n) = label.1 {
                // TODO: another candidate for `&str`ification
                lt.insert(n.to_string(), pos);
            }
            for op in label.0 {
                dest.extend(op.extract_bytes());
                pos += 2;
            }
        }
    }
}
