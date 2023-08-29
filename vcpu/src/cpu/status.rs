// I know this isn't faithful, but Rust doesn't currently support C-style bitfields nicely
#[derive(Default, Debug)]
pub struct Status {
  pub carry: bool,
  pub zero: bool,
  pub interrupt_disable: bool,
  pub decimal: bool,
  pub break_command: bool,
  pub overflow: bool,
  pub negative: bool,
}

impl Status {
  pub fn reset(&mut self) {
    self.carry = false;
    self.zero = false;
    self.interrupt_disable = false;
    self.decimal = false;
    self.break_command = false;
    self.overflow = false;
    self.negative = false;
  }
}
