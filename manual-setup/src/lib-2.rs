extern {
  fn appendNumber(x: u32);
  fn alert(x: u32);
}

#[no_mangle]
pub extern fn run() {
    unsafe {
      appendNumber(10);
      alert(4);
    }
}