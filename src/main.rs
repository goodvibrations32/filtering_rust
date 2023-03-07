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
/*
use gnuplot::{PlotOption,LineWidth};
use gnuplot::AlignType::AlignCenter;
*/

struct WindSupply {
    compressed: String,
    wind_tunnel: String,
}
struct SysFolder {
    sys_path: String,
    data_parent: String,
    recording: String,
}
struct InnerExpFolders {
    ca_inv_on_ws_0: String,
    // ca_inv_on_ws_5: String,
    // ca_inv_on_ws_10: String,
    // ca_inv_off_ws_0: String,
    // ca_inv_off_ws_5: String,
    // ca_inv_off_ws_10: String,
    // inv_inv_off_ws_0: String,
    // inv_inv_on_ws_0: String,
    // inv_inv_on_ws_5: String,
    // inv_inv_on_ws_10: String,
    // inv_inv_on_ws_15: String,
    // inv_inv_on_ws_20: String,

}
// WindSupply::Compressed(String::from("compressed air"));
fn main(){

    // wind supply method
    let experiment = WindSupply {
        compressed: String::from("compressed air/"),
        wind_tunnel: String::from("inverter/"),
    };

    // system folders such as pc folder, parent of data
    // and recordings
    let sys_folder = SysFolder {
        sys_path: String::from("/home/dtos_experiment/Documents/"),
        data_parent: String::from("data_folder/"),
        recording: String::from("measurements_12_05_22/new_record_prop_channel/"),
    };

    //rename to better stuff but for now i have only one method experiment
    let inverter_and_ws = InnerExpFolders {
        ca_inv_on_ws_0: String::from("ca1_0.1/"),
        // ca_inv_on_ws_5: String::from("ca1_5.1/"),
        // ca_inv_on_ws_10: String::from("ca1_10.1/"),
        // ca_inv_off_ws_0: String::from("ca0_0.1/"),
        // ca_inv_off_ws_5: String::from("ca0_5.1/"),
        // ca_inv_off_ws_10: String::from("ca0_10.1/"),

        // inv_inv_off_ws_0: String::from("in0_0.1/"),
        // inv_inv_on_ws_0: String::from("in1_0.1/"),
        // inv_inv_on_ws_5: String::from("in1_5.1/"),
        // inv_inv_on_ws_10: String::from("in1_10.1/"),
        // inv_inv_on_ws_15: String::from("in1_15.1/"),
        // inv_inv_on_ws_20: String::from("in1_20.1/"),
    };
    let local_place = sys_folder.sys_path;
    let parent = sys_folder.data_parent;
    let record = sys_folder.recording;
    let experiment_type = experiment.compressed;
    let experiment_state = inverter_and_ws.ca_inv_on_ws_0;
    const FILE_NAME: &str = "Data.tdms";
    let full_path = format!(
        "{local_place}{parent}{record}{experiment_type}{experiment_state}{FILE_NAME}");
    let path = Path::new(&full_path);
    // if assert!(!path.try_exists().expect("no file BRO")) {
    //     path = Path::new("D:/_data/WEL/WEL20220512/")

    //     else {
    //         path = path
    //     }
    // }
    println!("{:?}", path);
    // open a single file and store it in "file"
    let file = match TDMSFile::from_path(
        path
    ) {
        // catch the error
        // dont know exactly
        // how it works
        Ok(f) =>f,
        Err(e) => panic!("{:?}", e),
    };

    // iterate in the file by the given group
    // name from tdms object
    let groups =file.groups();
    for group in groups {
        let channels = file.channels(&group);
        // println!("chann-name {:?}", channels);
        let mut _i = 0;
        for (_, channel) in channels{
            // let mut data = file.channel_data_single_float(channel);
            let full_channel = match channel.data_type {
                TdmsDataType::DoubleFloat(_) => file.channel_data_double_float(channel),
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
            fg.set_title("Compressed air measurements")
              .axes2d()
              .set_x_label("Time (s)", &[])
              .set_y_label("Amplitute of signal", &[])
              .lines(time, signal_raw,
                     &[Caption      ("raw data"),
                       Color        ("#a705b0"),
                       LineWidth    (0.5)]);
            // show the plot
            // fg.show().unwrap();
            let f_type: &str = ".png";
            let _save_to_file = format!(
                "{}{}", experiment_type, f_type)
                .replace(" ", "_")
                .replace("/","");
            // save to parent "project" folder
            // fg.save_to_png(save_to_file, 800, 600);
            }
            _i += 1;
        }
}
