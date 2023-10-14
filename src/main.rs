pub mod math;
pub mod solver;
pub mod dke_core;
pub mod constants;
pub mod io;

use crate::math::quat::Quat;
use crate::math::vec3::Vec3;

use crate::dke_core::state::State;
use crate::dke_core::dke_core::DKE;

fn main() {
    /* ---------------------------------------------------------------------- */
    /*  Set simulation time boundaries */
    let mut start_state = State::new();

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
