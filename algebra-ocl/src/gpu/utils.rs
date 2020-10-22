use log::{info, warn};
use rust_gpu_tools::*;
use std::collections::HashMap;
use std::env;

/// Divide anything into limbs of type `E`
pub fn limbs_of<T, E: Clone>(value: T) -> Vec<E> {
    unsafe {
        std::slice::from_raw_parts(
            &value as *const T as *const E,
            std::mem::size_of::<T>() / std::mem::size_of::<E>(),
        )
        .to_vec()
    }
}

lazy_static::lazy_static! {
    static ref CORE_COUNTS: HashMap<String, usize> = {
        let mut core_counts : HashMap<String, usize> = vec![
            // Pocl
            ("pthread-Intel(R) Core(TM) i7-4870HQ CPU @ 2.50GHz".to_string(), 4),

            // Apple
            ("Iris Pro".to_string(), 40),

            // AMD
            ("gfx1010".to_string(), 2560),

            // NVIDIA
            ("Quadro RTX 6000".to_string(), 4608),

            ("TITAN RTX".to_string(), 4608),

            ("Tesla V100".to_string(), 5120),
            ("Tesla P100".to_string(), 3584),
            ("Tesla T4".to_string(), 2560),
            ("Quadro M5000".to_string(), 2048),

            ("GeForce RTX 2080 Ti".to_string(), 4352),
            ("GeForce RTX 2080 SUPER".to_string(), 3072),
            ("GeForce RTX 2080".to_string(), 2944),
            ("GeForce RTX 2070 SUPER".to_string(), 2560),

            ("GeForce GTX 1080 Ti".to_string(), 3584),
            ("GeForce GTX 1080".to_string(), 2560),
            ("GeForce GTX 2060".to_string(), 1920),
            ("GeForce GTX 1660 Ti".to_string(), 1536),
            ("GeForce GTX 1060".to_string(), 1280),
            ("GeForce GTX 1050".to_string(), 640),
            ("GeForce GTX 1650 SUPER".to_string(), 1280),
            ("GeForce GTX 1650".to_string(), 896),
        ].into_iter().collect();

        match env::var("BELLMAN_CUSTOM_GPU").and_then(|var| {
            for card in var.split(",") {
                let splitted = card.split(":").collect::<Vec<_>>();
                if splitted.len() != 2 { panic!("Invalid BELLMAN_CUSTOM_GPU!"); }
                let name = splitted[0].trim().to_string();
                let cores : usize = splitted[1].trim().parse().expect("Invalid BELLMAN_CUSTOM_GPU!");
                info!("Adding \"{}\" to GPU list with {} CUDA cores.", name, cores);
                core_counts.insert(name, cores);
            }
            Ok(())
        }) { Err(_) => { }, Ok(_) => { } }

        core_counts
    };
}

const DEFAULT_CORE_COUNT: usize = 2560;
pub fn get_core_count(d: &opencl::Device) -> usize {
    let name = d.name();
    match CORE_COUNTS.get(&name[..]) {
        Some(&cores) => cores,
        None => {
            warn!(
                "Number of CUDA cores for your device ({}) is unknown! Best performance is \
                 only achieved when the number of CUDA cores is known! You can find the \
                 instructions on how to support custom GPUs here: \
                 https://lotu.sh/en+hardware-mining",
                name
            );
            DEFAULT_CORE_COUNT
        }
    }
}

pub fn dump_device_list() {
    info!("Device list dumping");
    for d in opencl::Device::all().unwrap() {
        info!("Device: {:?}", d);
    }
}

#[test]
pub fn test_list_devices() {
    let _ = env_logger::try_init();
    dump_device_list();
}