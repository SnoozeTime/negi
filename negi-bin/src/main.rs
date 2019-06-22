use negi_core::Dispatcher;

use negi_macro::task;

#[task(module)]
fn MyCustom(some_string: String, x: u32) {
    println!("HHHHHH {} + {}", some_string, x);
}

fn main() -> Result<(), redis::RedisError> {
    //deser(encoded.unwrap()).inspect();
    let dispatcher = Dispatcher::default();
    dispatcher.run()
}
