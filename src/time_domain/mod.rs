use itertools::Itertools;
extern crate tdms;
use tdms::{data_type::TdmsDataType, TDMSFile};
extern crate itertools;
extern crate itertools_num;
use itertools_num::linspace;
use gnuplot::{Figure, Caption, Color,
              AxesCommon,LineWidth};

#[derive(Debug)]
pub struct Signal <'a> {
    pub data: TDMSFile<'a>,
    pub state: String,
    pub inv_state_exp: String,
    pub ws: String,

}

impl Signal<'_> {

    pub fn plot_signal_in_time_domain (&self,
                             witch_channel: &str,
                             // plot_title: &str,
                             draw: &bool) -> () {
    let groups = self.data.groups();
    // let single_chann =(self.channels("Wind Measurement")).get(witch_channel);
    for group in &groups {
        let channels = self.data.channels(&group);
        // let total = &self.get_channel(ChannelPath("Wind2")); // .get(&witch_channel);
        // let channel_single = &self;

        // println!("{:?}", &channels.path); //("Wind Measurements")
        // println!("{:?}", &channel_single.get_many_mut([2,3])); //("Wind Measurements")
        let mut _i = 0;
        for (_, channel) in channels{
            let full_group = match channel.data_type {
                TdmsDataType::DoubleFloat(_) => self.data.channel_data_double_float(channel),
                _ => {
                    panic!("{}", "channel for data type unimplemented")
                }
            };

            let full_group_iterator = match full_group {
                Ok(i) => i,
                Err(e) => {
                    panic!("{:?}", e)
                }
            };
            let time_output: _ = full_group_iterator.map_into::<f64>().collect_vec();
            println!("length of time signal = {:1?} time name = {:2?} channel name ={:3?}",
                     &time_output.len(), &channel.group_path, &channel.path);
            _i += 1;

            // make the time increment for later usage!!
            let _increment = 1.0/time_output.len() as f64;
            // best way found for the time domain
            // data in respect to the signal
            let time: _ = linspace(0., 7., time_output.len())
                .map_into::<f64>()
                .collect_vec();

            // plot with gnuplot backend with the help of
            // gnuplot crate
            let plot_title =
                if witch_channel == "Wind2" && self.state.find("c") == Some(0)
            {"Compressed air"}
            else{"Inverter"};
            let mut fg = if *draw && channel.path == "Wind2" {Figure::new()} else {continue;};
            let the_title = format!("{} measurements", &plot_title);
            fg.set_title(&the_title)
              .axes2d()
              .set_x_label("Time (s)", &[])
              .set_y_label("Amplitute of signal", &[])
              .lines(time, time_output,
                     &[Caption      (&format!(" Inv {:1?} Ws {:2?}",
                                              self.inv_state_exp,
                                              self.ws)),
                       Color        ("#a705b0"),
                       LineWidth    (0.5)]);

            // check if user wants graph
            if *draw {fg.show().unwrap();} else {break;}

            // TODO attempt to save interactive semi-done!!
            let f_type: &str = ".png";
            let _save_to_file = format!(
                "{}{}", plot_title, f_type)
                .replace(" ", "_")
                .replace("/","");
            // save to parent "project" folder
            // fg.save_to_png(save_to_file, 800, 600);
            // let big_title = &experiment.replace("c","C").replace("/", "");
            // let big_title = &experiment.replace("i","I").replace("/", "");String::from("Wind2")
        }}}
}
