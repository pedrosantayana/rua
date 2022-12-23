







#[test]
fn test_create_chunk_from_file() {
    let bytecode = include_bytes!("luac.out");

    let chunk = Chunk::new(bytecode);
}
