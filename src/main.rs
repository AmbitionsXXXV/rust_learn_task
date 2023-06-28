pub mod module_first;
pub mod module_second;

fn main() {
    module_first::print_characters();
    module_second::sub_module::print_characters();
}
