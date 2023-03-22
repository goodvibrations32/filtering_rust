use std::str::Split;
// extern crate tdms;
use tdms::TDMSFile;
use native_dialog::{
    FileDialog,MessageDialog, MessageType};

// personal functions and struct
use crate::time_domain::Signal;

/// # Choosing a file and extract the data.
/// Here a possible channel name is given
/// from the user by defining the variable
/// `data_channel`. A specific channel name
/// is used as `unknown` in order to have
/// the option of oblivion. Meaning not
/// knowing whitch channel name to put
/// there.
///
/// Parameters
/// ----
/// - `data_channel`: `String` :
///     The wanted channel name as
///     literal string. This variable
///     is determined in the main.rs
///     file.
/// - `plot`: `true` or `false` :
///     Whether to plot a graph of
///     the found signal or just the
///     information.
///
/// # Panics
/// Panics if the file provided is not
/// a TDMS dataframe type (`.tdms`).
///
/// # Examples
///
/// ## Plotting a known channel
/// If there is a known channel name passing
/// the name in `"....".to_string()` and the
/// choise to plot true a graph will be
/// produced using gnuplot which if you
/// run this correctly should be installed
/// as a dependency.
/// ```
/// simple_gui::gui_single_file("Wind2".to_string(),true)
/// ```
/// ## Asking for information.
/// If the channel name is yet unknown passing
/// exactly that as the name and just shifting
/// `plot` to false will give some information
/// if the file `is .tdms` and there were data
/// there.
/// ```
/// simple_gui::gui_single_file("unknown".to_string(), false)
/// ```
pub fn gui_single_file(data_channel: String,
                       plot: bool){

    let path = FileDialog::new()
        .show_open_single_file();
    let temp = path.unwrap();

    let path = match &temp {
        Some(temp) => temp,
        None => return,
    };

    let yes = MessageDialog::new()
        .set_text(&format!("Do you want to open following file? \n\
                            {:#?}", path))
        .set_type(MessageType::Warning)
        .show_confirm();
    match yes.unwrap() {
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
                        let sig = match TDMSFile::from_path(path){
                            Ok(f) =>f,
                            Err(e) => panic!("{:?}", e),
                        };
                        let raw_signal = Signal{data: sig,
                                                state: experiment.to_string(),
                                                inv_state_exp: inv_state.to_string(),
                                                ws: speed.to_string()
                                                .replace(['_','.'], "")};
                        (raw_signal).plot_in_time_dom(&data_channel, &plot);
                    }
                    false => continue
                }
            }
        }
        false => println!("You canceled the opening operation!\n Thanks "),
    }
}
