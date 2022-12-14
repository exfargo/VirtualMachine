use std::path::Display;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use sdl2::keyboard::Keycode::Mute;

use crate::lib::bus::bus::Bus;
use crate::lib::cpu::cpu::CPU;
use crate::lib::gpu::gpu::GPU;
use crate::lib::gpu::monitor::Monitor;
use crate::lib::mem::ram::RAM;

pub mod lib;

fn main() {
    // 536870912 * 8 => 4 GB => 4096 MB
    // address range => 0x0000'0000 <-> 0x1FFF'FFFF
    let mut ram = RAM::new(536_870_912);
    let mut bus = Arc::new(Mutex::new(Bus::new()));
    let mut bref1 = Arc::clone(&bus);
    let mut bref2 = Arc::clone(&bus);

    let mut m0 = Arc::new(Mutex::new(Monitor::new(20, 20)));
    let mut m1 = Arc::clone(&m0);
    let mut m2 = Arc::clone(&m0);
    let mut cpu = CPU::new();

    let mut gpu = GPU::new("vGPU - GACUM (Graphical Accelerated Compute Unit Magic)", "vgpu-acum-0000-0000");

    thread::spawn(move || {
        gpu.launch(&bref2, &mut [&mut m1])
    });

    thread::spawn(move || {
        m2.lock().unwrap().launch();
    });

    thread::sleep(Duration::new(0, 500_000));

    let cpu_thread = thread::spawn(move || {
        cpu.launch(&mut ram, &bref1)
    });


    while !cpu_thread.is_finished() {}
}