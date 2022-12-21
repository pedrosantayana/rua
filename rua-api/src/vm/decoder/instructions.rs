use std::{fs::File, io::{Read, Error}};

use nom::Slice;

pub enum Instruction {
    MOVE(Register, Register),
    LOADK(Register, )
}

impl Instruction {
    pub fn new(bytecode: u32) -> Option<Instruction> {
        let op = bytecode & 0x6;
        let mut regA = Register(bytecode);
        match op {
            0 => Some(Instruction::MOVE()),

        }
    }

    pub fn decode(bytecode: Vec<u8>, header: ChunkHeader) -> Vec<Instruction> {

    }
}

struct Register(u8);

#[repr(u8)]
pub enum OpCode {
    MOVE = 0,
    LOADK = 1,
    LOADKX = 2,
    LOADBOOL = 3,
    LOADNIL = 4,
    GETUPVAL = 5,
    GETTABUP = 6,
    GETTABLE = 7,
    SETTABUP = 8,
    SETUPVAL = 9,
    SETTABLE = 10,
    NEWTABLE = 11,
    SELF = 12,
    ADD = 13,
    SUB = 14,
    MUL = 15,
    MOD = 16,
    POW = 17,
    DIV = 18,
    IDIV = 19,
    BAND = 20,
    BOR = 21,
    BXOR = 22,
    SHL = 23,
    SHR = 24,
    UNM = 25,
    BNOT = 26,
    NOT = 27,
    LEN = 28,
    CONCAT = 29,
    JMP = 30,
    EQ = 31,
    LT = 32,
    LE = 33,
    TEST = 34,
    TESTSET = 35,
    CALL = 36,
    TAILCALL = 37,
    RETURN = 38,
    FORLOOP = 39,
    FORPREP = 40,
    TFORLOOP = 41,
    TFORCALL = 42,
    SETLIST = 43,
    CLOSURE = 44,
    VARARG = 45,
}

pub struct Chunk {
    header: ChunkHeader,

}

impl Chunk {
    pub fn new(bytecode: Vec<u8>) -> Result<Chunk, Error> {
        let header = ChunkHeader {
            signature: bytecode.get(0).unwrap().to_owned() as u32,
            version: bytecode.get(4).unwrap().to_owned(),
            format: bytecode.get(5).unwrap().to_owned(),
            endianness: bytecode.get(6).unwrap().to_owned(),
            int: bytecode.get(7).unwrap().to_owned(),
            size_t: bytecode.get(8).unwrap().to_owned(),
            instruction: bytecode.get(9).unwrap().to_owned(),
            number: bytecode.get(10).unwrap().to_owned(),
        };

        Ok(Chunk {
            header: header
        })
    }

    // pub fn from_bytecode_file(path: &str) -> Self {
    //     let mut file = File::open(path).expect("no file found");
    //     let mut bytes = file.bytes()
    //     .into_iter()
    //     .filter_map(|r|{
    //         match r {
    //             Ok(byte) => Some(byte),
    //             Err(e) => None
    //         }
    //     })
    //     .collect::<Vec<u8>>();

    //     let header = ChunkHeader::from_bytes(bytes.as_slice().slice(0..11));
    // }
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
    // pub fn from_bytes(b: Vec<u8>) -> Self {
    //     ChunkHeader {
    //         signature: b[0] << 24 || b[1] << 16 || b[2] << 8 || b[3],
    //         version: b[4],
    //         format: b[5],
    //         endianness: b[6],
    //         int: b[7],
    //         size_t: b[8],
    //         instruction: b[9],
    //         number: b[10]

    //     }
    // }
}

struct Function {
    line_start: u32,
    line_end: u32,
    params: u8,
    vararg_flag: u8,
    registers: Vec<Register>,
    instructions: Vec<Instruction>,
    constants: Vec<Constant>,
    function_prototypes: Vec<FnPrototype>,
    upvalues: Vec<dyn Value>
}

trait Value {
    
}

struct Upvalue {

}

struct FnPrototype {

}

struct Constant {

}

#[test]
fn test_create_chunk_from_file() {
    let bytecode = include_bytes!("luac.out");

    let chunk = Chunk::new(bytecode);
}