/* 
 * @brief: This class contains the core of the dynamic kinematic environment
 *         (DKE). It manages all core modules to run the simulation.
 * 
 * @description: This is the most central module of the simulation process. Not
 *               only the solving process is administered here, but also all 
 *               essential data to run the simulation is compiled and 
 *               distributed.
 * 
 *               TODO: Add extensive explanation.
 * 
 * 
 */
/* ---------------------------------------------------------------------------*/

/* Include external crates */
use std::time::Instant;
use ndarray::Array1;
use tqdm::tqdm;

/* Import (local) structs */
use crate::dke_core::state::State;

use crate::environment::environment::Environment;
/* Include local crates */
use crate::solver::rk4::step;
use crate::dke_core::eom::dxdt;
use crate::io::write_csv::{*, self};
use crate::dke_core::state_augmentation::{augment_state_solve,
                                          augment_state_write};
use crate::dke_core::dke_core_load_param::load_dke_core_parameters;
use crate::util::rlog::RLog;
use crate::util::plot::*;

/* Import constants */
use crate::constants::state::*;

pub struct DKE {
  /* [start time] 
   * @description : Simulation start time
   * @unit        : seconds
   * 
   * */
   sim_start_time_s: f64,
  /* [end time] 
   * @description : Simulation end time
   * @unit        : seconds
   * 
   * */
   sim_end_time_s: f64,
  /* [end time] 
   * @description : Current simulation time. This is to track the simulation 
   *                time while the simulator is running.
   * @unit        : seconds
   * 
   * */
   sim_current_time_s: f64,
  /* [simulation solver time increment] 
   * @description : Time increment used by the simulations solver.
   * @unit        : seconds
   * 
   * */
   dt_s: f64,
  /* [Print Interval] 
   * @description : Time interval print outs while the simulation is running are 
   *                called
   * @unit        : s
   * 
   * */
   param_sim_print_interval_s: f64,
  /* [write interval] 
   * @description : Time interval to call writing the simulation state to 
   *                archiving.
   *                Note: The actual write to file is controlled by when the 
   *                      buffer is flushed with param_sim_archive_flush_interval_s
   * @unit        : s
   * 
   * */
   param_sim_archive_interval_s: f64,
  /* [Flush write buffer interval] 
   * @description : Time interval to flush the archiving buffer.
   * @unit        : s
   * 
   * */
   param_sim_archive_flush_interval_s: f64,
  /* [State struct] 
   * @description : Full state struct, currently used to input the start state
   *                when initialising the simulation.
   * @unit        : N/A
   * 
   * */
   state: State,
  /* [Environment struct] 
   * @description : Full environment struct
   * @unit        : N/A
   * 
   * */
   environment: Environment,
}
/*
 * -----------------------------------------------------------------------------
 *                    [constructor]
 * -----------------------------------------------------------------------------
 */
impl DKE {
  pub fn new() -> DKE {
    DKE {
      sim_start_time_s: 0.0,
      sim_end_time_s: 0.0,
      sim_current_time_s: 0.0,
      dt_s: 0.0,
      param_sim_print_interval_s: 0.0,
      param_sim_archive_interval_s: 0.0,
      param_sim_archive_flush_interval_s: 0.0,
      state: State::new(),
      environment: Environment::new()
    }
  }
}
/* -----------------------------------------------------------------------------
*                    [setters]
* -----------------------------------------------------------------------------
*/
/* Set simulation start time [s]  */
impl DKE {
  pub fn set_t_start(&mut self, t_start_s_in: &f64) {
   self.sim_start_time_s = *t_start_s_in;
  }
  pub fn set_t_end(&mut self, t_end_s_in: &f64) {
    self.sim_end_time_s = *t_end_s_in;
  }
  pub fn set_step_size(&mut self, dt_s_in: &f64) {
    self.dt_s = *dt_s_in;
  }
  pub fn set_start_state(&mut self, start_state: State) {
    self.state = start_state.clone();
  }
  pub fn set_param_sim_print_interval_s(&mut self, param_sim_print_interval_s_in: &f64) {
    self.param_sim_print_interval_s = *param_sim_print_interval_s_in;
  }
  pub fn set_param_sim_archive_interval_s(&mut self, param_sim_archive_interval_s_in: &f64) {
    self.param_sim_archive_interval_s = *param_sim_archive_interval_s_in;
  }
  pub fn set_param_sim_archive_flush_interval_s(&mut self, param_sim_archive_flush_interval_s_in: &f64) {
    self.param_sim_archive_flush_interval_s = *param_sim_archive_flush_interval_s_in;
  }
}

/* -----------------------------------------------------------------------------
 *                    [getters]
 * ------------------------------------------------------------------------------
 */

impl DKE {
  pub fn get_dt_s(&self) -> f64 {self.dt_s}
  pub fn get_sim_time_s(&self) -> f64 {self.sim_current_time_s}

  pub fn get_mut_environment(&mut self) -> &mut Environment {&mut self.environment}
}


/*
 * -----------------------------------------------------------------------------
 *                            [functions]
 * -----------------------------------------------------------------------------
 */ 
 /*
  * @brief: Simulation core function
  * 
  * @description: This function handles:
  *               * Initialize simulation
  *               * Run through all time steps and handle integrator calls
  * 
  */
impl DKE {
  pub fn run_simulation(&mut self) 
  {
    /* ---------------------------------------------------------------------- */
    /*                 [Load DKE core parameters]                             */
    /* ---------------------------------------------------------------------- */
    load_dke_core_parameters(self);
    /* ---------------------------------------------------------------------- */
    /*       [Inititalize message log]                                        */
    let mut log: RLog = RLog::new();
    log.set_enable_debug_messages(&false);
    /* ---------------------------------------------------------------------- */
    /* Initialize state as vector */
    let mut state_vec: Array1<f64> = self.state.get_vector();
    /* Make sure the state vectors time value matches the start time selected 
     * for this simulation. */
    state_vec[STATE_VEC_INDX_SIM_TIME] = self.sim_start_time_s;
    /* Create a clone of the start state to keep track of the previous state
     * Note: This is used for post-solving state augmentation */
    let mut state_vec_n0: Array1<f64> = state_vec.clone();

    /* Calculate total number of simulation steps */
    let num_steps: i64 = ((self.sim_end_time_s - self.sim_start_time_s) 
                              / self.dt_s) as i64 + 1;
    
    /* Initialize variable to track simulation time */
    self.sim_current_time_s = self.sim_start_time_s;

    /* Create file writer */
    let mut results_writer = write_csv::create_csv(
      "./data_out/out.csv".to_string());

    log.log_msg("---------------------------------------------------------------");
    log.log_msg("              [SIMULATION START]");
    log.log_msg("---------------------------------------------------------------");
    log.log_msg("");

    /* Create timer for runtime profiling */
    let simulation_timer = Instant::now();

    /* Set counter for print outs while the simulation is running */
    let mut print_out_counter: f64 = 0.0;
    let mut write_out_counter: f64 = 0.0;
    let mut write_flush_counter: f64 = 0.0;

    /* Write initial state to csv */
    state_vec  = augment_state_write(&self.get_mut_environment(), &mut state_vec, &state_vec_n0 );
    write_csv::append_to_csv(&mut results_writer, &state_vec).unwrap();

    /* ---------------------------------------------------------------------- */
    /* [!] -----> Simulation main loop                                        */
    for sim_step in tqdm(0..num_steps).style(tqdm::Style::Block) {

      /* Write Simulation status to console  */
      if print_out_counter >= self.param_sim_print_interval_s 
        || sim_step == 0
        || sim_step == num_steps - 1
      {
        log.log_dbg(&format!("SimTime [s] {:.3?} ( {:.2?}  {:.2?}  {:.2?} ) ->> Altitude [m] {:.2?}", 
        self.sim_current_time_s, state_vec[STATE_VEC_INDX_POS_X], state_vec[STATE_VEC_INDX_POS_Y], 
        state_vec[STATE_VEC_INDX_POS_Z], self.state.get_altitude(&state_vec)));
        print_out_counter = 0.0;
      }
      print_out_counter += self.dt_s;

      /* -------------------------------------------------------------------- */
      /* !! ---> Perform integration step with step size dt_s <--- !!         */
      /* +_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_ */
      state_vec = step( &state_vec, 
                          &dxdt, 
                          self.dt_s, 
                          &mut self.environment);
      /* +_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_+_ */
      /* -------------------------------------------------------------------- */

      /* Update current simulation time for the current result step */
      self.sim_current_time_s += self.dt_s;
      
      /* Update environment with simtime information */
      let dts: f64 = self.dt_s;
      let simtime: f64 = self.sim_current_time_s;
      self.get_mut_environment().set_simtimes(&dts, &simtime);

      /* Post-process elements that are not filled in by the solver at solving 
       * frequency
       * */
      state_vec  = augment_state_solve(&self.get_mut_environment(),&mut state_vec, &state_vec_n0 );

      /* Increment counter to trigger [write results to file] */
      write_out_counter += self.dt_s;
      /* Write state udpates to file */
      if write_out_counter >= self.param_sim_archive_interval_s 
        || sim_step == num_steps - 1 
      { 
        /* Augment state at writing frequency */
        state_vec  = augment_state_write(&self.get_mut_environment(), &mut state_vec, &state_vec_n0 );
        /* Write state to csv */
        write_csv::append_to_csv(&mut results_writer, 
                                 &state_vec).unwrap();
        write_out_counter = 0.0;
      }

      /* Flush csv writer */
      if write_flush_counter >= self.param_sim_archive_flush_interval_s 
      {
        flush_csv_writer(&mut results_writer).unwrap();
        write_flush_counter = 0.0;
      }
      write_flush_counter += self.dt_s;

      /* Update vector to keep last timesteps state */
      state_vec_n0 = state_vec.clone();

      /* Check if early exit condition is met */
      if self.is_exit_conditions(&state_vec) == true
      {
        log.log_wrn("!! [ Exit Simulation ] !!");
        log.log_wrn("Early exit condition: [altitude below zero]");
        break;
      }
    } /* for(sim_step */
    /* ---------------------------------------------------------------------- */

    /* One extra flush to make sure everything is written   to the file
       before exiting */
    flush_csv_writer(&mut results_writer).unwrap();

    /* Print summary on completed simulation */
    log.log_msg("");
    log.log_msg("---------------------------------------------------------------");
    log.log_msg("              [FINISHED]");
    log.log_msg("---------------------------------------------------------------");
    log.log_msg(&format!("Simulated time                      [s] : {:.1}", 
      state_vec[STATE_VEC_INDX_SIM_TIME] ));
    log.log_msg(&format!("Runtime                             [s] : {:.3?}", 
    (simulation_timer.elapsed().as_millis() as f64) / 1000.0) );
    log.log_msg(&format!("Number of integration steps             : {:?}", 
      num_steps));
    log.log_msg(&format!("Step size                          [ms] : {:?}", 
      self.dt_s*1000.0));
    log.log_msg(&format!("Simulation time                    [ms] : {:.3?}", 
      simulation_timer.elapsed().as_millis()) );
    log.log_msg(&format!("Exec time per step                 [ms] : {:.6?}", 
      (simulation_timer.elapsed().as_millis() as f64) / num_steps as f64));
    log.log_msg(&format!("Simulated time / time to simulate   [-] : {:.1?}", 
      ((self.sim_end_time_s - self.sim_start_time_s) / simulation_timer
                                                        .elapsed()
                                                        .as_millis() as f64) * 1000.0));
    log.log_msg("---------------------------------------------------------------");
    /* Call Plotting functions on results */
    // TODO add enabler flags for postprocessing charts
    log.log_msg("[Save Plot] -> S/C ground track");
    plot_sc_groundtrack(&"./data_out/out.csv".to_string()).unwrap();

    log.log_msg("[Save Plot] -> S/C altitude vs longitiude");
    plot_sc_altitude_vs_longitude(&"./data_out/out.csv".to_string()).unwrap();
    
    log.log_msg("[Save Plot] -> S/C altitude vs simtime");
    plot_sc_altitude_vs_simtime(&"./data_out/out.csv".to_string()).unwrap();

    log.log_msg("[Save Plot] -> S/C drag coefficient vs altitude");
    plot_sc_dragcoeff_vs_altitude(&"./data_out/out.csv".to_string()).unwrap();

    log.log_msg("[Save Plot] -> S/C velocity vs altitude");
    plot_sc_velocity_altitude(&"./data_out/out.csv".to_string()).unwrap();

    log.log_msg("[Save Plot] -> Atmospheric density vs altitude");
    plot_atmos_density_altitude(&"./data_out/out.csv".to_string()).unwrap();

    log.close();
  }

  /* 
   * @brief: Function to check if a or several conditions are met to exit the 
   *         simulation before t_end is reached.
   * 
   * @description: This function should be called by run_simulation at any 
   *               integration step. If this function returns true the simulation
   *               shall exit and not continue to the next step.
   * 
   * @returns: true if any exit condition is met, false otherwise.
   */
  pub fn is_exit_conditions(&mut self, x_in: &Array1<f64>)
  -> bool
  {
    if x_in[STATE_VEC_INDX_ALTITUDE_PCPF_M] < 0.0 {true}
    else {false}

  }
}