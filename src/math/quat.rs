
/*
 * Basic Quaternion class 
 */


#[derive(Debug)]
#[derive(Copy, Clone)]
/*
 * ----------------------------------------------------------------------
 *                    [struct definition]
 * ----------------------------------------------------------------------
 */
pub struct Quat {
  x: f64,
  y: f64,
  z: f64, 
  w: f64
}
/*
 * ----------------------------------------------------------------------
 *                    [create instance]
 * ----------------------------------------------------------------------
 */
impl Quat {
  pub fn new() -> Quat {
    Quat {
      x: 0.0,
      y: 0.0,
      z: 0.0,
      w: 1.0
    }
  }
}

/*
 * ----------------------------------------------------------------------
 *                    [getters and setters]
 * ----------------------------------------------------------------------
 */
/* Set x component  */
impl Quat {
  pub fn set_x(&mut self, new_x: &f64) {
    self.x = *new_x;

    /* Normalize quaternion */
    self.normalize();
  }
}

/* Set y component */
impl Quat {
  pub fn set_y(&mut self, new_y: &f64) {
    self.y = *new_y;

    /* Normalize quaternion */
    self.normalize();
  }
}

/* Set z component */
impl Quat {
  pub fn set_z(&mut self, new_z: &f64) {
    self.z = *new_z;

    /* Normalize quaternion */
    self.normalize();
  }
}

/* Set scalar part (w component) */
impl Quat {
  pub fn set_w(&mut self, new_w: &f64) {
    self.w = *new_w;

    /* Normalize quaternion */
    self.normalize();
  }
}

/* Assign new quaternion */
impl Quat {
  pub fn set(&mut self, new_x: &f64, new_y: &f64, new_z: &f64, new_w: &f64) {
    self.x = *new_x;
    self.y = *new_y;
    self.z = *new_z;
    self.w = *new_w;

    /* Normalize quaternion */
    self.normalize();
  }
}

/*
 * ----------------------------------------------------------------------
 *                    [PRIVATE core functions]
 * ----------------------------------------------------------------------
 */
impl Quat {
  fn normalize(&mut self) {
    /* build squares */
    let xx: f64 = self.x * self.x;
    let yy: f64 = self.y * self.y;
    let zz: f64 = self.z * self.z;
    let ww: f64 = self.w * self.w;

    let  dd: f64 = (xx + yy + zz + ww).sqrt();
    /* assign new component values */
    self.x = xx / dd;
    self.y = yy / dd;
    self.z = zz / dd;
    self.w = ww / dd;
  }
}
/*
 * ----------------------------------------------------------------------
 *                    [PUBLIC core functions]
 * ----------------------------------------------------------------------
 */
impl Quat {
  pub fn conjugate(&mut self) {
    /* assign new component values */
    self.x = - self.x;
    self.y = - self.y;
    self.z = - self.z;
  }
}

impl Quat {
  pub fn magn(self) -> f64 {
    /* build squares */
    let xx: f64 = self.x * self.x;
    let yy: f64 = self.y * self.y;
    let zz: f64 = self.z * self.z;
    let ww: f64 = self.w * self.w;

    /* build length of vector  */
    return (xx + yy + zz + ww).sqrt();
  }
}
