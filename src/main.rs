mod process;

use crate::process::SysInfoSampler;
use crate::process::ProcessSampler;

fn main() {
    let mut sampler = SysInfoSampler::new();
    sampler.refresh();

    let processes = sampler.snapshot();
    for proc in processes {
        println!("Process name: {}", proc.name.to_string_lossy());
        println!("Process start time: {}", proc.start_time);
        println!("Process ID: {}", proc.pid);
    }  
}