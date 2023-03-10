use iir_filters::{filter_design::{butter, FilterType}, sos::zpk2sos, filter::DirectForm2Transposed};
use itertools::Itertools;
extern crate tdms;
use tdms::{data_type::TdmsDataType, TDMSFile, channel_iter::ChannelDataIter};
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
// trait Filtering {
//     fn plot_filt_over_raw (&self) -> Result<(), Box<dyn std::error::Error>> {
//         let order = 5;
//         let cutoff = 10.0;
//         let fs = 44210.0;
//         let zpk = butter(order, FilterType::LowPass(cutoff), fs)?;
//         let sos  = zpk2sos(&zpk, None)?;
//         let dft2 = DirectForm2Transposed::new(&sos);
//         return Ok( () );
//     }
// }

impl Signal<'_> {
   pub fn filter_butter(&self,order: u32, cutoff: f64) -> Result<(), Box<dyn std::error::Error>>{

        let groups = self.data.groups();
        // let single_chann =(self.channels("Wind Measurement")).get(witch_channel);
        for group in &groups {
            let channels = self.data.channels(&group);

            // begin the search through files
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
                // store the signal somewhere
                let time_output: _ = full_group_iterator
                    .map_into::<f64>()
                    .collect_vec();

                // print dataset groups information
                // println!("signal length ={:1?} group ={:2?} channel ={:3?}",
                //          &time_output.len(), &channel.group_path, &channel.path);
                _i += 1;

                // make the time increment for later usage!!
                let _increment = 1.0/time_output.len() as f64;
                let fs = time_output.len() as f64/7.;
                // best way found for the time domain
                // data in respect to the signal
                let time: _ = linspace(0., time_output.len() as f64/fs, time_output.len())
                    .map_into::<f64>()
                    .collect_vec();
                let cutoff_low: f64 = 10.;
                let cutoff_hi: f64 = cutoff;
                let zpk = butter(order, FilterType::BandPass(cutoff_low, cutoff_hi),fs)?;
                let sos = zpk2sos(&zpk, None)?;

                let mut dft2 = DirectForm2Transposed::new(&sos);
                let mut output:Vec<f64> = vec![];

                for x in time_output.iter() {
                    output.push( iir_filters::filter::Filter::filter(&mut dft2, *x) );
                    return {x; Ok( () )};
                }
            }
        }
        Ok(())//return Ok( () );
   }
    pub fn plot_signal_in_time_domain (&self,
                                       witch_channel: &str,
                                       // plot_title: &str,
                                       draw: &bool) -> () {
        let groups = self.data.groups();
        // let single_chann =(self.channels("Wind Measurement")).get(witch_channel);
        for group in &groups {
            let channels = self.data.channels(&group);

            // begin the search through files
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
                // store the signal somewhere
                let time_output: _ = full_group_iterator
                    .map_into::<f64>()
                    .collect_vec();

                // print dataset groups information
                // println!("signal length ={:1?} group ={:2?} channel ={:3?}",
                //          &time_output.len(), &channel.group_path, &channel.path);
                _i += 1;

                // make the time increment for later usage!!
                let _increment = 1.0/time_output.len() as f64;
                let fs = time_output.len() as f64/7.;
                // best way found for the time domain
                // data in respect to the signal
                let time: _ = linspace(0., time_output.len() as f64/fs, time_output.len())
                    .map_into::<f64>()
                    .collect_vec();
                if witch_channel == "unknown"{
                    println!("channel name   {:?}", &channel.path);
                    println!("  time vector length ={:?}",
                             time.len());
                    println!("  sampling freq ={:?} Hz",
                             fs as f32);
                    println!("  last time item ={:?} s",
                             time.last().copied());
                    // println!("signal length ={:1?} group ={:2?} channel ={:3?}",
                    //  &time_output.len(), &channel.group_path, &channel.path);
                }else {let plot_title =
                       if channel.path == witch_channel
                       && self.state.find("c") == Some(0){
                           "Compressed air"}
                       else{
                           "Inverter"};

                       let mut fg =
                       if *draw && &channel.path == witch_channel {
                           Figure::new()}else {continue;};

                       //make the plot
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
                       if *draw && &channel.path == witch_channel {fg.show().unwrap();} else {continue;}

                       // TODO attempt to save interactive semi-done!!
                       let f_type: &str = ".png";
                       let _save_to_file = format!(
                           "{}{}", plot_title, f_type)
                       .replace(" ", "_")
                       .replace("/","");
                };

            }}}
}
