/* 
 * @brief: This class contains the core of the dynamic kinematic environment
 *         (DKE). It manages all core modules to run the simulation.
 * 
 * 
 * 
 * 
 */
/* ---------------------------------------------------------------------------*/
/* Import structs */
use crate::dke_core::state::State;

/* Import constants */
use crate::constants::constants::*;

/* Import ODE solver */
use crate::solver::rk4::step;

/* Import Equations of Motion model */
use crate::dke_core::eom::dxdt;

/* Import IO */
use crate::io::write_csv::{*, self};

/* External library imports */
use ndarray::Array1;
use csv::Writer; 

pub struct DKE {
  /* [state time] 
   * @description : Simulation start time
   * @unit        : seconds
   * 
   * */
   sim_start_time_s: f64,
   sim_end_time_s: f64,
   dt_s: f64,
   state: State,
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
      state: State::new()
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
    /* Use time Instant for simulation runtime profiling */
    use std::time::Instant;

    /* Initialize state as vector */
    let mut x_vec = self.state.get_vector();

    /* Calculate total number of simulation steps */
    let num_steps: i64 = ((self.sim_end_time_s - self.sim_start_time_s) 
                              / self.dt_s) as i64;
    
    /* Initialize variable to track simulation time */
    let mut t_sim = self.sim_start_time_s;

    /* Create file writer */
    write_csv::create_csv("./data_out/out.csv".to_string());

    println!("---------------------------------------------------------------");
    println!("              [SIMULATION START]");
    println!("---------------------------------------------------------------");

    /* Create timer for runtime profiling */
    let simulation_timer = Instant::now();

    /* Set counter for print outs while the simulation is running */
    let mut out_counter: f64 = 0.0;

    /* Simulation main loop */
    for sim_step in 0..num_steps {
      if out_counter > SIMULATION_PRINT_INTERVAL_S 
      {
        print!("{:?} - {:.2?} ->> {:.2?} \n", sim_step, t_sim, x_vec[2]);
        out_counter = 0.0;
      }
      else 
      {
        out_counter += self.dt_s
      }
      
      /* Perform integration step with step size dt_s */
      x_vec = step( &x_vec, &dxdt, t_sim, self.dt_s);

      write_csv::append_to_csv("./data_out/out.csv".to_string(), 
                               &x_vec,
                               t_sim).unwrap();

      /* Update simulation time for the next step */
      t_sim += self.dt_s;
    }

    println!("");
    println!("---------------------------------------------------------------");
    println!("              [FINISHED]");
    println!("---------------------------------------------------------------");
    println!("Simulated time          [s] : {:?}", (self.sim_end_time_s
                                                    - self.sim_start_time_s));
    println!("Number of integration steps : {:?}", num_steps);
    println!("Step size               [s] : {:.4?}", self.dt_s);
    println!("Simulation time        [ms] : {:.3?}", simulation_timer
                                                .elapsed()
                                                .as_millis());
    println!("Time per step          [ms] : {:.3?}", (simulation_timer
                                                .elapsed()
                                                .as_millis() as f64) 
                                                    / num_steps as f64);
    println!("---------------------------------------------------------------");

  }
}