use sys_info;
use sys_info::MemInfo;

const KILOBYTES_IN_MEGABYTE: i16 = 1024;

pub struct ComputerInfo {
    pub cpu_num: u32,
    pub host_name: String,
    pub os_type: String,
    pub os_release: String,
    pub mem_info: MemInfo,
}

impl ComputerInfo {
    pub fn print(&self) {
        let mem_in_mbytes = self.mem_info.total / (KILOBYTES_IN_MEGABYTE as u64);

        println!("{}", self.host_name);
        println!("{} {}", self.os_type, self.os_release);
        println!("Number of cpus:  {}", self.cpu_num);
        println!("RAM total:  {}Mb", mem_in_mbytes);
    }
}

impl Default for ComputerInfo {
    fn default() -> Self {
        let cpu_num = sys_info::cpu_num().unwrap();
        let host_name = sys_info::hostname().unwrap();
        let os_type = sys_info::os_type().unwrap();
        let os_release = sys_info::os_release().unwrap();
        let mem_info = sys_info::mem_info().unwrap();

        return ComputerInfo {
            cpu_num: cpu_num,
            host_name: host_name,
            os_type: os_type,
            os_release: os_release,
            mem_info: mem_info,
        };
    }
}
