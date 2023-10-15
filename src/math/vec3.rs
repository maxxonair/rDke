
#[derive(Debug)]
#[derive(Copy, Clone)]

pub struct Vec3 {
  x: f64,
  y: f64,
  z: f64, 
}

impl Vec3 {
  pub fn new() -> Vec3 {
    Vec3 {
      x: 0.0,
      y: 0.0,
      z: 0.0
    }
  }
}
/*
 * -----------------------------------------------------------------------------
 *                    [setters]
 * -----------------------------------------------------------------------------
 */
/* Set x component  */
impl Vec3 {
  pub fn set_x(&mut self, new_x: &f64) {
    self.x = *new_x;

    /* Normalize Vec3ernion */
    self.normalize();
  }
}

/* Set y component */
impl Vec3 {
  pub fn set_y(&mut self, new_y: &f64) {
    self.y = *new_y;

    /* Normalize Vec3ernion */
    self.normalize();
  }
}

/* Set z component */
impl Vec3 {
  pub fn set_z(&mut self, new_z: &f64) {
    self.z = *new_z;

    /* Normalize Vec3ernion */
    self.normalize();
  }
}
/*
 * -----------------------------------------------------------------------------
 *                    [getters]
 * -----------------------------------------------------------------------------
 */
impl Vec3 {
  pub fn get_x(self) -> f64 {self.x}
}

impl Vec3 {
  pub fn get_y(self) -> f64 {self.y}
}

impl Vec3 {
  pub fn get_z(self) -> f64 {self.z}
}
/*
 * ----------------------------------------------------------------------
 *                    [core functions]
 * ----------------------------------------------------------------------
 */
impl Vec3 {
  pub fn normalize(&mut self) {
    /* build squares */
    let xx: f64 = self.x * self.x;
    let yy: f64 = self.y * self.y;
    let zz: f64 = self.z * self.z;

    /* build length of vector  */
    let dd = (xx + yy + zz ).sqrt();

    if dd != 0.0
    {
      /* assign new component values */
      self.x = xx / dd;
      self.y = yy / dd;
      self.z = zz / dd;
    }
  }
}

impl Vec3 {
  pub fn add(&mut self, vec: &Vec3) {
    /* assign new component values */
    self.x = self.x + vec.get_x();
    self.y = self.y + vec.get_y();
    self.z = self.z + vec.get_z();
  }
}

impl Vec3 {
  pub fn substr(&mut self, vec: &Vec3) {
    /* assign new component values */
    self.x = self.x - vec.get_x();
    self.y = self.y - vec.get_y();
    self.z = self.z - vec.get_z();
  }
}