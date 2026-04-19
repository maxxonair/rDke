/* init.rs */

/* External crates */
use ini::Ini;

/* Local crates */
use crate::dke_core::state::State;
use crate::dke_core::dke_core::DKE;

/* Constants */
use crate::constants::filepaths::*;

pub fn init() -> DKE {
    /* Load config */
    let sim_conf = Ini::load_from_file(SIM_PARAMETER_FILE_PATH).unwrap();

    /* ---------------- Start state ---------------- */
    let mut start_state = State::new();

    start_state.set_time(
        &sim_conf.section(Some("sim")).unwrap()
            .get("t_start_s").unwrap()
            .parse::<f64>().unwrap()
    );

    start_state.set_pos_x(
        &sim_conf.section(Some("start_state")).unwrap()
            .get("pos_pci_x_m").unwrap()
            .parse::<f64>().unwrap()
    );
    start_state.set_pos_y(
        &sim_conf.section(Some("start_state")).unwrap()
            .get("pos_pci_y_m").unwrap()
            .parse::<f64>().unwrap()
    );
    start_state.set_pos_z(
        &sim_conf.section(Some("start_state")).unwrap()
            .get("pos_pci_z_m").unwrap()
            .parse::<f64>().unwrap()
    );

    start_state.set_vel_x(
        &sim_conf.section(Some("start_state")).unwrap()
            .get("vel_pci_x_ms").unwrap()
            .parse::<f64>().unwrap()
    );
    start_state.set_vel_y(
        &sim_conf.section(Some("start_state")).unwrap()
            .get("vel_pci_y_ms").unwrap()
            .parse::<f64>().unwrap()
    );
    start_state.set_vel_z(
        &sim_conf.section(Some("start_state")).unwrap()
            .get("vel_pci_z_ms").unwrap()
            .parse::<f64>().unwrap()
    );

    start_state.set_mass_kg(
        &sim_conf.section(Some("start_state")).unwrap()
            .get("sc_mass_start_kg").unwrap()
            .parse::<f64>().unwrap()
    );

    start_state.set_date_time(
        sim_conf.section(Some("start_state")).unwrap()
            .get("start_date_time").unwrap()
    );

    /* ---------------- DKE setup ---------------- */
    let mut dke = DKE::new();

    dke.set_t_start(
        &sim_conf.section(Some("sim")).unwrap()
            .get("t_start_s").unwrap()
            .parse::<f64>().unwrap()
    );

    dke.set_t_end(
        &sim_conf.section(Some("sim")).unwrap()
            .get("t_end_s").unwrap()
            .parse::<f64>().unwrap()
    );

    dke.set_step_size(
        &sim_conf.section(Some("sim")).unwrap()
            .get("dt_sim_s").unwrap()
            .parse::<f64>().unwrap()
    );

    dke.set_start_state(start_state);

    dke
}