use crate::cpu::readfile:read_file;
use std::thread;
mod readfile;

// CPU struct
struct CPU {
    max_freq: u32,
    min_freq: u32,
    brand: String,
    byte_order: String,
    threads: u8,
    cores: u8,
    arch: String,
}

impl CPU {    
    pub fn get_cpu(){
        let mut cpuinfo: String;
        let mut max_freq: u32;
        let mut min_freq: u32;
        let mut thread_num: u8;
        let mut threads = vec![];
        
        // CPU info
        threads.push(thread::spawn(move || {
            let result = read_file("/proc/cpuinfo");
            cpuinfo = match result {
                Ok(res) => res,
                Err(error) => {
                    println!("Couldn't fetch CPU info due to a error: {}", error);
                    String::new("")
                }
            }
        }));

        // Maximum frequency
        threads.push(thread::spawn(move || {
            let result = read_file("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq");
            max_freq = match result {
                Ok(res) => {
                    res.parse().unwrap() / 1000
                },
                Err(error) => {
                    println!("Couldn't fetch max CPU frequency, due to an error: {}", error);
                 0
                }
            }
        }));
        
        // Minimum frequency
        threads.push(thread::spawn(move || {
            let result = read_file("/sys/devices/system/cpu/cpu0/cpufreq/scaling_min_freq");
            min_freq = match result {
            Ok(res) => {
                    res.parse().unwrap() / 1000
                },
                Err(error) => {
                    println!("Couldn't fetch min CPU frequency, due to an error: {}", error);
                    0
                }
            }
        }));
        
        // Number of threads
        threads.push(thread::spawn(move || {
            let result = read_file("/sys/devices/system/cpu/possible");
            thread_num = match result {
                Ok(res) => {
                    let pos = res.split("-").unwrap();
                    pos.parse::<u8>()+1
                },
                Err(error) => {
                    println!("Couldn't obtain the amount of threads due to an error: {}", error);
                    0
                }
            };
        }));
        
        
        for thread in threads {
            let _ = thread.join(); 
        };


        Self {
            max_freq: max_freq.to_owned(),
            min_freq: min_freq.to_owned(),
            byte_order: target_endian,
            threads: thread_num,
            cores: thread_num/2,
        }
    }
}


