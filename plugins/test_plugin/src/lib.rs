pub use horizon_plugin_api::Plugin;

// Define the trait properly
pub trait Plugin_API {    
    fn thing(&self) -> String;
}

pub trait Plugin_Construct {
    // If you want default implementations, mark them with 'default'
    fn new() -> Plugin;
}


// Implement constructor separately
impl Plugin_Construct for Plugin {
    fn new() -> Plugin {
        println!("Hello from the test plugin!!!!!");
        Plugin {}
    }
}

// Implement the trait for Plugin
impl Plugin_API for Plugin {    
    // Add the thing() method implementation
    fn thing(&self) -> String {
        "Hello from specific plugin implementation!".to_string()
    }
}