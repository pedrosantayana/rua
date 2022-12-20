mod state;
mod vm;
mod instructions;

#[cfg(test)]
mod test {
    use super::vm::RuaVM;

  #[test]
  fn luavm() {
    let mut vm = RuaVM::new();

    


    // vm.run_bytecode_file("/home/p314/src/langs/rua/luac.out");
  }
}