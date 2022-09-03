mod sys_info;
mod visuals;
use linked_hash_map::LinkedHashMap;
use colored::{Colorize, control};
use sysinfo::{System, SystemExt};


use sys_info::{ find_os, find_computer_name, find_cpu_brand, find_memory };
use visuals::{ draw_line, draw_blocks };

fn main() {
    control::set_virtual_terminal(true).unwrap();
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut sys_info = LinkedHashMap::new();
    
    sys_info.insert("OS", find_os(&sys));
    sys_info.insert("Computer name", find_computer_name(&sys));
    sys_info.insert("CPU", find_cpu_brand(&sys));
    sys_info.insert("Memory", find_memory(&sys));

    println!("\n{}{}", format!("cor").truecolor(200, 137, 0),format!("fetch").truecolor(247, 47, 0));
    draw_line(None);
    for (key, value) in &sys_info {
    println!("{}{} {}", format!("{key}").bold().white(),format!(":").white(), format!("{value}").white());
    }
    println!();
    draw_blocks();
    draw_line(None);
    println!("{} {}", format!("@kanyewestfan420").bold().white(), format!("on GitHub").white());
}

