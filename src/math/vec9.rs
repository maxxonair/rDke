
#[derive(Debug)]
#[derive(Copy, Clone)]

pub struct Vec9 {
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

impl Vec9 {

  pub fn new() -> Vec9 {
    Vec9 {
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
/* Set c00 component  */
impl Vec9 {
  pub fn set_c00(&mut self, new_c00: &f64) {
    self.c00 = *new_c00;
  }
}

/* TODO: implement missing setters */

impl Vec9 {
  pub fn get_c00(self) -> f64 {self.c00}
}

/* TODO: Implement missing getters */

/*
 * ----------------------------------------------------------------------
 *                    [core functions]
 * ----------------------------------------------------------------------
 */
/* None */