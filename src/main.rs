
fn main() {
    begin_setup();
    translate_command("documentclass", "article");
    end_setup();
}

fn begin_setup() {
    print!("\\begin{{setup}}\n");
}

fn end_setup() {
    print!("\\end{{setup}}\n");
}

fn document_class(argument: &str) {
    println!("\\documentclass{{{}}}", argument);
}

fn translate_command(command: &str, argument: &str) {
    match command {
        "documentclass" => document_class(argument),
        _ => println!("Unknown command: {}", command),
    }
}