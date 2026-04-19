use rouille::{ Request, Response, router };
use std::sync::{ Arc, Mutex, atomic::{ AtomicBool, Ordering } };
use std::thread;

use crate::init::init;
use crate::dke_core::dke_core::{ DKE, SimulationSummary };

// TODO move to config
const SERVER_IP: &str = "127.0.0.1";
const SERVER_PORT: u16 = 8000;

pub fn start_server() {
  println!("-----------------------------------------------------------------");
  println!("                Starting DKE API server...                       ");
  println!("-----------------------------------------------------------------");
  println!("   ");
  println!("Endpoints:");
  println!("POST /run      - Start a new simulation");
  println!("POST /stop     - Stop the currently running simulation");
  println!("GET  /status   - Check if a simulation is running and get last result");
  println!("GET  /download - Download the last simulation results as CSV");
  println!("   ");
  println!("Starting server on http://{}:{}", SERVER_IP, SERVER_PORT);
  println!("   ");
  println!("   ");
  println!("   ");
  println!("-----------------------------------------------------------------");

  let stop_flag = Arc::new(AtomicBool::new(false));
  let running = Arc::new(AtomicBool::new(false));

  // Shared state (optional, for storing last result path)
  let last_result: Arc<Mutex<Option<SimulationSummary>>> = Arc::new(Mutex::new(None));

  rouille::start_server(format!("{}:{}", SERVER_IP, SERVER_PORT), move |request| {
    let last_result = last_result.clone();

    router!(request,
            // ------------------     API ENDPOINTS     --------------------- //

            // --- RUN SIMULATION --- //
            (POST) (/run) => {
                if running.load(Ordering::Relaxed) {
                    return Response::text("Simulation already running").with_status_code(400);
                }

                running.store(true, Ordering::Relaxed);
                stop_flag.store(false, Ordering::Relaxed);

                let stop_flag_clone = stop_flag.clone();
                let running_clone = running.clone();
                let last_result_clone = last_result.clone();

                thread::spawn(move || {
                    let mut dke = init();
                    let summary = dke.run_simulation(false, stop_flag_clone);

                    *last_result_clone.lock().unwrap() = Some(summary);
                    running_clone.store(false, Ordering::Relaxed);
                });

                Response::text("Simulation started")
            },

            // ---  SIMULATION END STATUS --- //
            (GET) (/status) => {
                if running.load(Ordering::Relaxed) {
                    return Response::text("Simulation running");
                }

                let guard = last_result.lock().unwrap();

                if let Some(summary) = &*guard {
                    Response::json(summary)
                } else {
                    Response::text("No simulation run yet").with_status_code(404)
                }
            },

            // --- STOP/INTERRUPT SIMULATION --- //
            (POST) (/stop) => {
                if !running.load(Ordering::Relaxed) {
                    return Response::text("No simulation running").with_status_code(400);
                }

                stop_flag.store(true, Ordering::Relaxed);

                Response::text("Stop signal sent")
            },

            // --- DOWNLOAD RESULTS --- //
            (GET) (/download) => {
                let guard = last_result.lock().unwrap();

                if let Some(summary) = &*guard {
                    println!("Trying to read file: {}", summary.output_path);
                    match std::fs::read(&summary.output_path) {
                        Ok(data) => {
                            Response::from_data("application/octet-stream", data)
                                .with_additional_header(
                                    "Content-Disposition",
                                    "attachment; filename=\"results.csv\""
                                )
                        },
                        Err(_) => Response::text("Failed to read file").with_status_code(500)
                    }
                } else {
                    Response::text("No simulation run yet").with_status_code(404)
                }
            },

            _ => Response::empty_404()
        )
  });
}
