// use iir_filters::errors::Error;
use std::path::Path;

// extern crate tdms;
use tdms::TDMSFile;

// personal functions and struct
pub mod path;
pub mod time_domain;
use crate::time_domain::Signal;


fn main()-> Result<(), Box<dyn std::error::Error>>{

    // let file = std::fs::read_to_string(".")?;
    // be sure of what to plot
    let system_path = String::from("/home/dtos_experiment/Documents/");
    let experiment = String::from("compressed air/");
    let inverter_state = String::from("1");
    let wind_speed = String::from("5");
    let papadakis_sys = String::from("D:/_data/WEL/WEL20220512/");

    let checked_os = match cfg!(target_os = "windows"){
        true => {path::make_the_path(&papadakis_sys,
                                     &experiment,
                                     &inverter_state,
                                     &wind_speed)
        }
        false => {path::make_the_path(&system_path,
                                      &experiment,
                                      &inverter_state,
                                      &wind_speed)}
    };

    println!("{:?}", &checked_os);

    let sig = match TDMSFile::from_path(Path::new(&checked_os)){
        // catch the error
        // dont know exactly
        // how it works
        Ok(f) =>f,
        Err(e) => panic!("{:?}", e),
    };
    // println!("{:?}", sig.segments.len());

    let raw_signal = Signal{data: sig,
                            state: experiment,
                            inv_state_exp: inverter_state,
                            ws: wind_speed};
    // (&raw_signal).print_num_samp::<Error>();
    (&raw_signal).plot_raw_signal("Wind2", &true);
    Ok( () )
}
