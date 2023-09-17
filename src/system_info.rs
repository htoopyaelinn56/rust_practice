use std::time::Duration;

use sysinfo::{CpuExt, System, SystemExt};

pub fn get_sys_info() {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();
    let cpu = sys.cpus().first().unwrap().brand();

    // Display system information:
    println!("System CPU:              {:?}", cpu);
    println!("System name:             {:?}", sys.name().unwrap());
    println!(
        "System kernel version:   {:?}",
        sys.kernel_version().unwrap()
    );
    println!("System OS version:       {:?}", sys.os_version().unwrap());
    println!("System host name:        {:?}", sys.host_name().unwrap());
    println!("System host name:        {:?}", sys.total_memory());
}

pub fn stream_cpu_usage() {
    let mut sys = System::new();

    loop {
        sys.refresh_cpu(); // Refreshing CPU information.
        let mut cpu_usage_list: Vec<f32> = vec![];

        for (i, cpu) in sys.cpus().iter().enumerate() {
            println!("cpu {} : {}% ", i, cpu.cpu_usage());
            cpu_usage_list.push(cpu.cpu_usage());
        }
        // Sleeping to let time for the system to run for long
        // enough to have useful information.
        std::thread::sleep(Duration::new(4, 0));
    }
}
