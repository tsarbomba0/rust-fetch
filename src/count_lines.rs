use std::io::BufReader;
use std::fs::File;
use std::io::BufRead;


pub fn line_count(name: &str)->Result<u32, std::io::Error>{
    let f = match File::open(name){
        Ok(file) => file,
        Err(error) => return Err(error)
    };
    let mut buffer = BufReader::new(f);
    let mut line: Vec<u8> = vec![];
    let mut linecount: u32 = 0;

    while match buffer.read_until(10, &mut line) {
        Ok(n) => n != 0,
        Err(error) => return Err(error),
        _ => false,
    } {
        linecount += 1;
    };
    Ok(linecount)
}

