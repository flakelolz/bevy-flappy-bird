use bevy::prelude::*;

use crate::{score::Score, state::AppState};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (main_menu, score_ui, game_over, score_board))
            .add_systems(
                Update,
                update_scoreboard.run_if(in_state(AppState::GameOver)),
            )
            .add_systems(Update, hide_menus);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
pub struct GameOver;

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct ScoreBoard;

#[derive(Component)]
struct CurrentScore;

#[derive(Component)]
struct MaxScore;

fn main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture: Handle<Image> = asset_server.load("ui/message.png");

    commands.spawn((
        SpriteBundle {
            texture,
            visibility: Visibility::Hidden,
            transform: Transform::from_xyz(0.0, 20.0, 99.0),
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
            transform: Transform::from_xyz(0.0, 100.0, 99.0),
            ..default()
        },
        GameOver,
    ));
}

fn score_board(mut commands: Commands, asset_server: Res<AssetServer>, score: Res<Score>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("ui/scoreboard.png"),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 99.0),
                scale: Vec3::new(2.1, 2.1, 1.0),
                ..default()
            },
            visibility: Visibility::Hidden,
            ..default()
        },
        ScoreBoard,
    ));

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

    let current_score = (
        TextBundle::from_section(
            format!("{}", score.value),
            TextStyle {
                font: asset_server.load("fonts/04B_19__.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            display: Display::None,
            position_type: PositionType::Absolute,
            right: Val::Percent(20.0),
            top: Val::Percent(45.0),
            ..default()
        }),
        CurrentScore,
    );

    let max_score = (
        TextBundle::from_section(
            format!("{}", score.max),
            TextStyle {
                font: asset_server.load("fonts/04B_19__.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            display: Display::None,
            position_type: PositionType::Absolute,
            right: Val::Percent(20.0),
            top: Val::Percent(54.0),
            ..default()
        }),
        MaxScore,
    );

    let parent = commands.spawn(container).id();
    let value = commands.spawn(current_score).id();
    let max = commands.spawn(max_score).id();
    commands.entity(parent).push_children(&[value, max]);
}

fn update_scoreboard(
    score: Res<Score>,
    mut current_score: Query<&mut Text, With<CurrentScore>>,
    mut max_score: Query<&mut Text, (With<MaxScore>, Without<CurrentScore>)>,
) {
    if let Ok(mut current) = current_score.get_single_mut() {
        current.sections[0].value = format!("{}", score.value);
    }

    if let Ok(mut max) = max_score.get_single_mut() {
        max.sections[0].value = format!("{}", score.max);
    }
}

// Hides menus based on which state the game is in
fn hide_menus(
    states: Res<State<AppState>>,
    mut menus: ParamSet<(
        Query<&mut Visibility, With<MainMenu>>,
        Query<&mut Visibility, With<GameOver>>,
        Query<&mut Visibility, With<ScoreUI>>,
        Query<&mut Visibility, With<ScoreBoard>>,
    )>,
    mut scores: Query<&mut Style, Or<(With<CurrentScore>, With<MaxScore>)>>,
) {
    if *states.as_ref() == AppState::MainMenu {
        if let Ok(mut main) = menus.p0().get_single_mut() {
            *main.as_mut() = Visibility::Visible;
        }

        if let Ok(mut score) = menus.p2().get_single_mut() {
            *score.as_mut() = Visibility::Hidden;
        }

        if let Ok(mut board) = menus.p3().get_single_mut() {
            *board.as_mut() = Visibility::Hidden;
        }

        for mut display in &mut scores {
            display.display = Display::None;
        }
    }

    if *states.as_ref() == AppState::GameOver {
        if let Ok(mut over) = menus.p1().get_single_mut() {
            *over.as_mut() = Visibility::Visible;
        }

        if let Ok(mut board) = menus.p3().get_single_mut() {
            *board.as_mut() = Visibility::Visible;
        }

        for mut display in &mut scores {
            display.display = Display::Flex;
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
