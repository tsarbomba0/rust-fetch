use crate::drive::drive::Drive;
use crate::cpu::cpu::CPU;
use crate::gpu::gpu::GPU;
use crate::count_lines::line_count;
use crate::memory::memory::Memory;

mod memory;
mod drive;
mod cpu;
mod gpu;
mod count_lines;

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


    let memory = Memory::fetch().unwrap();
    println!("swap part: {:?}", memory.swap_partitions);
}
