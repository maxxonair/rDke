use rouille::{ Request, Response, router };
use std::sync::{ Arc, Mutex };

use crate::init::init;
use crate::dke_core::dke_core::DKE;

pub fn start_server() {
  println!("Starting server on http://localhost:8000");

  // Shared state (optional, for storing last result path)
  let last_result: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

  // TODO move IP and port to config file
  rouille::start_server("0.0.0.0:8000", move |request| {
    let last_result = last_result.clone();

    router!(request,
            // ------------------     API ENDPOINTS     --------------------- //

            // --- RUN SIMULATION --- //
            (POST) (/run) => {
                let mut dke = init();
                let run_verbose = false;

                let summary = dke.run_simulation(run_verbose);

                *last_result.lock().unwrap() = Some(summary.output_path.clone());

                Response::json(&summary)
            },

            // --- DOWNLOAD RESULTS --- //
            (GET) (/download) => {
                let guard = last_result.lock().unwrap();

                if let Some(path) = &*guard {
                    match std::fs::read(path) {
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
