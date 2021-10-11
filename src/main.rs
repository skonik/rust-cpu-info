mod computer_info;

fn main() {
    let computer_info = computer_info::ComputerInfo {
        ..Default::default()
    };
    computer_info.print();
}
