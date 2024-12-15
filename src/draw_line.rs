use rand::Rng;


pub fn drawline(lines: u32)->String{
    let mut output: String = String::new(); 
    for _ in 1..lines+1 {
        let random = rand::thread_rng().gen_range(0..101);
        output.push_str(match random {
            0..=  20 => "\x1b[36m█▓\x1b[0m",
            21..= 40 => "\x1b[36m▗░\x1b[0m",
            41..= 60 => "\x1b[45m▙▝\x1b[0m",
            61..= 80 => "\x1b[47m▚▏\x1b[0m",
            81..= 100 => "\x1b[44m▀▇\x1b[0m",
            _ => "ff",
        });
    }
    output
}


