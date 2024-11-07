pub use horizon_plugin_api::Plugin;

pub trait Plugin_API: {
    fn new() -> Plugin {
        println!("Hello from the test plugin!!!!!");
        let new_plugin = Plugin {};

        new_plugin
    }
    fn thing() -> String {
        "Hello World!".to_string()
    }
}

impl Plugin_API for Plugin {
    fn new() -> Plugin {
        Plugin {  }
    }
}