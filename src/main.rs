use std::{
    env, fs,
    process::{exit, Command},
};

fn main() {
    let argv: Vec<_> = env::args().collect();
    if argv.len() < 2 {
        println!("File not specified as argument");
        exit(1);
    }

    let mut code =
        String::from("#include<stdio.h>\nint main(){char array[30000]={0};char *ptr=array;");

    let file_name = &argv[1];
    let input = fs::read_to_string(file_name).expect("Unable to read file");

    for chr in input.chars() {
        match chr {
            '>' => code += "++ptr;",
            '<' => code += "--ptr;",
            '+' => code += "++*ptr;",
            '-' => code += "--*ptr;",
            '.' => code += "putchar(*ptr);",
            ',' => code += "*ptr=getchar();",
            '[' => code += "while(*ptr){",
            ']' => code += "}",
            _ => {}
        }
    }

    code.push_str("return 0;}");

    fs::write("./out.c", &code).expect("Failed to write to file");

    let mut bin_name = "/".to_owned();
    bin_name.push_str(file_name.split_once('.').unwrap().0);

    Command::new("gcc")
        .args(&["out.c", "-o", bin_name.as_ref()])
        .spawn()
        .expect("Unable to run gcc")
        .wait()
        .unwrap();

    fs::remove_file("./out.c").unwrap();
    Command::new(bin_name).spawn().unwrap();
}
