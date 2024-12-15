use crate::drive::fsinfo::fs_info;
use crate::drive::mounts::mounts;

use std::fmt;

pub struct Drive {
    pub dev_name: String,
    pub mount_point: String,
    pub space_total: f64,
    pub space_free: f64,
}

impl fmt::Debug for Drive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>)->fmt::Result{
        f.debug_struct("Drive")
            .field("dev_name: ", &self.dev_name)
            .field("mount_point: ", &self.mount_point)
            .field("space_total: ", &self.space_total)
            .field("space_free: ", &self.space_free)
            .finish()
    }
}


impl Drive {
    pub fn new(mnt: &str, dev: &str)->Drive{
        let result = fs_info(mnt);
        let disk_info = match result {
            Ok(info) => info,
            Err(e) => panic!("Error: {}", e),
        };
        
        Self {
            dev_name: dev.to_owned(),
            mount_point: mnt.to_owned(),
            space_total: (disk_info.f_blocks * disk_info.f_bsize as u64) as f64 / 1024.0_f64.powf(3.0) ,
            space_free:  (disk_info.f_bfree * disk_info.f_bsize as u64) as f64 / 1024.0_f64.powf(3.0) ,
        }
    }
    
    
    pub fn load_disks()->Vec<Drive>{
        let (avail, mounts) = mounts();
        let mut drives: Vec<Drive> = vec![];

        for (index, mnt) in avail.iter().enumerate() {
            drives.push(Drive::new(mounts[index*3+1].as_str(), mnt));
        }
        drives
    }
}


