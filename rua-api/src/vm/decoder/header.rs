use derive_builder::Builder;

#[repr(u32)]
#[derive(Clone, Default)]
pub enum Signature {
    #[default]
    Lua = 0x1b4c7561
}

#[derive(Clone, Default)]
pub enum NumberSize {
    #[default]
    Float,
    Double
}

#[derive(Clone, Default)]
pub enum LuaVersion {
    #[default]
    V544
}

#[derive(Clone, Default)]
pub enum Endianess {
    #[default]
    LittleEndianess,
    BigEndianess
}

#[repr(u8)]
#[derive(Clone, Default)]
pub enum InstructionSize {
    #[default]
    _4byte = 4
}

#[derive(Default, Builder)]
pub struct Header {
    signature: Signature,
    version: LuaVersion,
    compatibility: u8,
    endianess: Endianess,
    integer_size: u8,
    instruction_size: u8,
    size_t: u8,
    number_size: NumberSize
}

#[test]
fn build_header() {
    let mut builder = HeaderBuilder::default();
    builder.signature(Signature::Lua);
    builder.endianess(Endianess::BigEndianess);
    builder.compatibility(1);
    builder.integer_size(1);
    builder.instruction_size(3);
    builder.size_t(3);

    let header = builder.build();
}
