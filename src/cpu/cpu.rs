use crate::drive::readfile::read_file;

// CPU struct
pub struct CPU {
    pub max_freq: u32,
    pub min_freq: u32,
    pub brand: String,
    pub byte_order: String,
    pub threads: u8,
    pub cores: u8,
    pub arch: String,
}

impl CPU {   
    pub fn get_byte_order()->String{
        if cfg!(target_endian = "little"){
                "little-endian".to_string() 
        } else {
                "big-endian".to_string()
        }
    }
    pub fn get_cpu()->Result<Self, std::io::Error>{
        let mut cpu_brand: String = String::from("unknown");
        
        let read = read_file("/proc/cpuinfo");
        let proc_cpuinfo = match read
        {
            Ok(s) => s,
            Err(error) => return Err(error)
        };

        

        // CPU brand
        for line in proc_cpuinfo.split_whitespace(){
            if line.starts_with("model name") {
                cpu_brand = line.replace("model name", "").to_string()
            }
        }
        
        // Maximum frequency
        let max_freq: u32 = match read_file("/sys/devices/system/cpu/cpu0/cpufreq/scaling_max_freq"){
            Ok(res) => {
                println!("{}", res);
                match res.trim().parse::<u32>() {
                    Ok(n) => n/1000,
                    Err(error) => {
                        println!("This shouldn't happen. {}", error);
                        panic!("{}", error)
                    }
                }
            },
            Err(error) => {
                return Err(error)
             }
        };
        
        // Minimum frequency
        let min_freq: u32 = match read_file("/sys/devices/system/cpu/cpu0/cpufreq/scaling_min_freq"){
            Ok(res) => {
                    res.trim().parse::<u32>().unwrap() / 1000
                },
                Err(error) => {
                    return Err(error)
                }
        };
        
        // Number of threads
        let thread_num: u8 = match read_file("/sys/devices/system/cpu/possible") {
              Ok(res) => {
                let pos = res.split("-").collect::<Vec<&str>>()[1];
                let result = pos.trim().parse::<u8>();
                match result {
                    Ok(i) => i+1,
                    Err(error) => {
                       panic!("How? {}", error)                       
                    }
                }
            },
            Err(error) => {
                 return Err(error)                                      
            }
        };
        
        
        

        Ok(Self {
            max_freq: max_freq.to_owned(),
            min_freq: min_freq.to_owned(),
            byte_order: CPU::get_byte_order(),
            threads: thread_num.to_owned(),
            cores: (thread_num/2).to_owned(),
            arch: std::env::consts::ARCH.to_owned(),
            brand: cpu_brand.to_owned(),
        })
    }
}


