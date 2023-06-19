fn main() {
    println!("arguments:");

    for argument in std::env::args() {
        println!("  {argument}");
    }

    println!("\nvariables:");
    // print environment variables
    for (key, value) in std::env::vars() {
        println!("  {key}: {value}");
    }
}
