pub mod math;

use crate::math::state::State;
use crate::math::quat::Quat;
use crate::math::vec3::Vec3;

use crate::math::vec_math::add_vec3;

fn main() {

    // Create dummy state to start with 
    let s_1 = State::new();
    let mut q1 = Quat::new();

    let mut vec1: Vec3 = Vec3::new();
    let mut vec2: Vec3 = Vec3::new();

    vec1.set_x(&0.1);
    vec2.set_x(&0.2345);

    let vec3: Vec3 = add_vec3(vec1, vec2);

    println!("${:?}", vec3);

    println!("${:?}", s_1);

    println!("${:?}", q1);

    q1.set_x(&0.1);
    q1.set_z(&0.234);

    println!("${:?}", q1);

    // Create solver instance 

    // Solver loop 
}
