pub fn hello() -> String {
    "Hello Rust".to_string()
}

pub fn greet(name: &str) -> String {
    "Hello ".to_string() + name
    //format!("Hello {}", name)
}

pub fn append(mut s: String) -> String {
    format!("{}!", s)
    //s += "!";
    //s
}
