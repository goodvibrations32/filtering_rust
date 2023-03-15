use std::str::Split;
// extern crate tdms;
use tdms::TDMSFile;
use native_dialog::{
    FileDialog,MessageDialog, MessageType};

// personal functions and struct
use crate::time_domain::Signal;


pub fn gui_single_file(data_channel: String, plot: bool){
    // -> Result<(), Box<dyn std::error::Error>>{
    let path = FileDialog::new()
        .set_location("../../data")
    // .add_filter("TDMS dataset", &[".tdms"])
    // .add_filter("JPEG Image", &["csv", "jpeg"])
        .show_open_single_file()
        .unwrap();

    // TODO take the experiment info here
    // from the file dialod path and the
    // cycle of hell will end !!
    let path = match path {
        Some(path) => path,
        None => return,
    };

    let yes = MessageDialog::new()
        .set_type(MessageType::Info)
        .set_text(&format!("Do you want to open following file? \n\
                            {:#?}", path))
        .show_confirm()
        .unwrap();
    match yes {
        true => {
            let speeds = "_0. _5. _10. _15. _20.";
            let each_speed: Split<char> = speeds.split(' ');
            let experiment = match path.clone()
                                       .into_os_string()
                                       .into_string()
                                       .unwrap()
                                       .find("inv"){
                                           Some(..) => "inv",
                                           None => "comp",
                                       };
            let inv_state = match path.clone()
                                      .into_os_string()
                                      .into_string()
                                      .unwrap()
                                      .find("1_") {
                                          Some(..) => 1,
                                          None => 0
                                      };
            // let _each_experiment: Split<&str> = experiment.split(" ");
            println!("{:?}", path);
            for speed in each_speed {

                let checker_inv = path.clone()
                                      .into_os_string()
                                      .into_string()
                                      .unwrap()
                                      .find(experiment);
                let ws = path.clone()
                             .into_os_string()
                             .into_string()
                             .unwrap()
                             .find(speed);

                match checker_inv.is_some() && ws.is_some() {
                    true => {
                        // println!("{:?}  {:?}", path, ws);

                        // println!("{:?}", checker_inv);
                        let sig = match TDMSFile::from_path(&path){
                            Ok(f) =>f,
                            Err(e) => panic!("{:?}", e),
                        };
                        let raw_signal = Signal{data: sig,
                                                state: experiment.to_string(),
                                                inv_state_exp: inv_state.to_string(),
                                                ws: speed.to_string()
                                                .replace(['_','.'], "")};
                        // (&raw_signal).print_num_samp::<Error>();
                        (raw_signal).plot_raw_signal(&data_channel, &plot);
                    }
                    false => continue
                        // println!(
                        // "Consider following a naming convention for the folders i.e. 'caINV_WS.1' (INV=0 or 1 and WS=0,5,10,15,20)"),
                }
            }
        }
        false => println!("You canceled the opening operation!\n Thanks "),
    }
}
