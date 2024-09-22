pub trait PluginInformation {
    fn name(&self) -> String;
    fn get_instance(&self) -> Box<SayHello>;
}

pub trait SayHello {
    fn say_hello(&self) -> String;
}

pub mod components {
    use std::any::Any;

    pub trait Plugin: Any + Send + Sync {
        fn name(&self) -> &'static str;

        fn version(&self) -> &'static str;

        fn initialize(&self);

        fn execute(&self);
    }

    pub trait AsAny {
        fn as_any(&self) -> &dyn Any;
    }

    impl<T: Plugin+ 'static> AsAny for T {
        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    // Plugin metadata to descript the plguin.
    #[derive(Debug, Clone)]
    pub struct PluginMetadata {
        pub name: &'static str,
        pub version: &'static str,
        pub description: &'static str,
    }

    pub type PluginCreateFn = fn() -> Box<dyn Plugin>;
    
    #[macro_export]
    macro_rules! declare_plugin {
        ($metadata:expr, %create_fn:expr) => {
            pub fn get_plugin_metadata() -> plugin_api::PluginMetadata {
                metadata
            }

            pub fn create_plugin() -> Box<dyn plugin_api::Plugin> {
                create_fn()
            }
        };
    }
}

