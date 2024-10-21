mod tokeniser;

fn main() {
    let input = "
    begin(setup)
    documentclass('article')
    A = [
        [a, b, c]
        [d, e, f]
    ]
    $(sum (n -> n-1) \\gx)
    end(setup)
    ";

    let tokens = tokeniser::tokenise(input);
    for token in tokens {
        print!("{:?}\n", token);
    }

}

// fn begin_setup() {
//     print!("\\begin{{setup}}\n");
// }

// fn end_setup() {
//     print!("\\end{{setup}}\n");
// }

// fn document_class(argument: &str) {
//     println!("\\documentclass{{{}}}", argument);
// }

// fn translate_command(command: &str, argument: &str) {
//     match command {
//         "documentclass" => document_class(argument),
//         _ => println!("Unknown command: {}", command),
//     }
// }
