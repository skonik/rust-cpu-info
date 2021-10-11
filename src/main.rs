use sys_info;
use sys_info::MemInfo;

const KILOBYTES_IN_MEGABYTE: i16 = 1024;

struct ComputerInfo {
    cpu_num: u32,
    host_name: String,
    os_type: String,
    os_release: String,
    mem_info: MemInfo,
}

impl ComputerInfo {

    fn print(&self) {
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

        return ComputerInfo{
            cpu_num: cpu_num,
            host_name: host_name,
            os_type: os_type,
            os_release: os_release,
            mem_info: mem_info
        };
    }
}

fn main() {

    let computer_info = ComputerInfo{..Default::default()};
    computer_info.print();
}
