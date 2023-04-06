use itertools::Itertools;
extern crate tdms;
use native_dialog::{MessageDialog, MessageType};
use tdms::{data_type::TdmsDataType, TDMSFile};

extern crate itertools_num;
use itertools_num::linspace;


use gnuplot::{Figure, Caption, Color,
              AxesCommon,LineWidth};
use welch_sde::{SpectralDensity, Build};

pub(crate) use crate::filtering::FilterMods;

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

trait RawData{
    fn gime_data(self) -> Vec<f64>;
}

impl RawData for tdms::TDMSFile<'_> {
    fn gime_data(self) -> std::vec::Vec<f64> {

        let mut out: Vec<f64> = vec![];
        let groups = self.groups();

        groups.iter().for_each( |group| {
            let channels = self.channels(group);

            // begin the search through files
            // let mut _i = 0;
            channels.into_iter().for_each(|(_, channel)| {
                let full_group = match channel.data_type {
                    TdmsDataType::DoubleFloat(_) => {
                        self.channel_data_double_float(channel)},
                    _ => {
                        panic!("{}", "channel for data type unimplemented")}
                };

                let full_group_iterator = match full_group {
                    Ok(i) => i,
                    Err(e) => {
                        panic!("{:?}", e)
                    }
                };
                // store the signal somewhere
                out = full_group_iterator
                    .map_into::<f64>()
                    .collect_vec();
            });
        });out
    }
}

impl Signal <'_> {
    pub fn spectra(&self, title: String, raw_data: Vec<f64>) {
        let raw_signal: Vec<f64> = raw_data;

        let fs = raw_signal.len() as f64 / 7.;
        let welch: SpectralDensity<f64> =
            SpectralDensity::<f64>::builder(&raw_signal, fs).build();

        let low = FilterMods { order: 2,
                                  cutoff_low: 2_000.,
                                  cutoff_high: 0. };
        let filtrd = low.simple_lowpass(raw_signal.clone(), fs)
                           .into_iter()
                           .collect_vec()
                           .concat();

        let band_stop_filter = FilterMods { order: 2,
                                  cutoff_low: 2_000.,
                                  cutoff_high: 15_000. };
        let band = band_stop_filter.band_stop(raw_signal.clone(), fs)
                           .into_iter()
                           .collect_vec()
                           .concat();
        // make the power spectrum for low pass filtered and raw data
        let welch_filtrd: SpectralDensity<f64> =
            SpectralDensity::<f64>::builder(&filtrd, fs).build();

        let sdens = welch.periodogram();
        let sdens_filt = welch_filtrd.periodogram();

        // calculate welch for band stop filter
        let welch_band: SpectralDensity<f64> =
            SpectralDensity::<f64>::builder(&band, fs).build();
        let sdens_filt_band = welch_band.periodogram();

        let mut fig = Figure::new();
        fig.set_title(&format!("Welch Spectral Density of {:?} measurements",
                               title))
           .set_multiplot_layout(1, 2)
           .axes2d()
           // .set_x_range(gnuplot::Fix(1e1), gnuplot::Fix(1e6))
           .set_y_range(gnuplot::Fix(1e-15), gnuplot::Fix(1e-1))
           .set_x_label("Frequencies [Hz]", &[])
           .set_x_log(Some(10.))
           .set_y_label("Spectral density [s^2/Hz]", &[])
           .set_y_log(Some(10.))
           .lines(
               sdens.frequency()
                    .into_iter(),
               &(*sdens),
               &[Caption (&format!(" Inv {:1?} Ws {:2?}",
                                   self.inv_state_exp,
                                   self.ws.to_string())),
                 Color("blue"),
                 LineWidth(0.9)] )
           .lines(
               sdens_filt.frequency()
                         .into_iter(),
               &(*sdens_filt),
               &[Caption        ("2000 Hz lowpass"),
                 Color          ("magenta"),
                 LineWidth      (0.9)] );
        fig.axes2d()
           .set_y_range(gnuplot::Fix(1e-15), gnuplot::Fix(1e-1))
           .set_x_label("Frequencies [Hz]", &[])
           .set_x_log(Some(10.))
           .set_y_label("Spectral density [s^2/Hz]", &[])
           .set_y_log(Some(10.))
           .lines(
               sdens_filt_band.frequency()
                              .into_iter(),
               &(*sdens_filt),
               &[Caption        ("2.000-15.000 bandstop"),
                 Color          ("black"),
                 LineWidth      (0.9)] );

        fig.show().unwrap();
    }
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
    pub fn plot_in_time_dom (self,
                             witch_channel: &str,
                             draw: &bool) {
        // store the signal somewhere
        let out: Vec<f64> = RawData::gime_data(self.data.clone());
        // make the time increment for later usage!!
        // let _increment = 1.0/out.len() as f64;
        let fs = out.len() as f64/7.;

        // // best way found for the time domain
        // // data in respect to the signal
        let time = linspace(0., out.len() as f64 / fs,
                            out.len()).map_into::<f64>()
            .collect_vec();

        // construct a basic 2nd order filter
        let fc_2000 = FilterMods{
            order: 2,
            cutoff_low: 2000.,
            cutoff_high: 10_000.,
        };
        // filter the time output
        let filtered = fc_2000.simple_lowpass(out.clone(),
                                              fs).into_iter()
                                                 .collect_vec();

        // let filt_stop = fc_2000.band_stop(out.clone(),
        //                                   fs).into_iter()
        //                                      .collect_vec();

        let groups = &self.data.groups();

        groups.iter().for_each( |group| {
            let channels = self.data.channels(group);

            // begin the search through files
            // let mut _i = 0;
            for (_, channel) in channels{

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
                       .lines(&time, &out,
                              &[Caption      (&format!(" Inv {:1?} Ws {:2?}",
                                                       self.inv_state_exp,
                                                       self.ws.to_string())),
                                Color        ("blue"),
                                LineWidth    (0.5)])
                       .lines(
                           &time,
                           &filtered.concat(),
                           &[Caption      ("2000 Hz lowpass"),
                             Color        ("red"),
                             LineWidth    (0.9)]);
                       // .lines(
                       //     &time,
                       //     &filt_stop.concat(),
                       //     &[Caption      ("2000 Hz lowpass"),
                       //       Color        ("black"),
                       //       LineWidth    (0.5)]);

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

                           let yes = MessageDialog::new().set_text("Would you like to \
                                                                    plot also the Spectral \
                                                                    Density of the \
                                                                    signal?")
                                               .set_type(MessageType::Info)
                                               .show_confirm();
                           match yes.unwrap() {
                               true => {
                                   self.spectra(plot_title.to_string(),
                                                out.to_vec(),
                                                )
                               }
                               false => {
                                   continue;
                               }
                           }
                       }
                       else{
                           continue;}
        }
      }
    });
  }
}
