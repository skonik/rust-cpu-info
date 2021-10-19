mod computer_info;
use computer_info::Printable;

fn main() {
    let computer_info = computer_info::ComputerInfo {
        ..Default::default()
    };
    computer_info.print();
}
