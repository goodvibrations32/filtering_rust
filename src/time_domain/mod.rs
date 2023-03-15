// use iir_filters::{filter::DirectForm2Transposed,
//                   sos::zpk2sos,
//                   filter_design::{butter, FilterType}};
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
  // Ok(());//return Ok( () );
  // todo!("find how to return the filter output");

  /// Plots the signal in time domain.
  /// Parameters
  /// ----------
  /// Takes 2 arguments and plots the signal in
  /// time domain. Also provides information if
  /// the file is found but the channel in the
  /// function described as `witch_channel` is
  /// `"unknown"`.
  /// - witch_channel: "Wind2"
  /// - draw: `true` or `false`
  ///
  /// # Panics
  ///
  /// Panics if .
  ///
  /// # Examples
  ///
  /// ```
  /// use spectrum_in_rust::time_domain::Signal;
  ///
  /// let raw_data = Signal{
  /// ....here we get the signal data from the gui operation} ;
  /// raw_data.plot_raw_signal(witch_channel: "unknown",
  ///                                     draw: true);
  /// ```
  pub fn plot_raw_signal(&self,
                         witch_channel: &str,
                         // plot_title: &str,
                         draw: &bool) {
    let groups = self.data.groups();
    // let single_chann =(self.channels("Wind Measurement")).get(witch_channel);
    groups.iter()
          .for_each(|group| {
            let channels = self.data.channels(group);

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
              let time_output = full_group_iterator
                .map_into::<f64>()
                .collect_vec();

              _i += 1;

              // make the time increment for later usage!!
              let _increment = 1.0/time_output.len() as f64;
              let fs = time_output.len() as f64/7.;

              // best way found for the time domain
              // data in respect to the signal
              let time = linspace(0., time_output.len() as f64 / fs,
                                  time_output.len()).map_into::<f64>()
                .collect_vec();

              if witch_channel == "unknown"{
                println!("channel name {:?} \n\
                          samples = {:?} ~ duration = {:?} s ~ \
                          sampling frequency = {:?} Hz\n",
                         &channel.path,
                         time.len(),
                         time.last().copied(),
                         fs as f32);

              }else {let plot_title =
                     if &channel.path == witch_channel
                     && self.state.contains('c') {
                       "Compressed air"}
                     else{
                       "Inverter"};

                     let mut fg =
                     if *draw && (&channel.path == witch_channel) {
                       Figure::new()}
                     else {
                       println!(
                         "no maching channel in dataset to plot \
                          or you coose only info in the start \n\
                          channel name {:?}",
                         channel.path);
                       continue;};

                     //make the plot
                     let the_title = format!("{} measurements", &plot_title);
                     fg.set_title(&the_title)
                     .axes2d()
                     .set_x_label("Time (s)", &[])
                     .set_y_label("Amplitute of signal", &[])
                     .lines(time, time_output,
                            &[Caption      (&format!(" Inv {:1?} Ws {:2?}",
                                                     self.inv_state_exp,
                                                     self.ws.to_string())),
                              Color        ("#a705b0"),
                              LineWidth    (0.5)]);

                     // check if user wants graph
                     if *draw && (&channel.path == witch_channel){
                       fg.show().unwrap();}

                     else{
                       continue;}

                     // TODO attempt to save interactive semi-done!!
                     let f_type: &str = ".png";
                     let _save_to_file = format!("{}{}",
                                                 plot_title,
                                                 f_type)
                     .replace(' ', "_")
                     .replace('/',"");
              };
            }
          });
  }
}
