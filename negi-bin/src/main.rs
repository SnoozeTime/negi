use negi_core::Dispatcher;

fn main() -> Result<(), redis::RedisError> {
    //deser(encoded.unwrap()).inspect();
    let dispatcher = Dispatcher::default();
    dispatcher.run()
}
