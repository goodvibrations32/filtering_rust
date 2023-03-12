
// extern crate tdms;
use tdms::TDMSFile;
use native_dialog::{
    FileDialog,MessageDialog, MessageType};

// personal functions and struct
use crate::time_domain::Signal;


pub fn gui_single_file(){
        // -> Result<(), Box<dyn std::error::Error>>{
    let path = FileDialog::new()
        .set_location("~/Documents/data_folder")
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
        .set_title("Do you want to open the file?")
        .set_text(&format!("{:#?}", path))
        .show_confirm()
        .unwrap();

    if yes {
        println!("{:?}", path);
        let sig = match TDMSFile::from_path(&path){
            // catch the error
            // dont know exactly
            // how it works
            Ok(f) =>f,
            Err(e) => panic!("{:?}", e),
        };
        // println!("{:?}", sig.segments.len());
        let experiment = String::from("compressed air/");
        let inverter_state = String::from("1");
        let wind_speed = String::from("5");


        let raw_signal = Signal{data: sig,
                                state: experiment,
                                inv_state_exp: inverter_state,
                                ws: wind_speed};
        // (&raw_signal).print_num_samp::<Error>();
        (&raw_signal).plot_raw_signal("Wind2", &true);

    }

}
