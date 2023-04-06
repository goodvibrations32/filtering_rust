/// This is for filtering and experimenting with implementations
///
use iir_filters::filter_design::FilterType;
use iir_filters::filter_design::butter;
use iir_filters::sos::zpk2sos;
use iir_filters::filter::{DirectForm2Transposed, Filter};

pub struct FilterMods{
    pub order: u32,
    pub cutoff_low: f64,
    pub cutoff_high: f64
}
impl FilterMods {
    pub fn simple_lowpass(self,
                          sig: Vec<f64>,
                          sigfs: f64)
                          -> Result<Vec<f64>, Box<dyn std::error::Error>> {

        let zpk = butter(self.order,
                         FilterType::LowPass(self.cutoff_low), sigfs)?;
        let sos = zpk2sos(&zpk, None)?;
        let mut dft2mode = DirectForm2Transposed::new(&sos);
        let mut out: Vec<f64> = Vec::new();

        sig.iter().for_each(|x| {
            out.push(dft2mode.filter(*x));
        });
        Ok(out)
    }
}
impl FilterMods{
    pub fn band_stop(self,
                     sign: Vec<f64>,
                     sigfs: f64) -> Result<Vec<f64>, Box<dyn std::error::Error>>{
        let coeff = butter(self.order,
                           FilterType::BandStop(self.cutoff_low,
                                                self.cutoff_high), sigfs)?;
        let sos = zpk2sos(&coeff, None)?;
        let mut trans = DirectForm2Transposed::new(&sos);
        let mut outpt: Vec<f64> = Vec::new();
        sign.iter().for_each(|x| {
            outpt.push(trans.filter(*x));
        });
        Ok(outpt)
    }
}
impl FilterMods {
    pub fn _simple_highpass (self,
                             _sig: Vec<f64>,
                             _sigfs: f64)
                             -> Result<Vec<f64>, Box<dyn std::error::Error>>{
        todo!("Have to implement a simple high pass butterworth for general use")
    }
}
#[test]
fn lets_be_sure_for_filters(){
    let check = FilterMods{
        order:1,
        cutoff_low:2.
    };
    let x = vec![1., 2., 3.];
    match check.simple_lowpass(x, 8.).is_ok(){
        true => println!("The filter works as intended"),
        false => panic!()
    }
}
