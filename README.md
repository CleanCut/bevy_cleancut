# bevy_cleancut

Random utility stuff for bevy for my personal use. You're welcome to use it, too, if you like.

# Stuff

It's all in the prelude: `use bevy_cleancut::prelude::*`

|Thing|Does What?|
|---|---|
|`play_sound`|Plays a single sound effect with a minor, random pitch alteration which then despawns itself.|
|`collision_started`|Easy way to verify if entities of two specific types have _started_ colliding. (`bevy_rapier2d`)|
|`collision_stopped`|Easy way to verify if entities of two specific types have _stopped_ colliding. (`bevy_rapier2d`)|
|`particle_trail_bundle`|Creates a particle trail as a bundle to be added as a child to an entity|
|`spawn_particle_poof`|Creates a oneshot particle system that poofs in a global location|
|`PlayerColors`|A struct with nice default colors to use for players|

# Compatibility

|bevy|bevy_cleancut
|---|---|
| `0.14.*` | `0.1.0`-`0.2.x` |
| `< 0.14` | Unsupported |
