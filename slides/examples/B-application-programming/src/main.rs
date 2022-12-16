// import an item from this crate, called `b`
use example_b::add;
// import an item from the `tracing` dependency
use tracing::info;

fn main() {
    // Use qualified path
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let x = 4;
    let y = 6;

    // Use imported items
    let z = add(x, y);
    info!("Let me just add {x} and {y}: {z}")
}
