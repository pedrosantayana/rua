use super::header::Header;

pub enum Instruction {
    MOVE(, dyn Register),
    LOADK(dyn Register),
}

impl Instruction {
    pub fn new(bytecode: u32) -> Option<Instruction> {
        let operation_code = bytecode & 0x6;
        let register_A = URegister((bytecode << 0x6) & 0xFF);
        let register_B = IRegister((bytecode << 23) & );

        let register_Ax = 

        match operation_code {
            0 => Some(Instruction::MOVE(register_A, register_B)),
        }
    }
}

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

        Ok(Chunk { header: header })
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

pub struct URegister(u8);


pub struct IRegister(i16);

#[repr(type)]
pub enum RegisterType {
    A = u8,
    B = i16,
    C = i16,
    Bx = i32,
    SBx = i32,
    Ax = i32
}

pub struct Register {
    r_type: RegisterType,
    
}


pub struct Function {
  line_start: u32,
  line_end: u32,
  params: u8,
  vararg_flag: u8,
  registers: Vec<dyn Register>,
  instructions: Vec<Instruction>,
  constants: Vec<Constant>,
  function_prototypes: Vec<FnPrototype>,
  upvalues: Vec<dyn Value>,
}

trait Register {}

trait Value {}

struct Upvalue {}

struct FnPrototype {}

struct Constant {}