use bevy::prelude::*;

use crate::state::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, main_menu)
            .add_systems(Startup, game_over)
            .add_systems(Update, hide_menus);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
pub struct GameOver;

fn main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("ui/message.png");

    commands.spawn((
        SpriteBundle {
            texture,
            visibility: Visibility::Hidden,
            transform: Transform::from_xyz(0.0, 0.0, 99.0),
            ..default()
        },
        MainMenu,
    ));
}

fn game_over(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("ui/gameover.png");

    commands.spawn((
        SpriteBundle {
            texture,
            visibility: Visibility::Hidden,
            transform: Transform::from_xyz(0.0, 0.0, 99.0),
            ..default()
        },
        GameOver,
    ));
}

fn hide_menus(
    states: Res<State<AppState>>,
    mut menus: ParamSet<(
        Query<&mut Visibility, With<MainMenu>>,
        Query<&mut Visibility, With<GameOver>>,
    )>,
) {
    if *states.as_ref() == AppState::MainMenu {
        if let Ok(mut main) = menus.p0().get_single_mut() {
            *main.as_mut() = Visibility::Visible;
        }
    }

    if *states.as_ref() == AppState::GameOver {
        if let Ok(mut over) = menus.p1().get_single_mut() {
            *over.as_mut() = Visibility::Visible;
        }
    }

    if *states.as_ref() == AppState::InGame {
        if let Ok(mut main_menu) = menus.p0().get_single_mut() {
            *main_menu.as_mut() = Visibility::Hidden;
        }

        if let Ok(mut over_over) = menus.p1().get_single_mut() {
            *over_over.as_mut() = Visibility::Hidden;
        }
    }
}
