use crate::cpu::cpu::CPU;
use crate::drive::drive::Drive;
use crate::gpu::gpu::GPU;
use crate::memory::memory::Memory;
use std::sync::{Arc, Mutex};
use std::thread;

mod count_lines;
mod cpu;
mod draw_line;
mod drive;
mod gpu;
mod memory;

fn construct() -> String {
    let message = Arc::new(Mutex::new(String::new()));
    let mut threads = vec![];

    {
        let data = Arc::clone(&message);
        // Drives.
        threads.push(thread::spawn(move || {
            let drives = Drive::load_disks();
            let mut msg = data.lock().unwrap();
            msg.push_str("        ⇒ Drives!\n");
            for drive in drives {
                msg.push_str(&format!(
                    "     {0}: {1:.2}/{2:.2} GB ({3})\n",
                    drive.mount_point, drive.space_free, drive.space_total, drive.dev_name
                ))
            }
            msg.push('\n');
        }));
    };

    // CPU
    {
        let data = Arc::clone(&message);
        threads.push(thread::spawn(move || {
            let cpu = CPU::get_cpu().unwrap();
            let mut msg = data.lock().unwrap();
            msg.push_str("        ⇒ CPU!\n");
            msg.push_str(&format!("     {}\n\n", cpu.brand));
        }));
    };

    // GPU.
    {
        let data = Arc::clone(&message);
        threads.push(thread::spawn(move || {
            let gpu = GPU::get_name().unwrap();
            let mut msg = data.lock().unwrap();
            msg.push_str("        ⇒ GPU!\n");
            msg.push_str(&format!("     {0} {1}\n\n", gpu.vendor, gpu.model));
        }));
    };

    {
        let data = Arc::clone(&message);
        threads.push(thread::spawn(move || {
            let mem = Memory::fetch().unwrap();
            let mut msg = data.lock().unwrap();
            msg.push_str("        ⇒ Memory!\n");
            msg.push_str(&format!(
                "     Total: {0} MB\n     Available: {1} MB\n",
                mem.total, mem.available
            ));
            if mem.swap {
                msg.push_str(&format!(
                    "\n        ⇒ Swap!\n     Total: {0} MB\n     Free: {1} MB\n     Used: {2} MB\n",
                    mem.swap_total, mem.swap_free, mem.swap_used
                ));
                msg.push_str("       ⇒ Swap partitions!\n");
                for a in mem.swap_partitions {
                    msg.push_str(&format!("    {0}\n", a))
                }
            }
        }))
    };

    for t in threads {
        let _ = t.join();
    }

    let a = message.lock().unwrap().clone();
    a.to_string()
}
fn main() {
    let fetch = construct();
    let line_count = fetch.lines().collect::<Vec<&str>>().len();
    for line in fetch.split("\n") {
        println!("{0}  {1}", draw_line::drawline(line_count as u32), line)
    }
}
