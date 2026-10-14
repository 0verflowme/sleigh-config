use libsla::{GhidraSleigh, Sleigh};
use sleigh_config::processor_x86;

struct InstructionLoader;

impl libsla::LoadImage for InstructionLoader {
    fn instruction_bytes(
        &self,
        _data: &libsla::VarnodeData,
    ) -> std::result::Result<Vec<u8>, String> {
        // PUSH RBP
        Ok(vec![0x55])
    }
}

#[test]
fn run_x86_64_libsla() -> Result<(), Box<dyn std::error::Error>> {
    let sleigh_spec = processor_x86::SLA_X86_64;
    let processor_spec = processor_x86::PSPEC_X86_64;
    let sleigh = GhidraSleigh::builder()
        .processor_spec(processor_spec)?
        .build(sleigh_spec)?;

    let loader = InstructionLoader;
    let address = libsla::Address::new(sleigh.default_code_space(), 0);
    let disassembly = sleigh
        .disassemble_native(&loader, address)
        .expect("disassembly should succeed");

    let instruction = &disassembly.instructions[0];
    assert_eq!(instruction.mnemonic, "PUSH");
    assert_eq!(instruction.body, "RBP");
    Ok(())
}

#[test]
fn a_processor_lists_its_files_by_name() {
    let names: Vec<&str> = processor_x86::FILES.iter().map(|(name, _)| *name).collect();
    assert!(names.contains(&"x86.ldefs"), "{names:?}");
    assert!(names.contains(&"x86-64-gcc.cspec"), "{names:?}");
    assert!(names.contains(&"x86.dwarf"), "{names:?}");
    let ldefs = processor_x86::FILES.iter().find(|(name, _)| *name == "x86.ldefs").unwrap().1;
    assert!(ldefs.contains("<language_definitions>"));
    let slas: Vec<&str> = processor_x86::SLAS.iter().map(|(name, _)| *name).collect();
    assert!(slas.contains(&"x86-64.sla"), "{slas:?}");
}
