use colored::Colorize;

pub fn print_error(text:&str){
    println!("{}",text.red().bold());
}

pub fn print_info(text:&str, param:&str){
    println!("{} {}",text.cyan(),param.magenta());
}

pub fn print_common_text(text:&str){
    println!("{}",text.italic().bold());
}

pub fn print_cyan(text:&str){
    println!("{}",text.italic().bold().cyan());
}