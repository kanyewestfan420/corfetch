use sysinfo::{System, SystemExt, CpuExt};
use big_bytes::BigByte;

pub fn find_os(sys: &System) -> String {
    sys.name().unwrap().to_string() + " " + &sys.os_version().unwrap().to_string()
}

pub fn find_computer_name(sys: &System) -> String{
    sys.host_name().unwrap().to_string()
}

pub fn find_cpu_brand(sys: &System) -> String {
    sys.cpus()[0].brand().to_string().trim_end().to_string()
}

pub fn find_memory(sys: &System) -> String {
    fn memory_to_str(memory: u64) -> String {
        memory.big_byte(2).to_string()
    }
    
    let used_ram = sys.used_memory();
    let total_ram = sys.total_memory();
    memory_to_str(used_ram) + " " + "/" + " " + &memory_to_str(total_ram)
    // fix remove GiB at the end of string from big_byte function
}