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
use ini::Ini;
use ndarray::Array1;

/* Import (local) structs */
use crate::dke_core::state::State;
use crate::environment::planet::planet::Planet;

/* Include local crates */
use crate::solver::rk4::step;
use crate::dke_core::eom::dxdt;
use crate::environment::gravity::gravity::get_grav_acc;
use crate::io::write_csv::{*, self};

/* Import constants */
use crate::constants::state::*;
use crate::constants::filepaths::*;

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
  /* [Planet struct] 
   * @description : Data struct containing all planet relevant parameters
   * @unit        : N/A
   * 
   * */
   planet: Planet
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
      dt_s: 0.0,
      param_sim_print_interval_s: 0.0,
      param_sim_archive_interval_s: 0.0,
      param_sim_archive_flush_interval_s: 0.0,
      state: State::new(),
      planet: Planet::new()
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
}
/* Set simulation end time [s]  */
impl DKE {
  pub fn set_t_end(&mut self, t_end_s_in: &f64) {
    self.sim_end_time_s = *t_end_s_in;
  }
 }
 /* Set simulation step time [s]  */
impl DKE {
  pub fn set_step_size(&mut self, dt_s_in: &f64) {
    self.dt_s = *dt_s_in;
  }
 }
/* Set simulation start state [-]  */
impl DKE {
  pub fn set_start_state(&mut self, start_state: State) {
    self.state = start_state.clone();
  }
 }

 impl DKE {
  pub fn set_planet(&mut self, planet_in: Planet) {
    self.planet = planet_in.clone();
  }
 }
/* -----------------------------------------------------------------------------
*                    [getters]
* ------------------------------------------------------------------------------
* Allow immutable access only.
*/
impl DKE {
  pub fn get_planet(&self) -> &Planet {&self.planet}
}
/*
 * -----------------------------------------------------------------------------
 *                    [functions]
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
  pub fn run_simulation(&mut self) {
    /* ---------------------------------------------------------------------- */
    /*                 [Load from paramters]                                  */
    /* ---------------------------------------------------------------------- */
    self.load_parameters();
    /* ---------------------------------------------------------------------- */
    /* Initialize state as vector */
    let mut x_vec = self.state.get_vector();
    /* Make sure the state vectors time value matches the start time selected 
     * for this simulation. */
    x_vec[STATE_VEC_INDX_SIM_TIME] = self.sim_start_time_s;
    /* Create a clone of the start state to keep track of the previous state
     * Note: This is used for post-solving state augmentation */
    let mut x_vec_n0 = x_vec.clone();

    /* Calculate total number of simulation steps */
    let num_steps: i64 = ((self.sim_end_time_s - self.sim_start_time_s) 
                              / self.dt_s) as i64 + 1;
    
    /* Initialize variable to track simulation time */
    let mut t_sim = self.sim_start_time_s;

    /* Create file writer */
    let mut results_writer = write_csv::create_csv(
      "./data_out/out.csv".to_string());

    println!("---------------------------------------------------------------");
    println!("              [SIMULATION START]");
    println!("---------------------------------------------------------------");
    println!("Initial state: {:?}", x_vec);
    println!("");

    /* Create timer for runtime profiling */
    let simulation_timer = Instant::now();

    /* Set counter for print outs while the simulation is running */
    let mut print_out_counter: f64 = 0.0;
    let mut write_out_counter: f64 = 0.0;
    let mut write_flush_counter: f64 = 0.0;

    /* Simulation main loop */
    for sim_step in 0..num_steps {
      /* Write Simulation status to console  */
      if print_out_counter >= self.param_sim_print_interval_s 
        || sim_step == 0
        || sim_step == num_steps - 1
      {
        print!("SimTime [s] {:.3?} ( {:.2?}  {:.2?}  {:.2?} ) ->> Altitude [m] {:.2?} \n", 
          t_sim, x_vec[STATE_VEC_INDX_POS_X], x_vec[STATE_VEC_INDX_POS_Y], 
          x_vec[STATE_VEC_INDX_POS_Z], self.state.get_altitude(&x_vec));
        print_out_counter = 0.0;
      }
      print_out_counter += self.dt_s;

      
      /* Assign simulation time to current state */
      x_vec[STATE_VEC_INDX_SIM_TIME] = t_sim;
      /* Update state epoch */
      x_vec[STATE_VEC_INDX_J2000_S] += self.dt_s;

      /* -------------------------------------------------------------------- */
      /* !! ---> Perform integration step with step size dt_s <--- !! */
      x_vec = step( &x_vec, &dxdt, t_sim, self.dt_s, &self);
      /* -------------------------------------------------------------------- */

      /* Post-process elements that are not filled in by the solver at solving 
       * frequency
       * */
      x_vec  = self.augment_state_solve(&mut x_vec, &x_vec_n0 );

      /* Write state udpates to file */
      if write_out_counter >= self.param_sim_archive_interval_s 
        || sim_step == 0
        || sim_step == num_steps - 1
      { 
        /* Augment state at writing frequency */
        x_vec  = self.augment_state_write(&mut x_vec, &x_vec_n0 );
        /* Write state to csv */
        write_csv::append_to_csv(&mut results_writer, 
                                 &x_vec,
                                 t_sim).unwrap();
        write_out_counter = 0.0;
      }
      write_out_counter += self.dt_s;


      /* Flush csv writer */
      if write_flush_counter >= self.param_sim_archive_flush_interval_s 
      {
        flush_csv_writer(&mut results_writer).unwrap();
        write_flush_counter = 0.0;
      }
      write_flush_counter += self.dt_s;

      /* Update vector to keep last timesteps state */
      x_vec_n0 = x_vec.clone();
      /* Update simulation time for the next step */
      t_sim += self.dt_s;


      /* Check if early exit condition is met */
      if x_vec[STATE_VEC_INDX_ALTITUDE_PCPF_M] < 0.0
      {
        println!("!! [ Exit Simulation ] !!");
        println!("Early exit condition: [altitude below zero]");
        break;
      }
    } /* sim_step */
    /* One extra flush to make sure everything is written to the file
       before exiting */
    flush_csv_writer(&mut results_writer).unwrap();

    println!("");
    println!("---------------------------------------------------------------");
    println!("              [FINISHED]");
    println!("---------------------------------------------------------------");
    println!("Simulated time                      [s] : {:?}", 
      (self.sim_end_time_s - self.sim_start_time_s) );
    println!("Number of integration steps             : {:?}", 
      num_steps);
    println!("Step size                          [ms] : {:?}", 
      self.dt_s*1000.0);
    println!("Simulation time                    [ms] : {:.3?}", 
      simulation_timer.elapsed().as_millis() );
    println!("Exec time per step                 [ms] : {:.6?}", 
      (simulation_timer.elapsed().as_millis() as f64) / num_steps as f64);
    println!("Simulated time / time to simulate   [-] : {:.1?}", 
      ((self.sim_end_time_s - self.sim_start_time_s) / simulation_timer
                                                        .elapsed()
                                                        .as_millis() as f64) * 1000.0);
    println!("---------------------------------------------------------------");
    println!("");
    println!("Final state: {:?}", x_vec);
    println!("---------------------------------------------------------------");

  }
}

/*
 * @brief: This function is to fill fields of the full state vector that are not 
 *         filled by the solver itself.
 * 
 * @details: Note: State augmentation is the process of filling in missing fields
 *                 (field not solved by the solving step) of the full state 
 *                 vector. For computational efficiency state augmentation is 
 *                 done in two steps. 
 *                 (1) augment_state_solve() is called at solving frequency and 
 *                     only contains post-processing computations that need to 
 *                     be done at full frequency (e.g. acceleration from velocity
 *                     derivative)
 *                 (2) augment_state_write() is called at writing frequency and 
 *                     is only called when writing the results to file. This 
 *                     function contains all processing that can be done at a
 *                     lower frequency.
 * 
 *           Post solving computations include: 
 *           * Acceleration
 * 
 * @param[in] x1_in - Full state vector for current solving step n
 * 
 * @param[in] x0_in - Full state vector for previous solving step n-1
 *  
 * @returns x1_in - Full state vector for current solving step with additional 
 *                  fields filled.
 * 
 */
impl DKE {
  fn augment_state_solve(&mut self, x1_inout: &Array1<f64>, x0_in: &Array1<f64>)
  -> Array1<f64>
  {
    let mut x_out = (*x1_inout).clone();
    /* Compute linear acceleration from incremental velocity change */
    x_out[STATE_VEC_INDX_ACC_X] = (x_out[STATE_VEC_INDX_VEL_X] 
      - x0_in[STATE_VEC_INDX_VEL_X]) / self.dt_s;
    x_out[STATE_VEC_INDX_ACC_Y] = (x_out[STATE_VEC_INDX_VEL_Y] 
      - x0_in[STATE_VEC_INDX_VEL_Y]) / self.dt_s;
    x_out[STATE_VEC_INDX_ACC_Z] = (x_out[STATE_VEC_INDX_VEL_Z] 
      - x0_in[STATE_VEC_INDX_VEL_Z]) / self.dt_s;
    x_out
  }
}

/*
 * @brief: This function is to fill fields of the full state vector that are not 
 *         filled by the solver itself.
 * 
 * @details: Note: State augmentation is the process of filling in missing fields
 *                 (field not solved by the solving step) of the full state 
 *                 vector. For computational efficiency state augmentation is 
 *                 done in two steps. 
 *                 (1) augment_state_solve() is called at solving frequency and 
 *                     only contains post-processing computations that need to 
 *                     be done at full frequency (e.g. acceleration from velocity
 *                     derivative)
 *                 (2) augment_state_write() is called at writing frequency and 
 *                     is only called when writing the results to file. This 
 *                     function contains all processing that can be done at a
 *                     lower frequency.
 * 
 *           Post solving computations include: 
 *           * TODO
 * 
 * @param[in] x1_in - Full state vector for current solving step n
 * 
 * @param[in] x0_in - Full state vector for previous solving step n-1
 *  
 * @returns x1_in - Full state vector for current solving step with additional 
 *                  fields filled.
 * 
 */
impl DKE {
  fn augment_state_write(&mut self, x1_inout: &Array1<f64>, x0_in: &Array1<f64>)
  -> Array1<f64>
  {
    /* TODO */
    let mut x_out = (*x1_inout).clone();

    // TODO: This is not to proper way to compute the local altitude by any 
    //       standards and needs to be replaced by calculating the geodetic 
    //       altitude.
    let local_radius_m: f64 = ( x_out[STATE_VEC_INDX_POS_X].powf(2.0)
                              + x_out[STATE_VEC_INDX_POS_Y].powf(2.0) 
                              + x_out[STATE_VEC_INDX_POS_Z].powf(2.0)).sqrt() ;
    x_out[STATE_VEC_INDX_ALTITUDE_PCPF_M] = local_radius_m
      - (self.planet.get_semi_major_axis() + self.planet.get_semi_minor_axis()) * 0.5;

    /* Get local magnitude of the gravitational acceleration */
    x_out[STATE_VEC_INDX_GRAV_ACC_MSS] = get_grav_acc(&x1_inout, &self);

    /* The following is only true if we assume that the Earth is not rotating
     * around it's axis.
     * TODO: Replace with proper implementation */
    if local_radius_m != 0.0
    {
      x_out[STATE_VEC_INDX_POS_PCPF_LAT_DEG] = (x_out[STATE_VEC_INDX_POS_Z] / local_radius_m).asin();

      x_out[STATE_VEC_INDX_POS_PCPF_LONG_DEG] = (x_out[STATE_VEC_INDX_POS_Y] / x_out[STATE_VEC_INDX_POS_X]).atan();
    }
    else 
    {
    println!{"[WRN] Conversion from Cartesian to cylindrical coordinates failed. Radius found to be zero!"};    
    }

    x_out
  }
}

/*
 * @brief: This function is to load all required parameters from configuration 
 *         files before initialising and starting the simulation. This includes 
 *         all parameters for sub-structs for all simulation modules.
 * 
 * @details: TODO
 * 
 */
impl DKE {
  fn load_parameters(&mut self)
  {
    /* Create parameter file instance > sim.ini < */
    let sim_conf = Ini::load_from_file(SIM_PARAMETER_FILE_PATH)
      .expect(&"! [ERROR] ! > sim.ini not found! <".to_string());
    /* -------------------------------------------------------------------------
     * WRITE/PRINT Intervals
     * 
     * -----------------------------------------------------------------------*/
    self.param_sim_print_interval_s = (sim_conf
        .section(Some("print_setting")).unwrap()
        .get("sim_print_interval_s").unwrap())
        .parse::<f64>().unwrap();

    self.param_sim_archive_interval_s = (sim_conf
      .section(Some("write_setting")).unwrap()
      .get("sim_archive_interval_s").unwrap())
      .parse::<f64>().unwrap();

    self.param_sim_archive_flush_interval_s = (sim_conf
      .section(Some("write_setting")).unwrap()
      .get("sim_archive_flush_interval_s").unwrap())
      .parse::<f64>().unwrap();
    /* -------------------------------------------------------------------------
     *      [PLANET]
     * 
     * -----------------------------------------------------------------------*/
    let planet_conf = Ini::load_from_file(PLANET_PARAMETER_FILE_PATH)
      .expect(&"! [ERROR] ! > planet.ini not found! <".to_string());

    self.planet.set_semi_major_axis(&(planet_conf
      .section(Some("general")).unwrap()
      .get("planet_semi_major_axis_m").unwrap())
      .parse::<f64>().unwrap() );

    self.planet.set_semi_minor_axis(&(planet_conf
      .section(Some("general")).unwrap()
      .get("planet_semi_minor_axis_m").unwrap())
      .parse::<f64>().unwrap() );

    self.planet.set_gravitational_constant(&(planet_conf
      .section(Some("general")).unwrap()
      .get("plant_gravitational_constant").unwrap())
      .parse::<f64>().unwrap() );

    self.planet.set_flattening_factor(&(planet_conf
      .section(Some("general")).unwrap()
      .get("planet_flattening_factor").unwrap())
      .parse::<f64>().unwrap() );

    self.planet.set_omega(&(planet_conf
      .section(Some("general")).unwrap()
      .get("planet_omega_rads").unwrap())
      .parse::<f64>().unwrap() );

  }
}