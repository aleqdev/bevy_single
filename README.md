Get single entity from query

## Example

Before using Single:
```rust
use bevy::prelude::*;

#[derive(Component)]
struct Player;

pub fn snap_camera(
    players: Query<&Transform, With<Player>>,
    mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player = players.single();
    let mut camera = cameras.single_mut();

    camera.translation = player.translation.xy().extend(100.0);
}
```

After using Single:
```rust
use bevy::prelude::*;
use bevy_single::prelude::*;

#[derive(Component)]
struct Player;

pub fn snap_camera(
    player: Single<&Transform, With<Player>>,
    mut camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    camera.translation = player.translation.xy().extend(100.0);
}
```

Or like this:
```rust
use bevy::prelude::*;
use bevy_single::prelude::*;

#[derive(Component)]
struct Player;

pub fn snap_camera(
    Single(player): Single<&Transform, With<Player>>,
    Single(mut camera): Single<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    camera.translation = player.translation.xy().extend(100.0);
}
```

## Example with multiple components

Before using Single:
```rust
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Player;

pub fn move_with_wasd(
    mut players: Query<(&mut KinematicCharacterController, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut controller, player) = players.single_mut();

    const WASD_DISPATCH: [(KeyCode, Vec2); 4] = [
        (KeyCode::KeyW, Vec2::Y),
        (KeyCode::KeyA, Vec2::NEG_X),
        (KeyCode::KeyS, Vec2::NEG_Y),
        (KeyCode::KeyD, Vec2::X),
    ];

    let mut translation = None;

    for (key, vector) in WASD_DISPATCH {
        if input.pressed(key) {
            translation = Some(translation.unwrap_or(Vec2::ZERO) + vector);
        }
    }

    controller.translation =
        translation.map(|t| t.normalize() * player.speed * time.delta_seconds());
}
```

After using Single:
```rust
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_single::prelude::*;

#[derive(Component)]
struct Player;

pub fn move_with_wasd(
    Single((
        mut controller,
        player
    )): Single<(&mut KinematicCharacterController, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    const WASD_DISPATCH: [(KeyCode, Vec2); 4] = [
        (KeyCode::KeyW, Vec2::Y),
        (KeyCode::KeyA, Vec2::NEG_X),
        (KeyCode::KeyS, Vec2::NEG_Y),
        (KeyCode::KeyD, Vec2::X),
    ];

    let mut translation = None;

    for (key, vector) in WASD_DISPATCH {
        if input.pressed(key) {
            translation = Some(translation.unwrap_or(Vec2::ZERO) + vector);
        }
    }

    controller.translation =
        translation.map(|t| t.normalize() * player.speed * time.delta_seconds());
}
```

## Example with SystemParam

Before using Single:
```rust
use bevy::prelude::*;

#[derive(Component)]
struct SpritesheetAnimation;

pub fn example(
    mut param_set: ParamSet<(
        Query<Entity>,
        Query<&mut SpritesheetAnimation>
    )>,
) {
    let entities = param_set.p0();
    let mut animation = param_set.p1().single_mut();
    // ...
}
```

After using Single:
```rust
use bevy::prelude::*;
use bevy_single::prelude::*;

#[derive(Component)]
struct SpritesheetAnimation;

pub fn example(
    mut param_set: ParamSet<(
        Query<Entity>,
        Single<&mut SpritesheetAnimation>
    )>,
) {
    let entities = param_set.p0();
    let Single(mut animation) = param_set.p1();
    // ...
}
```
