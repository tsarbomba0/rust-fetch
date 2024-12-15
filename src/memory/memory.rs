use crate::drive::readfile::read_file;
use regex::Regex;
use crate::count_lines::line_count;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

pub struct Memory {
    pub free: u32,
    pub total: u32,
    pub available: u32,
    pub swap: bool,
    pub swap_partitions: Vec<String>,
    pub swap_total: u32,
    pub swap_free: u32,
    pub swap_used: u32,
}



impl Memory {
    pub fn fetch()->Result<Memory, std::io::Error>{
        let meminfo = match read_file("/proc/meminfo"){
            Ok(info) => info,
            Err(error) => return Err(error),
        };

        let mem_regex = Regex::new("(Mem.+):").unwrap();
        
        let mut swap_bool: bool = false;
        let mut swap_vec: Vec<String> = vec![];

        match line_count("/proc/swaps"){
            Ok(count) => {
                swap_bool = count != 1;
            },
            Err(_) => {
                println!("Couldn't open /proc/swaps, assuming there is no swap");
            }
        };

        let proc_swaps = match File::open("/proc/swaps"){
            Ok(f) => f,
            Err(error) => {
                println!("Couldn't open /proc/swaps, assuming there is no swap"); 
                panic!("{}", error)
            }
        };
        let mut proc_swaps_reader = BufReader::new(proc_swaps);
        let _ = proc_swaps_reader.skip_until(10);
        for line in proc_swaps_reader.lines(){ 
                swap_vec.push(line?.split_whitespace().next().unwrap().to_string());
        }

        
        let mut mem_free:   u32 = 0;
        let mut mem_total:  u32 = 0;
        let mut mem_avail:  u32 = 0;
        let mut swap_free:  u32 = 0;
        let mut swap_total: u32 = 0;

        if swap_bool {    
            let swap_regex = Regex::new("(Swap.+):").unwrap();
            for line in meminfo.split("\n") {
                if mem_regex.is_match(line) {
                    let split_line: Vec<&str> = line.split_whitespace().collect();
                    match split_line[0] {
                        "MemFree:" => {
                            mem_free = split_line[1].trim().parse::<u32>().unwrap() / 1024;
                        },
                        "MemTotal:" => {
                            mem_total = split_line[1].trim().parse::<u32>().unwrap() / 1024;
                        },
                        "MemAvailable:" => {
                            mem_avail = split_line[1].trim().parse::<u32>().unwrap() / 1024;
                        },
                        _ => ()
                    }
                } else if swap_regex.is_match(line) {
                    let split_line: Vec<&str> = line.split_whitespace().collect();
                    match split_line[0]{
                        "SwapFree:" => {
                            swap_free = split_line[1].trim().parse::<u32>().unwrap() / 1024;
                        },
                        "SwapTotal:" => {
                            swap_total = split_line[1].trim().parse::<u32>().unwrap() / 1024;
                        },
                        _ => (),
                    
                    }
                }  
            }

        } else {
            for line in meminfo.split("\n") {
                if mem_regex.is_match(line) {
                    let split_line: Vec<&str> = line.split_whitespace().collect();
                    match split_line[0] {
                        "MemFree:" => {
                            mem_free = split_line[1].trim().parse::<u32>().unwrap() / 1024;
                        },
                        "MemTotal:" => {
                            mem_total = split_line[1].trim().parse::<u32>().unwrap() / 1024;
                        },
                        "MemAvailable:" => {
                            mem_avail = split_line[1].trim().parse::<u32>().unwrap() / 1024;
                        },  
                        _ => ()
                    }
                }
            }
        };

        Ok(Self {
            free: mem_free,
            available: mem_avail,
            total: mem_total,
            swap: swap_bool,
            swap_partitions: swap_vec,
            swap_total: swap_total,
            swap_free: swap_free,
            swap_used: swap_total - swap_free,
        })
    }
}

