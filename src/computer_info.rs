use std::collections::HashMap;
use sys_info;
use sys_info::MemInfo;

const KILOBYTES_IN_MEGABYTE: i16 = 1024;
const SECONDS_IN_MINUTE: i16 = 60;
const SECONDS_IN_HOUR: i32 = 216000;


pub trait Printable {
    fn print(&self);
}

pub struct ComputerInfo {
    pub cpu_num: u32,
    pub host_name: String,
    pub os_type: String,
    pub os_release: String,
    pub mem_info: MemInfo,
    pub os_logos: HashMap<String, String>,
    pub cpu_speed: u64,
    pub proc_total: u64,
}

impl Printable for ComputerInfo {
    fn print(&self) {
        let mem_in_mbytes = self.mem_info.total / (KILOBYTES_IN_MEGABYTE as u64);
        let os_logo = self.os_logos.get(&self.os_type);

        let os_logo = match os_logo {
            Some(logo) => logo,
            None => "",
        };

        println!("{}", self.host_name);
        println!("{} {} {}", os_logo, self.os_type, self.os_release);
        println!("Number of cpus:  {}", self.cpu_num);
        println!("RAM total:  {}Mb", mem_in_mbytes);
        println!("CPU Speed: {} MHz", self.cpu_speed);
        println!("Running processes: {}", self.proc_total);
    }
}

impl Default for ComputerInfo {
    fn default() -> Self {
        let cpu_num = sys_info::cpu_num().unwrap();
        let host_name = sys_info::hostname().unwrap();
        let os_type = sys_info::os_type().unwrap();

        let os_release = sys_info::os_release().unwrap();
        let mem_info = sys_info::mem_info().unwrap();

        let cpu_speed = sys_info::cpu_speed().unwrap();
        let proc_total = sys_info::proc_total().unwrap();

        let mut os_logos = HashMap::new();
        os_logos.insert(String::from("Darwin"), String::from(""));

        return ComputerInfo {
            cpu_num: cpu_num,
            host_name: host_name,
            os_type: os_type,
            os_release: os_release,
            mem_info: mem_info,
            os_logos: os_logos,
            cpu_speed: cpu_speed,
            proc_total: proc_total,
        };
    }
}
