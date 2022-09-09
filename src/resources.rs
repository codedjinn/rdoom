use bevy::prelude::*;

struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_resources);
    }
}

fn load_resources(
    mut commands: Commands,
    mut server: ResMut<AssetServer>
) {
    
}

// TODO: Refactor this later
pub struct Resources {
    images: Vec<Resource<Handle<Image>>>,
}

pub enum LoadingState {
    None,
    Loading,
    Loaded,
    Failed(&str)
}

pub struct Resource<T> {
    state: LoadingState,
    name: String,
    handle: Handle<T>,
}
