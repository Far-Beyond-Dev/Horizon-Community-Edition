#[macro_export]
macro_rules! load_plugins {
    ($($plugin:ident),* $(,)?) => {
        {
            let mut plugins = HashMap::new();
            $(
                plugins.insert(
                    stringify!($plugin).to_string(),
                    (Pluginstate::ACTIVE, <$plugin::Plugin as $plugin::PluginConstruct>::new(plugins.clone())),
                );
            )*
            
            plugins
        }
    };
}