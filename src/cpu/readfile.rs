use std::fs::File;
use std::io::Read;

pub mod readfile;
pub fn read_file(filename: &str)->std::io::Result<String>{
    let openfile = File::open(filename);
    let mut file = match openfile {
        Ok(file) => file,
        Err(error) => panic!("{error:?}")
    };

    let mut content = String::new();
    
    let r = file.read_to_string(&mut content);
    match r {
        Ok(_) => Ok(content),
        Err(error) => Err(error),
    }
}
