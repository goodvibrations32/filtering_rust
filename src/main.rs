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
              AxesCommon};
use gnuplot::PlotOption::LineWidth;
// use gnuplot::AlignType::AlignCenter;

fn main(){

    let home_docs = "/home/dtos_experiment/Documents/";
    let parent = "data_folder/";
    let new_rec_folder = "measurements_12_05_22/new_record_prop_channel/";
    let comp_air_folder = "compressed air/";
    let state_folder = "ca1_0.1/";
    let file_name = "Data.tdms";
    let full_path = format!(
        "{home_docs}{parent}{new_rec_folder}{comp_air_folder}{state_folder}{file_name}");
    let path = Path::new(&full_path);

    println!("{:?}", path);
    // open a single file and store it in "file"
    let file = match TDMSFile::from_path(
        path
        // Path::new(
        // // &("{:?}",full_path)
        // "/home/dtos_experiment/Documents/data_folder/measurements_12_05_22/new_record_prop_channel/compressed air/ca1_0.1/Data.tdms"
        // )
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
            let y: Vec<f64> = full_channel_iterator
                .map_into::<f64>()
                .collect();

            // make the time increment for later usage!!
            let _increment = 1.0/y.len() as f64;

            // best way found for the time domain
            // data in respect to the signal
            let time: _ = linspace(0., 7., y.len())
                .map_into::<f64>()
                .collect_vec();

            // plot with gnuplot backend with the help of
            // gnuplot crate
            let mut fg = Figure::new();
            fg.set_title("Compressed air measurements")
              .axes2d()
              .set_x_label("Time (s)", &[])
              .set_y_label("Amplitute from dataset", &[])
              .lines(time, y,
                     &[Caption      ("raw data"),
                       Color        ("#a705b0"),
                       LineWidth    (0.5)]);
            // show the plot
            // fg.show().unwrap();
            let f_type = ".png";
            let save_to_file = format!(
                "{}{}", comp_air_folder, f_type)
                .replace(" ", "_")
                .replace("/","");
            // save to parent "project" folder
            // fg.save_to_png(save_to_file, 800, 600);
            }
            _i += 1;
        }
}
