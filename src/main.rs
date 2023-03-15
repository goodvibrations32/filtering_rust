mod simple_gui;
pub mod path;
pub mod time_domain;
pub mod filters;
// use crate::time_domain::Signal;

use native_dialog::{
    MessageDialog, MessageType};


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let choises = MessageDialog::new().set_type(MessageType::Info)
                                      .set_text(
                                          "🧪 Do you know the channel name of the dataset? 🧪\n\
                                           If yes, remember ... \n\
                                           Have them in the project folder to \n\
                                           navigate easy there.\n\
                                           (The default script plots: \n\
                                           - Wind2 channel and the speed \n\
                                           that you choose in a legend\n\
                                           in the plot.) \n\
                                           If you dont know dont worry we cover that !! \n\
                                           choose `NO` and have some info in the terminal \n\
                                           about the file you will choose. 
                                           ").show_confirm().unwrap();
    match choises {
        true => simple_gui::gui_single_file("Wind2".to_string()),
        false => simple_gui::gui_single_file("unknown".to_string())
    }

    // time_domain::

    Ok( () )
}
