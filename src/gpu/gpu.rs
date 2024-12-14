use std::fs::File;
use std::io::Read;
use std::io::SeekFrom;
use std::fs::read_dir;
use std::io::BufReader;
use std::io::Seek;
use std::io::{self, prelude::*};

use crate::drive::readfile::read_file;



// Returns a tuple with Vendor and the line to start reading from in
// /usr/share/hwinfo/pci.ids.
fn vendor_to_pos(code: &str)->(&str, u64){
    match code {
        "8086" => ("Intel", 28347),
        "10de" => ("Nvidia", 9891),
        "15ad" => ("VMware", 21984),
        "beef" => ("VirtualBox", 37390),
        "1002" => ("AMD", 1208),
        _ => panic!("I don't know this one! {}", code)
    }
}

pub struct GPU {
     model: String,
     vendor: String,
}

impl GPU {
    pub fn get_name()->Result<GPU,std::io::Error>{
        // find correct id
        let gpu = match read_dir("/sys/bus/pci/devices/") {
            Ok(dirs) => {
                for dir in dirs {
                    let path = dir.unwrap().path();
                    match read_file(&format!("{:?}/class", path)){
                        Err(error) => return Err(error),
                        Ok(class) => {
                            if class == "0x030000" {
                                let mut buf: [u8; 2];
                                // Vendor 
                                let vendor_code = match read_file(&format!("{:?}/vendor", path)) {
                                    Ok(v) => v,
                                    Err(e) => return Err(e)
                                };

                                // "Model" code
                                let mut model = File::open(format!("{:?}/config", path)).unwrap();
                                model.seek(SeekFrom::Start(2));
                                model.read_exact(&mut buf).unwrap();
                                let model_string = format!("{0}{1}", buf[0].to_string(), buf[1].to_string()); // format to a str
                                // model name for later
                                let mut model_name: String;

                                // Get vendor name and line to start reading from
                                let (vendor, offset) = vendor_to_pos(&vendor_code); 
                                let file = match File::open("/usr/share/hwdata/pci.ids"){
                                    Ok(file) => file,
                                    Err(error) => return Err(error)
                                };

                                let reader = BufReader::new(file);

                                // skip lines till offset
                                let mut lines = reader.lines();
                                for _ in 0..(offset){
                                    lines.next();
                                };
                                
                                // Loop through lines of the file
                                for line in lines {
                                    if line?.starts_with(&model_string) {
                                        model_name = line?;
                                        break;
                                    } else {
                                        lines.next();
                                    }       
                                }

                                // Return GPU
                                Self {
                                        vendor: vendor.to_owned(),
                                        model: model_name.to_owned(),       
                                }

                                } else {
                                    // Panic if no gpu is found
                                  panic!("No GPU found!")
                                };
                        
                        } 
                    }
                }
            },
            Err(error) => return Err(error)
        
        };
        Ok(gpu)
    }
}
