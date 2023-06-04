
#[derive(Debug)]
#[derive(Copy, Clone)]

pub struct Mat33 {
  c00: f64,
  c01: f64,
  c02: f64,
  c10: f64,
  c11: f64,
  c12: f64,
  c20: f64,
  c21: f64,
  c22: f64,
}

impl Vec3 {
  pub fn new() -> Mat33 {
    Mat33 {
      c00: 0.0,
      c01: 0.0,
      c02: 0.0,
      c10: 0.0,
      c11: 0.0,
      c12: 0.0,
      c20: 0.0,
      c21: 0.0,
      c22: 0.0,
    }
  }
}

/*
 * ----------------------------------------------------------------------
 *                    [getters and setters]
 * ----------------------------------------------------------------------
 */

impl Mat33 {
  pub fn set_c00(&mut self, new_c00: &f64) {
    self.c00 = *new_c00;
  }
}

impl Mat33 {
  pub fn set_c01(&mut self, new_c01: &f64) {
    self.c01 = *new_c01;
  }
}

impl Mat33 {
  pub fn set_c02(&mut self, new_c02: &f64) {
    self.c02 = *new_c02;
  }
}

impl Mat33 {
  pub fn set_c10(&mut self, new_c10: &f64) {
    self.c10 = *new_c10;
  }
}

impl Mat33 {
  pub fn set_c11(&mut self, new_c11: &f64) {
    self.c11 = *new_c11;
  }
}

impl Mat33 {
  pub fn set_c12(&mut self, new_c12: &f64) {
    self.c12 = *new_c12;
  }
}

impl Mat33 {
  pub fn set_c20(&mut self, new_c20: &f64) {
    self.c20 = *new_c20;
  }
}

impl Mat33 {
  pub fn set_c21(&mut self, new_c21: &f64) {
    self.c21 = *new_c21;
  }
}

impl Mat33 {
  pub fn set_c22(&mut self, new_c22: &f64) {
    self.c22 = *new_c22;
  }
}

impl Vec3 {pub fn get_c00(self) -> f64 {self.c00}}

impl Vec3 {pub fn get_c01(self) -> f64 {self.c01}}

impl Vec3 {pub fn get_c02(self) -> f64 {self.c02}}

impl Vec3 {pub fn get_c10(self) -> f64 {self.c10}}

impl Vec3 {pub fn get_c11(self) -> f64 {self.c11}}

impl Vec3 {pub fn get_c12(self) -> f64 {self.c12}}

impl Vec3 {pub fn get_c20(self) -> f64 {self.c20}}

impl Vec3 {pub fn get_c21(self) -> f64 {self.c21}}

impl Vec3 {pub fn get_c22(self) -> f64 {self.c22}}