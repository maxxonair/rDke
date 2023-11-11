
use chrono::{Local, DateTime};
pub struct RLog 
{
   msg_counter: i64,

   msg_type_counter: i64,
   wrn_type_counter: i64,
   err_type_counter: i64,
   dbg_type_counter: i64,

   enable_debug_msgs: bool
}

impl RLog {
  pub fn new() -> RLog {
    println!("-------------------------------------------");
    println!("+++++++++++  [ start log ]   ++++++++++++++");
    println!("-------------------------------------------");
    println!("  {}", Local::now().format("%Y-%m-%d %H:%M:%S %z").to_string());
    println!("");
    RLog {
      msg_counter: 0,
      msg_type_counter: 0,
      wrn_type_counter: 0,
      err_type_counter: 0,
      dbg_type_counter: 0,
      enable_debug_msgs: false
    }
  }
}

impl RLog {
  /* ------------------------------------------------------------------------ */
  /*    [setters]   */
  /* ------------------------------------------------------------------------ */
  pub fn set_enable_debug_messages(&mut self, flag_in: &bool) {
    self.enable_debug_msgs = *flag_in;
  }
 }



impl RLog {
  /* ------------------------------------------------------------------------ */
  /*                      [support functions] */
  /* ------------------------------------------------------------------------ */
  pub fn create_counter_string(&self)
  -> String
  {
    let mut str_out: String = "".to_owned();
    if self.msg_counter < 10
    {
      str_out = format!("[000{}] ", &self.msg_counter);
    }
    else if self.msg_counter < 100
    {
      str_out = format!("[00{}] ", self.msg_counter);
    }
    else if self.msg_counter < 1000
    {
      str_out = format!("[0{}] ", self.msg_counter);
    }
    else
    {
      str_out = format!("[{}] ", self.msg_counter);
    }
    return String::from(str_out);
  }

 }

impl RLog {
  /* ------------------------------------------------------------------------ */
  /*                      [message functions] */
  /* ------------------------------------------------------------------------ */
  pub fn rLogMsg(&mut self, msg_str_in: &str) {
    /* Initialize complete message string */
    let mut msg_string: String = "".to_owned();
    /* Add message count */
    msg_string.push_str(self.create_counter_string().as_str());
    /* Add message type  */
    msg_string.push_str("[MSG] ");
    /* Add message content */
    msg_string.push_str(msg_str_in);
    /* -> Print complete message string */
    println!("{}", msg_string);
    /* Increment message type counter */
    self.msg_type_counter += 1;
    /* Increment total message counter */
    self.msg_counter += 1;
  }

  pub fn rLogWrn(&mut self, msg_str_in: &str) {
    /* Initialize complete message string */
    let mut msg_string: String = "".to_owned();
    /* Add message count */
    msg_string.push_str(self.create_counter_string().as_str());
    /* Add message type  */
    msg_string.push_str("[WRN] ");
    /* Add message content */
    msg_string.push_str(msg_str_in);
    /* -> Print complete message string */
    println!("{}", msg_string);
    /* Increment message type counter */
    self.wrn_type_counter += 1;
    /* Increment total message counter */
    self.msg_counter += 1;
  }


  pub fn rLogErr(&mut self, msg_str_in: &str) {
    /* Initialize complete message string */
    let mut msg_string: String = "".to_owned();
    /* Add message count */
    msg_string.push_str(self.create_counter_string().as_str());
    /* Add message type  */
    msg_string.push_str("[ERR] !! ");
    /* Add message content */
    msg_string.push_str(msg_str_in);
    /* -> Print complete message string */
    println!("{}", msg_string);
    /* Increment message type counter */
    self.err_type_counter += 1;
    /* Increment total message counter */
    self.msg_counter += 1;
  }

  pub fn rLogDbg(&mut self, msg_str_in: &str) {
    /* Only show debug messages if enabled */
    if self.enable_debug_msgs == true
    {
      /* Initialize complete message string */
      let mut msg_string: String = "".to_owned();
      /* Add message count */
      msg_string.push_str(self.create_counter_string().as_str());
      /* Add message type  */
      msg_string.push_str("[DBG] ");
      /* Add message content */
      msg_string.push_str(msg_str_in);
      /* -> Print complete message string */
      println!("{}", msg_string);
      /* Increment message type counter */
      self.msg_type_counter += 1;
      /* Increment total message counter */
      self.dbg_type_counter += 1;
    }
  }

  pub fn close(&mut self) {
      /* -> Print complete message string */
      println!("-------------------------------------------");
      println!("++++++++  [ close message log ]  ++++++++++");
      println!("-------------------------------------------");
      println!("  {}", Local::now().format("%Y-%m-%d %H:%M:%S %z").to_string());
      println!("");
      println!("    MSG: {}", self.msg_type_counter);
      if self.wrn_type_counter == 0
      {
        println!("[x] WRN: {}", self.wrn_type_counter);
      }
      else 
      {
        println!("[!] WRN: {} <----- !", self.wrn_type_counter); 
      }
      if self.err_type_counter == 0
      {
        println!("[x] ERR: {}", self.err_type_counter);
      }
      else 
      {
        println!("[!] ERR: {} <------ !!!", self.err_type_counter);
      }
      println!("    DBG: {}", self.dbg_type_counter);
      println!("-------------------------------------------");
    }
 }