pub mod math;
pub mod solver;
pub mod dke_core;
pub mod constants;
pub mod io;
pub mod environment;

use crate::math::quat::Quat;
use crate::math::vec3::Vec3;

use crate::dke_core::state::State;
use crate::dke_core::dke_core::DKE;

fn main() {
    /* ---------------------------------------------------------------------- */
    /*  Set simulation time boundaries */
    let mut start_state = State::new();

    start_state.set_pos_z(&1000.0);

    start_state.set_vel_x(&5.234213);
    start_state.set_vel_y(&-50.234213);
    start_state.set_vel_z(&150.0);

    start_state.set_mass_kg(&1000.0);

    println!("{:?}", start_state.get_pos());
    /* ---------------------------------------------------------------------- */
    /* Create simulation core */
    let mut dke: DKE = DKE::new();
    /* ---------------------------------------------------------------------- */
    /* Set simulation boundary conditions */
    dke.set_t_start(&0.0);
    dke.set_t_end(&60.0);
    dke.set_step_size(&0.001);
    dke.set_start_state(start_state);
    /* ---------------------------------------------------------------------- */
    /* [RUN SIMULATION] */
    /* ---------------------------------------------------------------------- */
    dke.run_simulation();
    /* ---------------------------------------------------------------------- */
}
