use my_types::{Numbers, Action};

extern {
    fn with_padding(num: usize) -> usize;
    fn append_number(numbers: Numbers, num: u32);
    fn print_numbers(numbers: Numbers);
}

#[no_mangle]
extern fn my_main(input: Action) {
    match input {
        Action::Num(num) => unsafe { println!("Size {} with padding is {}", num, with_padding(num - 1)) }
        Action::Append(nums, num) => unsafe { append_number(nums, num) }
        Action::Print(nums) => unsafe { print_numbers(nums) }
        Action::Clear => { std::process::Command::new("clear").output().unwrap(); },
        _ => ()
    };
}
