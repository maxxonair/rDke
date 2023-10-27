use ini::Ini;

pub mod math;
pub mod solver;
pub mod dke_core;
pub mod constants;
pub mod io;
pub mod environment;

use crate::dke_core::state::State;
use crate::dke_core::dke_core::DKE;
use crate::constants::constants::*;

fn main() {
    /* Create parameter file instance > sim.ini < */
    let sim_conf = Ini::load_from_file(SIM_PARAMETER_FILE_PATH).unwrap();
    /* ---------------------------------------------------------------------- */
    /*  Initialise start state from parameters */
    let mut start_state = State::new();

    start_state.set_time(&((sim_conf
        .section(Some("sim")).unwrap()
        .get("t_start_s").unwrap())
        .parse::<f64>().unwrap()));

    start_state.set_pos_x(&((sim_conf
        .section(Some("start_state")).unwrap()
        .get("pos_pci_x_m").unwrap())
        .parse::<f64>().unwrap()));
    start_state.set_pos_y(&((sim_conf
        .section(Some("start_state")).unwrap()
        .get("pos_pci_y_m").unwrap())
        .parse::<f64>().unwrap()));
    start_state.set_pos_z(&((sim_conf
        .section(Some("start_state")).unwrap()
        .get("pos_pci_z_m").unwrap())
        .parse::<f64>().unwrap()));

    start_state.set_vel_x(&((sim_conf
        .section(Some("start_state")).unwrap()
        .get("vel_pci_x_m").unwrap())
        .parse::<f64>().unwrap()));
    start_state.set_vel_y(&((sim_conf
        .section(Some("start_state")).unwrap()
        .get("vel_pci_y_m").unwrap())
        .parse::<f64>().unwrap()));
    start_state.set_vel_z(&((sim_conf
        .section(Some("start_state")).unwrap()
        .get("vel_pci_z_m").unwrap())
        .parse::<f64>().unwrap()));

    start_state.set_mass_kg(&((sim_conf
        .section(Some("start_state")).unwrap()
        .get("sc_mass_start_kg").unwrap())
        .parse::<f64>().unwrap()));

    println!("{:?}", start_state.get_pos());
    /* ---------------------------------------------------------------------- */
    /* Create simulation core */
    let mut dke: DKE = DKE::new();
    /* ---------------------------------------------------------------------- */
    /* Set simulation boundary conditions */
    dke.set_t_start(&((sim_conf
        .section(Some("sim")).unwrap()
        .get("t_start_s").unwrap())
        .parse::<f64>().unwrap()) );
    dke.set_t_end(&((sim_conf
        .section(Some("sim")).unwrap()
        .get("t_end_s").unwrap())
        .parse::<f64>().unwrap()) );
    dke.set_step_size(&((sim_conf
        .section(Some("sim")).unwrap()
        .get("dt_sim_s").unwrap())
        .parse::<f64>().unwrap()) );
    dke.set_start_state(start_state);
    /* ---------------------------------------------------------------------- */
    /* [RUN SIMULATION] */
    /* ---------------------------------------------------------------------- */
    dke.run_simulation();
    /* ---------------------------------------------------------------------- */
}
