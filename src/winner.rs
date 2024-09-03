use bevy::{
    asset::embedded_asset,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub struct WinnerPlugin;

impl Plugin for WinnerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<WinState>()
            .insert_resource(Winner {
                entity: None,
                wait_timer: Timer::default(),
            })
            // So that any entities created by these systems are automatically despawned after leaving
            // the WinState.
            .enable_state_scoped_entities::<WinState>()
            .add_systems(PreUpdate, detect_win.run_if(in_state(WinState::Detecting)))
            .add_systems(Update, display_win.run_if(in_state(WinState::Displaying)));

        let omit_prefix = "src";
        embedded_asset!(app, omit_prefix, "winner_assets/KaushanScript-Regular.ttf");
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum WinState {
    #[default]
    Detecting,
    Displaying,
}

#[derive(Resource)]
pub struct Winner {
    entity: Option<Entity>,
    wait_timer: Timer,
}

impl Winner {
    pub fn show_winner_screen(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }
}

#[allow(clippy::too_many_arguments)]
fn detect_win(
    mut commands: Commands,
    mut winner: ResMut<Winner>,
    mut next_state: ResMut<NextState<WinState>>,
    mut virtual_time: ResMut<Time<Virtual>>,
    texture_qry: Query<(&Sprite, &Handle<Image>)>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let Some(winner_entity) = winner.entity else {
        return;
    };
    // Someone just won, get stuff set up!
    winner.wait_timer = Timer::from_seconds(2.5, TimerMode::Repeating);
    next_state.set(WinState::Displaying);
    let font =
        asset_server.load("embedded://bevy_cleancut/winner_assets/KaushanScript-Regular.ttf");
    commands.spawn((
        StateScoped(WinState::Displaying),
        Name::new("Winner Text"),
        Text2dBundle {
            text: Text::from_section(
                "Winner!!!",
                TextStyle {
                    font_size: 196.0,
                    font,
                    ..Default::default()
                },
            )
            .with_justify(JustifyText::Center),
            transform: Transform::from_xyz(0.0, 0.0, 999.0),
            ..Default::default()
        },
    ));
    virtual_time.pause();
    let (sprite, texture) = texture_qry.get(winner_entity).unwrap();
    let mut transform = Transform::from_xyz(0.0, 150.0, 999.0);
    transform.scale = Vec3::splat(2.5);
    commands.spawn((
        StateScoped(WinState::Displaying),
        Name::new("Winning Player Picture"),
        SpriteBundle {
            sprite: sprite.clone(),
            transform,
            texture: texture.clone(),
            ..Default::default()
        },
    ));

    commands.spawn((
        StateScoped(WinState::Displaying),
        Name::new("Winning Player Darkened Background"),
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(1280.0, 768.0))),
            material: materials.add(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            transform: Transform::from_xyz(0.0, 0.0, 998.0),
            ..Default::default()
        },
    ));
}

fn display_win(
    mut winner: ResMut<Winner>,
    mut next_state: ResMut<NextState<WinState>>,
    time: Res<Time<Real>>,
    mut virtual_time: ResMut<Time<Virtual>>,
) {
    if winner.wait_timer.tick(time.delta()).just_finished() {
        next_state.set(WinState::Detecting);
        winner.entity = None;
        virtual_time.unpause();
    }
}
