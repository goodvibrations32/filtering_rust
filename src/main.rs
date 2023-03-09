use std::path::Path;

// extern crate tdms;
use tdms::TDMSFile;

// personal make path function
mod path;
mod time_domain;

fn main(){

    // be sure of what to plot
    let system_path = String::from("/home/dtos_experiment/Documents/");
    let experiment = String::from("inverter/");
    let inverter_state = String::from("1");
    let wind_speed = String::from("5");

    // make the path
    let sig_path = path::make_the_path(&system_path,
                                       &experiment,
                                       &inverter_state,
                                       &wind_speed);
    // open a single file and store it in "file"

    let sys_path = if Path::new(&sig_path).is_dir() {
       let sp = String::from("D:/_data/WEL/WEL20220512/");
        let papadakis_sys = &sp;
        path::make_the_path(papadakis_sys,
                            &experiment,
                            &inverter_state,
                            &wind_speed)

    }else{String::from(sig_path)};
    println!("{:?}", sys_path);


    let sig = match TDMSFile::from_path(
        Path::new(&sys_path)
    ) {
        // catch the error
        // dont know exactly
        // how it works
        Ok(f) =>f,
        Err(e) => panic!("{:?}", e),
    };
    // println!("{:?}", sig.segments.len());

    let raw_signal = time_domain::Signal{
        data: sig,
        state: experiment,
        inv_state_exp: inverter_state,
        ws: wind_speed,
    };
    raw_signal.plot_signal_in_time_domain("Wind2", &true)
}

