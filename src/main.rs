
pub mod math;
pub mod solver;
pub mod dke_core;
pub mod constants;
pub mod io;
pub mod environment;
pub mod util;

/* Import external crates */
use ini::Ini;

/* Import local crates */
use crate::dke_core::state::State;
use crate::dke_core::dke_core::DKE;

use crate::math::time_math::*;
use chrono::*;

/* Import Constants */
use crate::constants::filepaths::*;

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
        .get("vel_pci_x_ms").unwrap())
        .parse::<f64>().unwrap()));
    start_state.set_vel_y(&((sim_conf
        .section(Some("start_state")).unwrap()
        .get("vel_pci_y_ms").unwrap())
        .parse::<f64>().unwrap()));
    start_state.set_vel_z(&((sim_conf
        .section(Some("start_state")).unwrap()
        .get("vel_pci_z_ms").unwrap())
        .parse::<f64>().unwrap()));

    start_state.set_mass_kg(&((sim_conf
        .section(Some("start_state")).unwrap()
        .get("sc_mass_start_kg").unwrap())
        .parse::<f64>().unwrap()));

    start_state.set_date_time(&((sim_conf
        .section(Some("start_state")).unwrap()
        .get("start_date_time").unwrap())));

    /* ---------------------------------------------------------------------- */
    /* Initialise dynamic kinematic environment */
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
    /* !! [RUN SIMULATION] !! */
    /* ---------------------------------------------------------------------- */
    dke.run_simulation();
    
    /* TODO [Remove] Debug Earth time functions */
    // let date_str = "2023-12-19 16:39:57 +00:00";
    // // let testTime = Utc::now();
    // let testTime = DateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S %z")
    //     .unwrap()
    //     .with_timezone(&Utc);
    // println!("Equation of equinoxes [s]: {:.4?}", calc_equ_equinoxes_hours(testTime) * 3600.0);
    // println!("Current GMST [deg]: {:.4?}", calc_earth_gmst_deg(testTime));
    // println!("Current GAST [deg]: {:.4?}", calc_earth_gast_deg(testTime));
    /* ---------------------------------------------------------------------- */
}
