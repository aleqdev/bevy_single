//! Get single entity from query
//!
//! ## Example
//!
//! Before using Single:
//! ```rust
//! # use bevy::prelude::*;
//! # #[derive(Component)]
//! # struct Player;
//! pub fn snap_camera(
//!     players: Query<&Transform, With<Player>>,
//!     mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
//! ) {
//!     let player = players.single();
//!     let mut camera = cameras.single_mut();
//!
//!     camera.translation = player.translation.xy().extend(100.0);
//! }
//! ```
//!
//! After using Single:
//! ```rust
//! # use bevy::prelude::*;
//! # use bevy_single::prelude::*;
//! # #[derive(Component)]
//! # struct Player;
//! pub fn snap_camera(
//!     player: Single<&Transform, With<Player>>,
//!     mut camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
//! ) {
//!     camera.translation = player.translation.xy().extend(100.0);
//! }
//! ```
//!
//! Or like this:
//! ```rust
//! # use bevy::prelude::*;
//! # use bevy_single::prelude::*;
//! # #[derive(Component)]
//! # struct Player;
//! pub fn snap_camera(
//!     Single(player): Single<&Transform, With<Player>>,
//!     Single(mut camera): Single<&mut Transform, (With<Camera>, Without<Player>)>,
//! ) {
//!     camera.translation = player.translation.xy().extend(100.0);
//! }
//! ```
//!
//! ## Example with multiple components
//!
//! Before using Single:
//! ```rust
//! # use bevy::prelude::*;
//! # use bevy_rapier2d::prelude::*;
//! # #[derive(Component)]
//! # struct Player;
//! pub fn move_with_wasd(
//!     mut players: Query<(&mut KinematicCharacterController, &Player)>,
//!     input: Res<ButtonInput<KeyCode>>,
//!     time: Res<Time>,
//! ) {
//!     let (mut controller, player) = players.single_mut();
//!
//!     const WASD_DISPATCH: [(KeyCode, Vec2); 4] = [
//!         (KeyCode::KeyW, Vec2::Y),
//!         (KeyCode::KeyA, Vec2::NEG_X),
//!         (KeyCode::KeyS, Vec2::NEG_Y),
//!         (KeyCode::KeyD, Vec2::X),
//!     ];
//!
//!     let mut translation = None;
//!
//!     for (key, vector) in WASD_DISPATCH {
//!         if input.pressed(key) {
//!             translation = Some(translation.unwrap_or(Vec2::ZERO) + vector);
//!         }
//!     }
//!
//!     controller.translation =
//!         translation.map(|t| t.normalize() * player.speed * time.delta_seconds());
//! }
//! ```
//!
//! After using Single:
//! ```rust
//! # use bevy::prelude::*;
//! # use bevy_rapier2d::prelude::*;
//! # use bevy_single::prelude::*;
//! # #[derive(Component)]
//! # struct Player;
//! pub fn move_with_wasd(
//!     Single((
//!         mut controller,
//!         player
//!     )): Single<(&mut KinematicCharacterController, &Player)>,
//!     input: Res<ButtonInput<KeyCode>>,
//!     time: Res<Time>,
//! ) {
//!     const WASD_DISPATCH: [(KeyCode, Vec2); 4] = [
//!         (KeyCode::KeyW, Vec2::Y),
//!         (KeyCode::KeyA, Vec2::NEG_X),
//!         (KeyCode::KeyS, Vec2::NEG_Y),
//!         (KeyCode::KeyD, Vec2::X),
//!     ];
//!
//!     let mut translation = None;
//!
//!     for (key, vector) in WASD_DISPATCH {
//!         if input.pressed(key) {
//!             translation = Some(translation.unwrap_or(Vec2::ZERO) + vector);
//!         }
//!     }
//!
//!     controller.translation =
//!         translation.map(|t| t.normalize() * player.speed * time.delta_seconds());
//! }
//! ```
//!
//! ## Example with SystemParam
//!
//! Before using Single:
//! ```rust
//! # use bevy::prelude::*;
//! # #[derive(Component)]
//! # struct SpritesheetAnimation;
//! pub fn example(
//!     mut param_set: ParamSet<(
//!         Query<Entity>,
//!         Query<&mut SpritesheetAnimation>
//!     )>,
//! ) {
//!     let entities = param_set.p0();
//!     let mut animation = param_set.p1().single_mut();
//!
//!     // ...
//! }
//! ```
//!
//! After using Single:
//! ```rust
//! # use bevy::prelude::*;
//! # use bevy_single::prelude::*;
//! # #[derive(Component)]
//! # struct SpritesheetAnimation;
//! pub fn example(
//!     mut param_set: ParamSet<(
//!         Query<Entity>,
//!         Single<&mut SpritesheetAnimation>
//!     )>,
//! ) {
//!     let entities = param_set.p0();
//!     let Single(mut animation) = param_set.p1();
//!
//!     // ...
//! }
//! ```

use std::{borrow::Cow, mem, ops::{Deref, DerefMut}};

use bevy_ecs::{archetype::{Archetype, ArchetypeComponentId}, component::{ComponentId, Tick}, query::{Access, FilteredAccessSet, QueryData, QueryFilter, QueryState, ReadOnlyQueryData, WorldQuery}, system::{Query, ReadOnlySystemParam, SystemMeta, SystemParam}, world::{unsafe_world_cell::UnsafeWorldCell, World}};


pub mod prelude {
    pub use super::Single;
}


/// Helper trait for avoiding unused type parameters and lifetimes without PhantomData field on Single
pub trait SingleDescriptor<'world, 'state, D: QueryData, F: QueryFilter> {
    type D: QueryData;    
}

impl <'world, 'state, D: QueryData, F: QueryFilter> SingleDescriptor<'world, 'state, D, F> for () {
    type D = D;
}

/// Get single entity from query
/// 
/// ## Example
/// 
/// Before using Single:
/// ```rust
/// # use bevy::prelude::*;
/// # #[derive(Component)]
/// # struct Player;
/// pub fn snap_camera(
///     players: Query<&Transform, With<Player>>,
///     mut cameras: Query<&mut Transform, (With<Camera>, Without<Player>)>,
/// ) {
///     let player = players.single();
///     let mut camera = cameras.single_mut();
/// 
///     camera.translation = player.translation.xy().extend(100.0);
/// }
/// ```
/// 
/// After using Single:
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_single::prelude::*;
/// # #[derive(Component)]
/// # struct Player;
/// pub fn snap_camera(
///     player: Single<&Transform, With<Player>>,
///     mut camera: Single<&mut Transform, (With<Camera>, Without<Player>)>,
/// ) {
///     camera.translation = player.translation.xy().extend(100.0);
/// }
/// ```
/// 
/// Or like this:
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_single::prelude::*;
/// # #[derive(Component)]
/// # struct Player;
/// pub fn snap_camera(
///     Single(player): Single<&Transform, With<Player>>,
///     Single(mut camera): Single<&mut Transform, (With<Camera>, Without<Player>)>,
/// ) {
///     camera.translation = player.translation.xy().extend(100.0);
/// }
/// ```
/// 
/// ## Example with multiple components
/// 
/// Before using Single:
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_rapier2d::prelude::*;
/// # #[derive(Component)]
/// # struct Player;
/// pub fn move_with_wasd(
///     mut players: Query<(&mut KinematicCharacterController, &Player)>,
///     input: Res<ButtonInput<KeyCode>>,
///     time: Res<Time>,
/// ) {
///     let (mut controller, player) = players.single_mut();
/// 
///     const WASD_DISPATCH: [(KeyCode, Vec2); 4] = [
///         (KeyCode::KeyW, Vec2::Y),
///         (KeyCode::KeyA, Vec2::NEG_X),
///         (KeyCode::KeyS, Vec2::NEG_Y),
///         (KeyCode::KeyD, Vec2::X),
///     ];
/// 
///     let mut translation = None;
/// 
///     for (key, vector) in WASD_DISPATCH {
///         if input.pressed(key) {
///             translation = Some(translation.unwrap_or(Vec2::ZERO) + vector);
///         }
///     }
/// 
///     controller.translation =
///         translation.map(|t| t.normalize() * player.speed * time.delta_seconds());
/// }
/// ```
/// 
/// After using Single:
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_rapier2d::prelude::*;
/// # use bevy_single::prelude::*;
/// # #[derive(Component)]
/// # struct Player;
/// pub fn move_with_wasd(
///     Single((
///         mut controller, 
///         player
///     )): Single<(&mut KinematicCharacterController, &Player)>,
///     input: Res<ButtonInput<KeyCode>>,
///     time: Res<Time>,
/// ) {
///     const WASD_DISPATCH: [(KeyCode, Vec2); 4] = [
///         (KeyCode::KeyW, Vec2::Y),
///         (KeyCode::KeyA, Vec2::NEG_X),
///         (KeyCode::KeyS, Vec2::NEG_Y),
///         (KeyCode::KeyD, Vec2::X),
///     ];
/// 
///     let mut translation = None;
/// 
///     for (key, vector) in WASD_DISPATCH {
///         if input.pressed(key) {
///             translation = Some(translation.unwrap_or(Vec2::ZERO) + vector);
///         }
///     }
/// 
///     controller.translation =
///         translation.map(|t| t.normalize() * player.speed * time.delta_seconds());
/// }
/// ```
/// 
/// ## Example with SystemParam
/// 
/// Before using Single:
/// ```rust
/// # use bevy::prelude::*;
/// # #[derive(Component)]
/// # struct SpritesheetAnimation;
/// pub fn example(
///     mut param_set: ParamSet<(
///         Query<Entity>,
///         Query<&mut SpritesheetAnimation>
///     )>,
/// ) {
///     let entities = param_set.p0();
///     let mut animation = param_set.p1().single_mut();
/// 
///     // ...
/// }
/// ```
/// 
/// After using Single:
/// ```rust
/// # use bevy::prelude::*;
/// # use bevy_single::prelude::*;
/// # #[derive(Component)]
/// # struct SpritesheetAnimation;
/// pub fn example(
///     mut param_set: ParamSet<(
///         Query<Entity>,
///         Single<&mut SpritesheetAnimation>
///     )>,
/// ) {
///     let entities = param_set.p0();
///     let Single(mut animation) = param_set.p1();
/// 
///     // ...
/// }
/// ```
pub struct Single<'world, 'state, D: QueryData, F: QueryFilter = (), Desc: SingleDescriptor<'world, 'state, D, F> = ()>(pub <Desc::D as WorldQuery>::Item<'world>);


impl<'world, 'state, D: QueryData, F: QueryFilter, Desc: SingleDescriptor<'world, 'state, D, F>> Deref for Single<'world, 'state, D, F, Desc> {
    type Target = <Desc::D as WorldQuery>::Item<'world>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'world, 'state, D: QueryData, F: QueryFilter, Desc: SingleDescriptor<'world, 'state, D, F>> DerefMut for Single<'world, 'state, D, F, Desc> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

unsafe impl<'w, 's, D: ReadOnlyQueryData + 'static, F: QueryFilter + 'static> ReadOnlySystemParam
    for Single<'w, 's, D, F>
{
}


// SAFETY: Relevant query ComponentId and ArchetypeComponentId access is applied to SystemMeta. If
// this Query conflicts with any prior access, a panic will occur.
unsafe impl<'ww, 'ss, D: QueryData + 'static, F: QueryFilter + 'static> SystemParam for Single<'ww, 'ss, D, F> {
    type State = QueryState<D, F>;
    type Item<'w, 's> = Single<'w, 's, D, F>;

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
        <Query<'ww, 'ss, D, F> as SystemParam>::init_state(world, system_meta)
    }

    unsafe fn new_archetype(
        state: &mut Self::State,
        archetype: &Archetype,
        system_meta: &mut SystemMeta,
    ) {
        <Query<'ww, 'ss, D, F> as SystemParam>::new_archetype(state, archetype, system_meta)
    }

    #[inline]
    unsafe fn get_param<'w, 's>(
        state: &'s mut Self::State,
        system_meta: &SystemMeta,
        world: UnsafeWorldCell<'w>,
        change_tick: Tick,
    ) -> Self::Item<'w, 's> {
        // SAFETY: We have registered all of the query's world accesses,
        // so the caller ensures that `world` has permission to access any
        // world data that the query needs.
        unsafe {
            state.validate_world(world.id());

            let public_meta: &SystemMetaPublicFields = mem::transmute(system_meta);
            
            let single = state.get_single_unchecked_manual(
                world,
                public_meta.last_run,
                change_tick,
            ).unwrap();

            Single(single)
        }
    }
}


struct SystemMetaPublicFields {
    _name: Cow<'static, str>,
    _component_access_set: FilteredAccessSet<ComponentId>,
    _archetype_component_access: Access<ArchetypeComponentId>,
    _is_send: bool,
    _has_deferred: bool,
    last_run: Tick,
    #[cfg(feature = "trace")]
    _system_span: Span,
    #[cfg(feature = "trace")]
    _commands_span: Span,
}
