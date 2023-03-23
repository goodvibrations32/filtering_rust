use itertools::Itertools;
extern crate tdms;
use tdms::{data_type::TdmsDataType, TDMSFile};

extern crate itertools_num;
use itertools_num::linspace;

use gnuplot::{Figure, Caption, Color,
              AxesCommon,LineWidth};

use crate::filtering::FilterMods;

/// This Struct is used to store the
/// necessary information to process
/// and or produce a plot of a signal
/// in the time domain.
pub struct Signal <'a> {
  /// A parsed tdms file which
  /// contains all nessecary
  /// info.
  pub data: TDMSFile<'a>,
  /// The experiment state
  /// this is taken from the
  /// simple file dialog choise.
  pub state: String,
  /// The inverter state whethere
  /// `ON` or `OFF`. We extract
  /// this info from the folder
  /// naming convention.
  pub inv_state_exp: String,
  /// The wind speed feeded to
  /// the sensor taken from the
  /// folder naming convention
  /// also.
  pub ws: String,

}

impl Signal<'_> {
  /// # Plots the signal in time domain.
  /// Here for plotting gnuplot is used and decleared
  /// as a dependency in Cargo.toml among other deps.
  /// Takes 2 arguments and plots the signal in
  /// time domain. Also provides information if
  /// the file is found but the variable `witch_channel`
  /// is set to `"unknown"`. This is choosed in the
  /// first message dialog box. The variable `draw`
  /// is internal and used for separate the operation
  /// of giving the channel info from the plotting to
  /// avoid crashes for the `unknown` option.
  /// # Parameters
  /// - witch_channel: `String`
  /// - draw: `true` or `false`.
  ///
  /// # Panics
  ///
  /// Panics if there is nothing in the given
  /// channel.
  ///
  /// # Example
  ///
  /// ```
  /// use spectrum_in_rust::time_domain::Signal;
  ///
  /// let raw_data = Signal{
  /// ....here we get the signal data from the gui operation} ;
  /// raw_data.plot_raw_signal(witch_channel: "unknown",
  ///                                     draw: true);
  /// ```
  pub fn plot_in_time_dom (&self,
                           witch_channel: &str,
                           draw: &bool) {
    let groups = self.data.groups();

    groups.iter().for_each( |group| {
      let channels = self.data.channels(group);

      // begin the search through files
      // let mut _i = 0;
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

        // _i += 1;

        // make the time increment for later usage!!
        let _increment = 1.0/time_output.len() as f64;
        let fs = time_output.len() as f64/7.;

        // best way found for the time domain
        // data in respect to the signal
        let time = linspace(0., time_output.len() as f64 / fs,
                            time_output.len()).map_into::<f64>()
          .collect_vec();

        // construct a basic 2nd order filter
        let fc_2000 = FilterMods{
          order: 2,
          cutoff_low: 2000.,
        };
        //filter the time output
        let filtered: Vec<_> = fc_2000.simple_lowpass(time_output.clone(),
                                                      fs).into_iter()
                                                         .collect();

        if witch_channel == "unknown"{
          println!("channel name {:?} \n\
                    samples = {:?} ~ duration = {:?} s ~ \
                    sampling frequency = {:?} Hz\n",
                   &channel.path,
                   time.len(),
                   time.last().copied(),
                   fs as f32);

        }else {let plot_title =
               if channel.path == witch_channel
               && self.state.contains('c') {
                 "Compressed air"}
               else{
                 "Inverter"};

               let mut fg =
               if *draw && (channel.path == witch_channel) {
                 Figure::new()}
               else {
                 println!(
                   "no maching channel in dataset to plot \n\
                    But we found one with the following name \n\
                    channel name {:?}",
                   channel.path);
                 continue;};

               //make the plot
               let the_title = format!("{} measurements", &plot_title);
               fg.set_title(&the_title)
               .axes2d()
               .set_x_label("Time (s)", &[])
               .set_y_label("Amplitute of signal", &[])
               .lines(&time, time_output,
                      &[Caption      (&format!(" Inv {:1?} Ws {:2?}",
                                               self.inv_state_exp,
                                               self.ws.to_string())),
                        Color        ("#a705b0"),
                        LineWidth    (0.5)])
               .lines(
                 &time,
                 &filtered.concat(),
                 &[Caption      ("2000 Hz lowpass"),
                   Color        ("black"),
                   LineWidth    (0.5)]);

               // check if user wants graph
               if *draw && (channel.path == witch_channel){

                 println!("channel name {:?} \n\
                           samples = {:?} ~ duration = {:?} s ~ \
                           sampling frequency = {:?} Hz\n",
                          &channel.path,
                          &time.len(),
                          time.last().copied(),
                          fs as f32);
                 // TODO attempt to save interactive semi-done!!
                 fg.show().unwrap();

               }

               else{
                 continue;}

                 // let f_type: &str = ".png";
                 // let _save_to_file = format!("{}{}",
                 //                             plot_title,
                 //                             f_type)
                   // .replace(' ', "_")
                   // .replace('/',"");
                 // unsafe{
                 // fg.save_to_png(_save_to_file, 800, 600).unwrap_unchecked();
                 // }

        };
      }
    });
  }
}
