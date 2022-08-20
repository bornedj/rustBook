use concurrency;
fn main () {
    // function is a basic demonstration of how threads work with concurrency
    // concurrency::intro_concurrency();

    println!("\nJoin handle");
    concurrency::join_handle();

    println!("\nMove example");
    concurrency::move_example();

}