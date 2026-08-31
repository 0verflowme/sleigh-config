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

/// The compiler specification is available, and says what it is expected to say.
///
/// A consumer needs the stack pointer and the storage a convention passes
/// parameters in. Both are stated here, and without the file the only remaining
/// option is to guess them from register spellings -- which collide across
/// architectures: `r14` is ARM's link register and x86-64's sixth general
/// register, `s8` is MIPS's frame pointer and AArch64's 32-bit SIMD register.
#[test]
fn compiler_specifications_are_available() {
    let cspec = sleigh_config::processor_x86::CSPEC_X86_64_GCC;
    assert!(
        cspec.contains("<stackpointer"),
        "the compiler specification names the stack pointer"
    );
    assert!(
        cspec.contains("<pentry"),
        "the compiler specification names where parameters are passed"
    );
    assert!(
        !sleigh_config::CSPEC_DATA.is_empty(),
        "the table lists every compiler specification built"
    );
}
