

pub fn linear_interpolate(x: f64, x0: f64, x1: f64, y0: f64, y1: f64)
-> f64
{
  let y: f64 = y0 + ( x - x0 ) * (y1 - y0) / (x1 - x0) ; y
}