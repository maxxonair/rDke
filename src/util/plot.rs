/* Include external crates */
use plotters::prelude::*;
use plotters::coord::combinators::IntoLogRange;

use crate::io::read_csv::read_csv_column_f64;
use crate::math::vec_math::{scale_vec_f64, find_max_vec_f64, find_min_vec_f64};

/* Import constants */
use crate::constants::state::*;
/*
 * ----------------------------------------------------------------------------
 *                          [PUBLIC FUNCTIONS]
 * ----------------------------------------------------------------------------
 */

/* 
  * @brief: Plot S/C ground track as longitude vs latitude
  * 
  * @description: Function to 
  *               (1) read the required data from the result csv file
  *               (2) plot the S/C position as longitude vs latitude in PCPF frame.
  *               (3) save chart as png 
  * 
  * @returns: true if any exit condition is met, false otherwise.
  */
pub fn plot_sc_groundtrack(filepath: &str) 
-> Result<(), Box<dyn std::error::Error>>
{
  /* Read S/C longitude and latitude coordinates from results file */
  let latitude_deg: Vec<f64> = read_csv_column_f64(&filepath, true, STATE_VEC_INDX_POS_PCPF_LAT_DEG);
  let longitude_deg: Vec<f64> = read_csv_column_f64(&filepath, true, STATE_VEC_INDX_POS_PCPF_LONG_DEG);

  let img_resolution: (u32, u32) = (800, 600);
  /* Create line chart */
  create_scatter_chart(longitude_deg, 
                       latitude_deg, 
                       "data_out/charts/ground_track_overview.png",
                       (-180.0, 180.0),
                       (-90.0, 90.0),
                       ("longitude [deg]", "latitude [deg]"),
                       "S/C ground track",
                      img_resolution).unwrap();
  Ok(())
}

 /* 
  * @brief: Plot S/C altitude over longitude
  * 
  * @description: Function to 
  *               (1) read the required data from the result csv file
  *               (2) plot the S/C position as longitude vs latitude in PCPF frame.
  *               (3) save chart as png 
  * 
  * @returns: true if any exit condition is met, false otherwise.
  */
  pub fn plot_sc_altitude_vs_longitude(filepath: &str) 
  -> Result<(), Box<dyn std::error::Error>>
  {
    /* Read S/C longitude and latitude coordinates from results file */
    let altitude_km: Vec<f64> = scale_vec_f64(read_csv_column_f64(&filepath, true, STATE_VEC_INDX_ALTITUDE_PCPF_M),
                                              0.001);
    let max_alt_limit: f64 = find_max_vec_f64(&altitude_km) * 1.1;
    let min_alt_limit: f64 = find_min_vec_f64(&altitude_km) * 0.9;
    let longitude_deg: Vec<f64> = read_csv_column_f64(&filepath, true, STATE_VEC_INDX_POS_PCPF_LONG_DEG);
    
    let img_resolution: (u32, u32) = (800, 600);
    /* Create line chart */
    create_scatter_chart(longitude_deg, 
                         altitude_km, 
                         "data_out/charts/altitude_over_longitude.png",
                        (-180.0, 180.0),
                        (min_alt_limit, max_alt_limit),
                        ("longitude [deg]", "altitude [m]"),
                      "S/C altitude vs longitude",
                      img_resolution).unwrap();
    Ok(())
  }

 /* 
  * @brief: Plot S/C aerodynamic drag coefficient over altitude
  * 
  * @description: Function to 
  *               (1) read the required data from the result csv file
  *               (2) plot the S/C position as longitude vs latitude in PCPF frame.
  *               (3) save chart as png 
  * 
  * @returns: true if any exit condition is met, false otherwise.
  */
  pub fn plot_sc_dragcoeff_vs_altitude(filepath: &str) 
  -> Result<(), Box<dyn std::error::Error>>
  {
    /* Read S/C longitude and latitude coordinates from results file */
    let altitude_km: Vec<f64> = scale_vec_f64(read_csv_column_f64(&filepath, true, STATE_VEC_INDX_ALTITUDE_PCPF_M),
                                              0.001);
    let max_alt_limit: f64 = find_max_vec_f64(&altitude_km) ;
    let min_alt_limit: f64 = find_min_vec_f64(&altitude_km) ;

    let drag_coeff: Vec<f64> = read_csv_column_f64(&filepath, true, STATE_VEC_INDX_DRAG_COEFF);
    let max_drag_limit: f64 = find_max_vec_f64(&drag_coeff) * 1.05;
    let min_drag_limit: f64 = find_min_vec_f64(&drag_coeff) * 0.95;

    let img_resolution: (u32, u32) = (800, 600);
    /* Create line chart */
    create_scatter_chart(altitude_km, 
                         drag_coeff, 
                         "data_out/charts/drag_coeff_over_altitude.png",
                         (min_alt_limit, max_alt_limit),
                         (min_drag_limit, max_drag_limit),
                        ("drag coefficient [-]", "altitude [m]"),
                      "S/C aerodynamic drag coefficient vs altitude",
                      img_resolution).unwrap();
    Ok(())
  }

/* 
  * @brief: Plot S/C altitude over simulation time
  * 
  * @description: Function to 
  *               (1) read the required data from the result csv file
  *               (2) plot the S/C position as longitude vs latitude in PCPF frame.
  *               (3) save chart as png 
  * 
  * @returns: true if any exit condition is met, false otherwise.
  */
  pub fn plot_sc_altitude_vs_simtime(filepath: &str) 
  -> Result<(), Box<dyn std::error::Error>>
  {
    /* Read S/C longitude and latitude coordinates from results file */
    let altitude_km: Vec<f64> = scale_vec_f64(read_csv_column_f64(&filepath, true, STATE_VEC_INDX_ALTITUDE_PCPF_M),
                                              0.001);
    let max_alt_limit: f64 = find_max_vec_f64(&altitude_km) * 1.1;
    let min_alt_limit: f64 = find_min_vec_f64(&altitude_km) * 0.9;

    let simtime_s: Vec<f64> = read_csv_column_f64(&filepath, true, STATE_VEC_INDX_SIM_TIME);
    let max_simtime_limit: f64 = find_max_vec_f64(&simtime_s) ;

    let img_resolution: (u32, u32) = (800, 600);
    /* Create line chart */
    create_scatter_chart(simtime_s, 
                         altitude_km, 
                         "data_out/charts/altitude_over_simtime.png",
                        (0.0, max_simtime_limit),
                        (min_alt_limit, max_alt_limit),
                  ("simtime [s]", "altitude [m]"),
                      "S/C altitude",
                      img_resolution).unwrap();
    Ok(())
  }

/* 
  * @brief: Plot S/C velocity over altitude
  * 
  * @description: Function to 
  *               (1) read the required data from the result csv file
  *               (2) plot the S/C position as longitude vs latitude in PCPF frame.
  *               (3) save chart as png 
  * 
  * @returns: true if any exit condition is met, false otherwise.
  */
  pub fn plot_sc_velocity_altitude(filepath: &str) 
  -> Result<(), Box<dyn std::error::Error>>
  {
    /* Read S/C longitude and latitude coordinates from results file */
    let altitude_km: Vec<f64> = scale_vec_f64(read_csv_column_f64(&filepath, true, STATE_VEC_INDX_ALTITUDE_PCPF_M),
                                              0.001);
    let max_alt_limit: f64 = find_max_vec_f64(&altitude_km) * 1.1;
    let min_alt_limit: f64 = find_min_vec_f64(&altitude_km) * 0.9;

    let velocity: Vec<f64> = read_csv_column_f64(&filepath, true, STATE_VEC_INDX_VEL_MAGN_PCI_MS);
    let max_velocity_limit: f64 = find_max_vec_f64(&velocity) ;

    let img_resolution: (u32, u32) = (800, 600);
    /* Create line chart */
    create_scatter_chart(velocity, 
                         altitude_km, 
                         "data_out/charts/altitude_over_velocity.png",
                        (0.0, max_velocity_limit),
                        (min_alt_limit, max_alt_limit),
                  ("velocity [m/s]", "altitude [km]"),
                      "S/C velocity vs altitude",
                      img_resolution).unwrap();
    Ok(())
  }

/* 
  * @brief: Plot S/C density over altitude
  * 
  * @description: Function to 
  *               (1) read the required data from the result csv file
  *               (2) plot the S/C position as longitude vs latitude in PCPF frame.
  *               (3) save chart as png 
  * 
  * @returns: true if any exit condition is met, false otherwise.
  */
  pub fn plot_atmos_density_altitude(filepath: &str) 
  -> Result<(), Box<dyn std::error::Error>>
  {
    /* Read S/C longitude and latitude coordinates from results file */
    let altitude_km: Vec<f64> = scale_vec_f64(read_csv_column_f64(&filepath, true, STATE_VEC_INDX_ALTITUDE_PCPF_M),
                                              0.001);
    let max_alt_limit: f64 = find_max_vec_f64(&altitude_km) ;
    let min_alt_limit: f64 = find_min_vec_f64(&altitude_km) ;

    let density_kgmmm: Vec<f64> = read_csv_column_f64(&filepath, true, STATE_VEC_INDX_ATMOS_DENSITY);
    let max_density_limit: f64 = find_max_vec_f64(&density_kgmmm) * 1.1;
    let min_density_limit: f64 = find_min_vec_f64(&density_kgmmm) * 0.01;

    let img_resolution: (u32, u32) = (800, 600);
    /* Create line chart */
    create_scatter_chart_logy(altitude_km, 
      density_kgmmm, 
                         "data_out/charts/density_over_altitude.png",
                         (min_alt_limit, max_alt_limit),
                        (min_density_limit, max_density_limit),
                  ("altitude [km]", "density [kg/m3]"),
                      "Atmospheric density vs altitude",
                      img_resolution).unwrap();
    Ok(())
  }
/*
 * ----------------------------------------------------------------------------
 *                          [PRIVATE FUNCTIONS]
 * ----------------------------------------------------------------------------
 */
/* 
  * @brief: Private function to create and save 
  * 
  * @description: TODO
  * 
  * @returns: true if any exit condition is met, false otherwise.
  */
fn create_line_chart(x_vals: Vec<f64>, y_vals: Vec<f64>, file_path: &str)
-> Result<(), Box<dyn std::error::Error>>
{
  let root = BitMapBackend::new(file_path, (800, 600)).into_drawing_area();
  root.fill(&WHITE)?;

  // Define the chart area
  let mut chart = ChartBuilder::on(&root)
      .caption("Line Chart", ("sans-serif", 40).into_font())
      .margin(10)
      .x_label_area_size(30)
      .y_label_area_size(30)
      .build_cartesian_2d(-180.0..180.0, -90.0..90.0)?;

  // Plot the data using LineSeries
  chart
      .configure_mesh()
      .draw()?;

  chart
      .draw_series(LineSeries::new(
        x_vals.iter().zip(y_vals.iter()).map(|(&x, &y)| (x, y)),
          &BLUE,
      ))?;
  Ok(())
}

/* 
  * @brief: Private function to create and save 
  * 
  * @description: TODO
  * 
  * @returns: true if any exit condition is met, false otherwise.
  */
  fn create_scatter_chart(x_vals: Vec<f64>, y_vals: Vec<f64>, file_path: &str, 
    xlims: (f64, f64), ylims: (f64, f64), axis_label_xy: (&str, &str), title: &str, 
    resolution: (u32, u32))
  -> Result<(), Box<dyn std::error::Error>>
  {
    let marker_size: i32 = 1;
    let root = BitMapBackend::new(file_path, resolution).into_drawing_area();
    root.fill(&WHITE)?;
  
    /* Define the chart area */ 
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 20).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .build_cartesian_2d(xlims.0..xlims.1, ylims.0..ylims.1)?;
  
    /* Plot the data using LineSeries */
    chart
        .configure_mesh()
        .x_desc(axis_label_xy.0)
        .y_desc(axis_label_xy.1)
        // .x_label_style(("helvetica", 13).into_font())
        // .y_label_style(("sans-serif", 13).into_font())
        .draw()?;
  
    chart
    .draw_series(x_vals.iter().zip(y_vals.iter()).map(|(&x, &y)| {
        Circle::new((x, y), marker_size, &BLUE)
    }))?;

    Ok(())
  }

/* 
  * @brief: Private function to create and save 
  * 
  * @description: TODO
  * 
  * @returns: true if any exit condition is met, false otherwise.
  */
  fn create_scatter_chart_logy(x_vals: Vec<f64>, y_vals: Vec<f64>, file_path: &str, 
    xlims: (f64, f64), ylims: (f64, f64), axis_label_xy: (&str, &str), title: &str, 
    resolution: (u32, u32))
  -> Result<(), Box<dyn std::error::Error>>
  {
    let marker_size: i32 = 1;
    let root = BitMapBackend::new(file_path, resolution).into_drawing_area();
    root.fill(&WHITE)?;

    /* Define the chart area */ 
    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 20).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(50)
        .build_cartesian_2d(xlims.0..xlims.1, (ylims.0..ylims.1).log_scale())?;
  
    /* Plot the data using LineSeries */
    chart
        .configure_mesh()
        .x_desc(axis_label_xy.0)
        .y_desc(axis_label_xy.1)
        // .x_label_style(("helvetica", 13).into_font())
        // .y_label_style(("sans-serif", 13).into_font())
        .draw()?;
  
    chart
    .draw_series(x_vals.iter().zip(y_vals.iter()).map(|(&x, &y)| {
        Circle::new((x, y), marker_size, &BLUE)
    }))?;

    Ok(())
  }