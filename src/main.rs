use sys_info;

const KILOBYTES_IN_MEGABYTE: i16 = 1024;

fn main() {

    let cpu_num = sys_info::cpu_num().unwrap();
    let host_name = sys_info::hostname().unwrap();
    let os_type = sys_info::os_type().unwrap();
    let os_release = sys_info::os_release().unwrap();
    let mem_info = sys_info::mem_info().unwrap();
    let mem_in_mbytes = mem_info.total / (KILOBYTES_IN_MEGABYTE as u64);

    println!("{}", host_name);
    println!("{} {}", os_type, os_release);
    println!("Number of cpus:  {}", cpu_num);
    println!("RAM total:  {}Mb", mem_in_mbytes);
}
