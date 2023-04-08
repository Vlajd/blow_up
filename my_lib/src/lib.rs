#[derive(Default)]
struct Numbers(Vec<u32>);

impl From<my_types::Numbers> for &Numbers {
    fn from(value: my_types::Numbers) -> Self {
        unsafe { std::mem::transmute::<_, Self>(value) }
    }
}

impl From<my_types::Numbers> for &mut Numbers {
    fn from(value: my_types::Numbers) -> Self {
        unsafe { std::mem::transmute::<_, Self>(value) }
    }
}

impl From<&mut Numbers> for my_types::Numbers {
    fn from(value: &mut Numbers) -> Self {
        unsafe { std::mem::transmute::<_, my_types::Numbers>(value) }
    }
}


#[no_mangle]
extern fn with_padding(input: usize) -> usize {
    (input + 7) & !7
}

#[no_mangle]
extern fn append_number(numbers: my_types::Numbers, n: u32) {
    let numbers: &mut Numbers = numbers.into();
    numbers.0.push(n)
}

#[no_mangle]
extern fn print_numbers(numbers: my_types::Numbers) {
    let numbers: &Numbers = numbers.into();
    numbers.0.iter().for_each(|num| print!("{}, ", num) );
    println!("");
}

pub fn run() {
    let numbers = &mut Numbers::default();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let lib = unsafe { libloading::Library::new("../../my_main/target/debug/my_main.dll").unwrap() };
        let main: libloading::Symbol<fn(input: my_types::Action)> = unsafe { lib.get(b"my_main").unwrap() };

        
        if input.len() == 0 {
            continue;
        }

        let input: Vec<_> = input.split(" ").collect();
        
        match input[0].to_lowercase().as_str() {
            "padding" => if let Some(str) = input.get(1) {
                if let Ok(num) = str.parse::<usize>() {
                    main(my_types::Action::Num(num))
                } else {
                    println!("Expected number, found {}", str)
                }
            } else {
                println!("Expected second argument!")
            }
            "append" => if let Some(str) = input.get(1) {
                if let Ok(num) = str.parse::<u32>() {
                    main(my_types::Action::Append(numbers.into(), num))
                } else {
                    println!("Expected number, found {}", str)
                }
            } else {
                println!("Expected second argument!")
            }
            "print" => main(my_types::Action::Print(numbers.into())),
            "clear" => main(my_types::Action::Clear),
            "exit" => break,
            _ => println!("Unknown command!")
        }
    }
}