<!-- next-header -->
## [Unreleased] - ReleaseDate

- Change `spawn_particle_poof` to take a reference to a Transform, clone the copy internally and set the Z to `10.0`
- Remove the `winner` plugin. (Moved to `bevy_scoreboard`)

## [0.2.6] - 2024-09-02

- Re-export `input_toggle_active` and `GamepadConnectionEvent` from `bevy`, since they aren't in its prelude.

## [0.2.5] - 2024-09-02

- Change `platform_boundaries_collider` into `create_gravity2d_boundaries`

## [0.2.4] - 2024-08-31

- Add `WinnerPlugin`. Pause the game, display a "Winner" message along with an enlarged copy of a sprite as the winner. To use it, add the plugin, then in a system access the `Winner` resource and call its `show_winner_screen` method, passing it a sprite entity.

## [0.2.3] - 2024-08-16

- Add `Action` enum for use with `leafwing-input-manager` suitable for 2d platformer with `Run` (single axis) and `Jump` (button) variants.

## [0.2.2] - 2024-08-16

- Add `platformer_boundaries_collider` function to return compound collider for nice default boundaries for a platformer.
- (Potentially Breaking) Renamed `collision` module to `rapier` (import from `bevy_cleancut::prelude` to avoid breakage from layout refactors).

## [0.2.1] - 2024-08-15

- Add particles and colors items to the prelude

## [0.2.0] - 2024-08-15

- Add `PlayerColors` with some good default colors for players.

## [0.1.0] - 2024-08-15

- Initial release with `collision_started`, `collision_stopped`, `play_sound`, `particle_trail_bundle`, and `spawn_particle_poof`

<!-- next-url -->
[Unreleased]: https://github.com/CleanCut/bevy_cleancut/compare/v0.2.6...HEAD
[0.2.6]: https://github.com/CleanCut/bevy_cleancut/compare/v0.2.5...v0.2.6
[0.2.5]: https://github.com/CleanCut/bevy_cleancut/compare/v0.2.4...v0.2.5
[0.2.4]: https://github.com/CleanCut/bevy_cleancut/compare/v0.2.3...v0.2.4
[0.2.3]: https://github.com/CleanCut/bevy_cleancut/compare/v0.2.2...v0.2.3
[0.2.2]: https://github.com/CleanCut/bevy_cleancut/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/CleanCut/bevy_cleancut/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/CleanCut/bevy_cleancut/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/CleanCut/bevy_cleancut/compare/v0.0.0...v0.1.0

