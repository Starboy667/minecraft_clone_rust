use bevy::prelude::Component;

#[derive(Component)]
pub struct Player {}

impl Default for Player {
    fn default() -> Self {
        Self {}
    }
}
