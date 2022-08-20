use concurrency;
fn main () {
    // function is a basic demonstration of how threads work with concurrency
    // concurrency::intro_concurrency();

    println!("\nJoin handle");
    concurrency::join_handle();

    println!("\nMove example");
    concurrency::move_example();

    println!("\nBasic channel example");
    concurrency::channel_example();

    println!("\nChannel showing concurrency as well");
    concurrency::channel_display_concurrency();

    println!("\nFunction showing how multiple producers can pass inputs to a single consumer thread");
    concurrency::multiple_producer_single_consumer();

    println!("\nMoving into shared state concurrency. Demonstration of mutex.");
    concurrency::example_mutex();

    println!("\nSharing state between threads using atomic reference counts and mutexes");
    concurrency::sharing_mutex_btw_threads();
}