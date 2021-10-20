mod computer_info;
use computer_info::Printable;


fn print_computer_info(computer_info: impl Printable) {
    computer_info.print();
}

fn main() {
    let computer_info = computer_info::ComputerInfo {
        ..Default::default()
    };
    print_computer_info(computer_info);
}
