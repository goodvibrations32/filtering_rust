#[doc = r"Selecting a file and extracting the data"]
pub mod simple_gui;
#[doc = r"Here all the plotting happens"]
pub mod time_domain;
mod filtering;

use native_dialog::{
    MessageDialog, MessageType};


/// # Main function
/// Here a simple first message is given to explain
/// to the user what will happen. The idea is to
/// have a first layer of information about what
/// the user needs to do. The message appearing in the
/// screen could look different  but the
/// text should be identical in all OS's.
/// ## Choices
/// - Yes :
///     The channel with name "Wind2" will be
///     plotted if it contains any data.
/// - No :
///     There will be displayied some information
///     about any found channel that could be parsed
///     and in the chosen `.tdms` file.

fn main() -> Result<(), Box<dyn std::error::Error>>{

    let choises = MessageDialog::new().set_type(MessageType::Info)
                                      .set_text(
                                          "🧪 Do you know the channel name of the dataset? 🧪 \n\
                                           If yes, remember ... \n\
                                           Have them in the latest naming \n\
                                           conventon folders to \
                                           plot the recording.\n \n\
                                           By default 'Wind2' channel will be plotted!! \n\
                                           If you dont know the channel name you want to see we cover that !! \n \n\
                                           Choose `NO` and have some info in the terminal \n\
                                           about the file you will choose. 
                                           ").show_confirm().unwrap();
    match choises {
        true => simple_gui::gui_single_file("Wind2".to_string(), true),
        false => simple_gui::gui_single_file("unknown".to_string(), false)
    }
    Ok( () )
}
