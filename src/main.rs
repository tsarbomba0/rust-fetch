use crate::drive::drive::Drive;
use crate::cpu::cpu::CPU;
use crate::gpu::gpu::GPU;
mod drive;
mod cpu;
mod gpu;

fn main(){
    let drives: Vec<Drive> = Drive::load_disks();
    for d in drives {
        println!("Drive: {}", d.dev_name)
    }
    let cpu = CPU::get_cpu().unwrap();
    println!("a: {}", cpu.arch);

    let gpu = GPU::get_name().unwrap();
    println!("vendor: {}", gpu.vendor);
    println!("model: {}", gpu.model);
}
