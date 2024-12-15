use std::fs::File;
use std::io::Read;
use std::io::BufRead;
use std::io::SeekFrom;
use std::fs::read_dir;
use std::io::BufReader;
use std::io::Seek;
use regex::Regex;
use crate::drive::readfile::read_file;



// Returns a tuple with Vendor and the line to start reading from in
// /usr/share/hwinfo/pci.ids.
fn vendor_to_pos(code: &str)->(&str, u64){
    match code.trim() {
        "0x8086" => ("Intel", 28347),
        "0x10de" => ("NVIDIA", 9891),
        "0x15ad" => ("VMware", 21984),
        "0xbeef" => ("VirtualBox", 37390),
        "0x1002" => ("AMD", 1208),
        _ => panic!("I don't know this one! {}", code)
    }
}

pub struct GPU {
     pub model: String,
     pub vendor: String,
}

impl GPU {
    pub fn get_name()->Result<GPU,std::io::Error>{
        let model_regex = Regex::new(r"\[(.+)\]").unwrap();
        // find all directories
        let dirs = match read_dir("/sys/bus/pci/devices/") {
            Ok(dirs) => dirs,
            Err(error) => {
                println!("Couldn't open /sys/bus/pci/devices, due to an error: {}", error);
                return Err(error)
            }        
        };
        
        // Path for the pci device
        let mut correct_pci_path = String::from("pholder"); 
        for dir in dirs {
            let path = dir.unwrap().path().into_os_string().into_string().unwrap();
            let result = match read_file(&format!("{}/class", path)){
                Err(error) => {
                    println!("Couldn't open {}/class", path);
                    return Err(error)
                },
                Ok(class) => class.trim().to_owned(),
            }; 
            if result == "0x030000" {
                correct_pci_path = path;
                break;
            }
        };
        if correct_pci_path == "pholder" {
            panic!("Couldn't find the GPU!");
        };

        let mut buf: [u8; 2] = [0,0];
        // Vendor 
        let vendor_code = match read_file(&format!("{}/vendor", correct_pci_path)){
            Ok(v) => v,
            Err(e) => {
                println!("Couldn't find the Vendor of this GPU due to an error: {}", e);
                "Unknown".to_string()
            }        
        };

        // "Model" code
        let mut model = File::open(format!("{}/config", correct_pci_path)).unwrap();
        match model.seek(SeekFrom::Start(2)){
            Err(error) => return Err(error),
            _ => (),
        }
        model.read_exact(&mut buf).unwrap();
        let model_string = format!("{0:x?}{1:x?}", buf[1], buf[0]); // format to a String
                                                              
        // Get vendor name and line to start reading from
        let (vendor, offset) = vendor_to_pos(&vendor_code);
        let file = match File::open("/usr/share/hwdata/pci.ids"){
            Ok(file) => file,
            Err(error) => {
                println!("Failed to fetch the pci id list, due to an error: {}", error);
                return Err(error)
            }        
        };
        let reader = BufReader::new(file);

        // skip lines till offset
        let mut lines = reader.lines();
        for _ in 0..(offset){
            lines.next();    
        };
                                                                
        // model name for later
        let mut model_name = String::from("unknown");
        // Loop through lines of the file
        for line in lines {        
            let actual_line = line?.clone();
            if actual_line.trim().starts_with(&model_string) {
                model_name = match model_regex.captures(&actual_line) {
                    Some(captured) => captured[1].to_string(),
                    None => "Unknown".to_string()
                };
                break;  
            }         
        };
        
        // Return GPU struct
        Ok(Self {
             model: model_name.to_owned(),
             vendor: vendor.to_owned(),                
        })
    }
}



                                
