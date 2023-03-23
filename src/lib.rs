extern crate tdms;
use tdms::TDMSFile;

#[derive(Debug)]
pub struct FilterMods{
  pub order: u32,
  pub cutoff_low: f64,
}

#[derive(Debug)]
pub struct Signal <'a> {
  pub data: TDMSFile<'a>,
  pub state: String,
  pub inv_state_exp: String,
  pub ws: String,
}
#[cfg(test)]
mod tests{

  use std::path::Path;

  use super::*;
  #[test]
  fn check_signal_const(){
    let file = match TDMSFile::from_path(Path::new("/home/dtos_experiment/Documents/data_folder/measurements_12_05_22/new_record_prop_channel/compressed air/ca0_0.1/Data.tdms")) {
      Ok(f) => f,
      Err(e) => panic!("{:?}", e)
    };
    let signal = Signal{
      data: file,
      state: "0".to_string(),
      inv_state_exp: "compressed air".to_string(),
      ws: "0".to_string(),
    };
    assert_eq!(signal.inv_state_exp, "compressed air");
    assert_eq!(signal.ws, "0");
    assert_eq!(signal.data.groups().get(0),
               Some(&String::from("Wind Measurement")));

  }

  #[test]
  fn check_filter_const(){
    let me_butter = FilterMods{
      order: 2,
      cutoff_low: 200.,
    };
    assert_eq!(me_butter.order, 2);
    assert_eq!(me_butter.cutoff_low, 200.)
  }
  #[ignore = "For later"]
  fn _check_the_time_ops() -> ! {
    todo!("Sould test the filtering opperation")
  }
}
