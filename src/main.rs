use itertools::Itertools;
extern crate tdms;
use std::path::Path;
use tdms::data_type::TdmsDataType;
use tdms::TDMSFile;
extern crate itertools;
extern crate itertools_num;
use itertools_num::linspace;
// use vector2d::Vector2D;
use gnuplot::{Figure, Caption, Color,
              AxesCommon,LineWidth};

struct WindSupply {
    type_of_experiment: String,
}
struct SysFolder {
    sys_path: String,
    data_parent: String,
    recording: String,
}
struct InnerExpFolders {
    ca: String,
    inv: String,
}

// windsupply::Compressed(String::from("compressed air"));
fn main(){

    // be sure of what to plot
    let system_path = String::from("/home/dtos_experiment/Documents/");
    // FIXME goes out of bounds and plots 4 figures
    // ONLY when using inverter's folder
    let experiment = String::from("compressed air/");
    // let experiment = String::from("inverter/");
    let inverter_state = String::from("0");
    let wind_speed = String::from("0");

    // make the path
    let sig_path = make_the_path(&system_path,
                                 &experiment,
                                 &inverter_state,
                                 &wind_speed);
    // open a single file and store it in "file"
    println!("{:?}", sig_path);
    let sig = match TDMSFile::from_path(
        Path::new(&sig_path)
    ) {
        // catch the error
        // dont know exactly
        // how it works
        Ok(f) =>f,
        Err(e) => panic!("{:?}", e),
    };
    let big_title = &experiment.replace("c","C").replace("/", "");
    // let big_title = &experiment.replace("i","I").replace("/", "");
    plot_time_domain(sig, &big_title);
}

pub fn make_the_path (local: &str, exp_type: &str, inv: &str, ws: &str) -> String {
    // wind supply method
    let experiment = WindSupply {
        type_of_experiment: String::from(exp_type),
        // compressed: String::from("compressed air/"),
        // wind_tunnel: String::from("inverter/"),
    };

    // system folders such as pc folder, parent of data
    // and recordings
    let sys_folder = SysFolder {
        sys_path: String::from(local),
        data_parent: String::from("data_folder/"),
        recording: String::from("measurements_12_05_22/new_record_prop_channel/"),
    };

    //rename to better stuff but for now i have only one method experiment
    let inverter_and_ws = InnerExpFolders {
        ca: String::from(format!("ca{inv}_{ws}.1/")),
        inv: String::from(format!("in0_0.1/")),
    };

    let local_place = sys_folder.sys_path;
    let parent = sys_folder.data_parent;
    let record = sys_folder.recording;
    let experiment_type = experiment.type_of_experiment;
    // let experiment_state =
    //     if experiment_type == "compressed air/" {
    //         let experiment_state = inverter_and_ws.ca;
    //     }else {
    //         let experiment_state = inverter_and_ws.inv;
    //     };

    let comp = inverter_and_ws.ca;
    // FIXME
    // let strng = inverter_and_ws.inv;
    // FIXME
    const FILE_NAME: &str = "Data.tdms";
    let full_path = format!(
        "{local_place}{parent}{record}{experiment_type}{comp}{FILE_NAME}");
    full_path

}

pub fn plot_time_domain <'a> (file_name: TDMSFile, plot_title: &str) {
    let groups =file_name.groups();
    for group in groups {
        let channels = file_name.channels(&group);
        let mut _i = 0;
        for (_, channel) in channels{
            let full_channel = match channel.data_type {
                TdmsDataType::DoubleFloat(_) => file_name.channel_data_double_float(channel),
                _ => {
                    panic!("{}", "channel for data type unimplemented")
                }
            };

            let full_channel_iterator = match full_channel {
                Ok(i) => i,
                Err(e) => {
                    panic!("{:?}", e)
                }
            };
            let signal_raw: Vec<f64> = full_channel_iterator
                .map_into::<f64>()
                .collect();

            // make the time increment for later usage!!
            let _increment = 1.0/signal_raw.len() as f64;

            // best way found for the time domain
            // data in respect to the signal
            let time: _ = linspace(0., 7., signal_raw.len())
                .map_into::<f64>()
                .collect_vec();

            // plot with gnuplot backend with the help of
            // gnuplot crate
            let mut fg = Figure::new();
            let the_title = format!("{} measurements", &plot_title);
            fg.set_title(&the_title)
              .axes2d()
              .set_x_label("Time (s)", &[])
              .set_y_label("Amplitute of signal", &[])
              .lines(time, signal_raw,
                     &[Caption      ("raw data"),
                       Color        ("#a705b0"),
                       LineWidth    (0.5)]);
            // show the plot
            fg.show().unwrap();
            let f_type: &str = ".png";
            let _save_to_file = format!(
                "{}{}", plot_title, f_type)
                .replace(" ", "_")
                .replace("/","");
            // save to parent "project" folder
            // fg.save_to_png(save_to_file, 800, 600);
        }
        _i += 1;
    }
}
