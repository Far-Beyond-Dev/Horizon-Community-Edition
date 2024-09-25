use plugin_test_api::{Plugin, PluginInformation, SayHello, PluginContext};
use std::any::Any;
use std::future;

pub struct PluginMetadataType;

pub const PLUGIN_METADATA: PluginMetadataType = PluginMetadataType;

impl PluginInformation for PluginMetadataType {
    fn name(&self) -> String {
        "english".to_string()
    }

    fn get_instance(&self) -> Box<SayHello> {
        Box::new(English)
    }
}

pub struct English;

impl SayHello for English {
    fn say_hello(&self) -> String {
        "hello, world".to_string()
    }
}

impl Plugin for English {
    fn name(&self) ->  &'static str {
        todo!()
    }
    
    fn version(&self) ->  &'static str {
        todo!()
    }
    
    fn description(&self) ->  &'static str {
        todo!()
    }
    
    #[must_use]
    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn initialize<'life0,'life1,'async_trait>(&'life0 self,context: &'life1 mut PluginContext) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    }
    
    #[must_use]
    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn shutdown<'life0,'life1,'async_trait>(&'life0 self,context: &'life1 mut PluginContext) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    }
    
    #[must_use]
    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn on_enable<'life0,'life1,'async_trait>(&'life0 self,context: &'life1 mut PluginContext) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    }
    
    #[must_use]
    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn on_disable<'life0,'life1,'async_trait>(&'life0 self,context: &'life1 mut PluginContext) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    }
    
    #[must_use]
    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn on_game_event<'life0,'life1,'life2,'async_trait>(&'life0 self,event: &'life1 GameEvent,context: &'life2 mut PluginContext) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,'life2:'async_trait,Self:'async_trait {
        todo!()
    }
    
    #[must_use]
    #[allow(elided_named_lifetimes,clippy::type_complexity,clippy::type_repetition_in_bounds)]
    fn on_game_tick<'life0,'life1,'async_trait>(&'life0 self,delta_time:f32,context: &'life1 mut PluginContext) ->  ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> + ::core::marker::Send+'async_trait> >where 'life0:'async_trait,'life1:'async_trait,Self:'async_trait {
        todo!()
    }
    
    fn as_any(&self) ->  &dyn Any {
        todo!()
    }

    
}