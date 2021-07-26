use std::{env, fs, process::exit};

fn main() {
    let argv: Vec<_> = env::args().collect();
    if argv.len() < 2 {
        println!("File not specified as argument");
        exit(1);
    }

    let mut code =
        String::from("#include<stdio.h>\nint main(){char array[30000]={0};char *ptr=array;");

    let input = fs::read_to_string(&argv[1]).expect("Unable to read file");

    for chr in input.chars() {
        match chr {
            '>' => code.push_str("++ptr;"),
            '<' => code.push_str("--ptr;"),
            '+' => code.push_str("++*ptr;"),
            '-' => code.push_str("--*ptr;"),
            '.' => code.push_str("putchar(*ptr);"),
            ',' => code.push_str("*ptr=getchar();"),
            '[' => code.push_str("while(*ptr){"),
            ']' => code.push_str("}"),
            _ => {}
        }
    }

    code.push_str("return 0;}");

    fs::write("./out.c", &code).expect("Failed to write to file");
}
