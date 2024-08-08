use bevy::{
    asset::Handle,
    prelude::{Image, Resource},
};

#[derive(Resource, Clone)]
pub struct TextureHandles {
    pub dirt: Handle<Image>,
}
