use bevy::prelude::*;

use crate::{score::Score, state::AppState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (main_menu, score_ui))
            .add_systems(Startup, game_over)
            .add_systems(Update, hide_menus);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct ScoreUI;

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

fn score_ui(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    let container = NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        ..default()
    };

    let score = (
        TextBundle::from_section(
            format!("{}", score.value),
            TextStyle {
                font: asset_server.load("fonts/04B_19__.ttf"),
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Percent(2.5),
            ..default()
        }),
        ScoreUI,
    );

    let parent = commands.spawn(container).id();
    let child = commands.spawn(score).id();
    commands.entity(parent).push_children(&[child]);
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
        Query<&mut Visibility, With<ScoreUI>>,
    )>,
) {
    if *states.as_ref() == AppState::MainMenu {
        if let Ok(mut main) = menus.p0().get_single_mut() {
            *main.as_mut() = Visibility::Visible;
        }

        if let Ok(mut score) = menus.p2().get_single_mut() {
            *score.as_mut() = Visibility::Hidden;
        }
    }

    if *states.as_ref() == AppState::GameOver {
        if let Ok(mut over) = menus.p1().get_single_mut() {
            *over.as_mut() = Visibility::Visible;
        }
    }

    if *states.as_ref() == AppState::InGame {
        if let Ok(mut score) = menus.p2().get_single_mut() {
            *score.as_mut() = Visibility::Visible;
        }

        if let Ok(mut main_menu) = menus.p0().get_single_mut() {
            *main_menu.as_mut() = Visibility::Hidden;
        }

        if let Ok(mut over_over) = menus.p1().get_single_mut() {
            *over_over.as_mut() = Visibility::Hidden;
        }
    }
}

