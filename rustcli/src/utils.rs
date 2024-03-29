pub fn echo_function(input: &str){
    if input.is_empty() {
        String::new();
    } else {
        println!("{}", input);
    }
}
