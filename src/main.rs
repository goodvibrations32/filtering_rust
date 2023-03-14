mod simple_gui;
pub mod path;
pub mod time_domain;
pub mod filters;
// use crate::time_domain::Signal;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    simple_gui::gui_single_file("unknown".to_string());
    // time_domain::

    Ok( () )
}
