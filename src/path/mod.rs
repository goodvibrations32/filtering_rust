// use crate::Cow;
// use path_slash::CowExt as _;


struct WindSupply {
    type_of_experiment: String,
}
struct SysFolder {
    sys_path: String,
    data_parent: String,
    recording: String,
}
struct InnerExpFolders {
    ca: String,
    inv: String,
}


pub fn make_the_path<'a> (local:      &'a str,
                          exp_type:   &'a str,
                          inv:        &'a str,
                          ws:         &'a str) -> String {
    // wind supply method
    let experiment = WindSupply {
        type_of_experiment: String::from(exp_type),
        // compressed: String::from("compressed air/"),
        // wind_tunnel: String::from("inverter/"),
    };

    // system folders such as pc folder, parent of data
    // and recordings
    let sys_folder = SysFolder {
        sys_path: String::from(local),
        data_parent: String::from("data_folder/"),
        recording: String::from("measurements_12_05_22/new_record_prop_channel/"),
    };

    //rename to better stuff but for now i have only one method experiment
    let inverter_and_ws = InnerExpFolders {
        ca: String::from(format!("ca{inv}_{ws}.1/")),
        inv: String::from(format!("in{inv}_{ws}.1/")),
    };

    let local_place = sys_folder.sys_path;
    let parent = sys_folder.data_parent;
    let record = sys_folder.recording;
    let experiment_type = experiment.type_of_experiment;
    let experiment_state =
        if exp_type.find("c") == Some(0) {
            inverter_and_ws.ca
        }else {
            inverter_and_ws.inv
        };

    // println!("{:?}", experiment_state);
    const FILE_NAME: &str = "Data.tdms";
    let full_path: String = String::from(&format!(
        "{local_place}{parent}{record}{experiment_type}{experiment_state}{FILE_NAME}"));
    full_path

    // if cfg!(target_os = "linux"){
    //     let x = Cow::<String>::Borrowed(&full_path);
    //     x
    // }else {let windows = Cow::<String>::Borrowed(&full_path);
    //        windows
    //        }
}
