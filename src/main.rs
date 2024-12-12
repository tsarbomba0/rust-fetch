use crate::drive::drive::Drive;
mod drive;

fn main(){
    let drives: Vec<Drive> = Drive::load_disks();
    for d in drives {
        println!("Drive: {}", d.dev_name)
    }
}
