use crate::drive::readfile::read_file;

// Obtains the mounted devices from /etc/mstab
// or alternatively /proc/mounts
pub fn mounts() -> (Vec<String>, Vec<String>){
    let r = read_file("/etc/mtab");
    let mounts = match r {
        Ok(s) => s,
        Err(_) => {
            let r1 = read_file("/proc/mounts");
            let result = match r1 {
                Ok(s1) => s1,
                Err(error) => panic!("Tried /proc/mounts and /etc/mtab, error: {}!", error),
            };
            result
        },
    };
    

    let mut output: Vec<String> = vec![];
    let mut avail: Vec<String> = vec![];

    for mount in mounts.split("\n") {
        if mount.starts_with('/') {
            let v: Vec<&str> = mount.split(" ").collect();
            avail.push(v[0].to_string());
            output.push(v[0].to_string());
            output.push(v[1].to_string());
            output.push(v[2].to_string());
        }
    }
    (avail, output)
}


