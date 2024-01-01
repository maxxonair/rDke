 /*
  * @brief: Reference table column index for the drag coefficient data table.
  *         This constant determines the column index at which the geometric altitude 
  *         reference is saved.
  *  
  */
  pub const SC_DRAG_COEFF_INDX_ALTITUDE: usize = 1;

  /*
   * @brief: Reference table column index for the drag coefficient data table.
   *         This constant determines the column index at which the drag coefficient value
   *         is saved.
   *  
   */
  pub const SC_DRAG_COEFF_INDX_DRAG_COEFF: usize = 0;
     
   /*
    * @brief: Path at which the mean free path table data can be loaded
    *  
    */
    pub const SC_DRAG_COEFF_TABLE_PATH: &str = "assets/spacecraft/cd_over_mach_saturnv.csv";