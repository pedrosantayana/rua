use std::{fs::File, io::Read};

use nom::Slice;

#[repr(u8)]
pub enum Instruction {
    MOVE = 1,
    LOADK,
    LOADKX,
    LOADBOOL,
    LOADNIL,
    GETUPVAL,
    GETTABUP,
    GETTABLE,
    SETTABUP,
    SETUPVAL,
    SETTABLE,
    NEWTABLE,
    SELF,
    ADD,
    SUB,
    MUL,
    MOD,
    POW,
    DIV,
    IDIV,
    BAND,
    BOR,
    BXOR,
    SHL,
    SHR,
    UNM,
    BNOT,
    NOT,
    LEN,
    CONCAT,
    JMP,
    EQ,
    LT,
    LE,
    TEST,
    TESTSET,
    CALL,
    TAILCALL,
    RETURN,
    FORLOOP,
    FORPREP,
    TFORLOOP,
    TFORCALL,
    SETLIST,
    CLOSURE,
    VARARG,
}

pub struct Chunk {
    header: ChunkHeader,

}

impl Chunk {
    pub fn from_bytecode_file(path: &str) -> Self {
        let mut file = File::open(path).expect("no file found");
        let mut bytes = file.bytes()
        .into_iter()
        .filter_map(|r|{
            match r {
                Ok(byte) => Some(byte),
                Err(e) => None
            }
        })
        .collect::<Vec<u8>>();

        let header = ChunkHeader::from_bytes(bytes.as_slice().slice(0..11));
    }
}


pub struct ChunkHeader {
    signature: u32,
    version: u8,
    format: u8,
    endianness: u8,
    int: u8,
    size_t: u8,
    instruction: u8,
    number: u8
}

impl ChunkHeader {
    pub fn from_bytes() -> Self {
        unimplemented!()
    }
}

#[test]
fn bytecode() {
    let mut a = Chunk::from_bytecode_file("");
}