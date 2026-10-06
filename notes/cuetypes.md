# Base Cue
- Type: CueType {Audio, Fade, Start, Stop, Pause, Group}
- Number: f32
    - Get Number
    - Set Number
- Name: String
    - Get Name
    - Set Name
- Follow: FollowState {None, Follow, Continue}
    - Get Follow
    - Set Follow
    - Render Follow {maybe doable within the struct}
- Prewait: f32
    - Get Prewait
    - Set Prewait
- Postwait: f32
    - Get Postwait
    - Set Postwait
- Icon: String
    - Get Icon
    - Set Icon
- Status: PlayState {None, Playing, Paused, Stopped, Fading Up, Fading Down, Error}
    - Get PlayState
    - Set PlayState
- Target: TargetType {None, File, TargetCue}
    - Get Target
    - Set Target

- Parent: Cue

- PlayCue: void
- PanicCue: void
- LoadCue: void
- UnloadCue: void

```rust
TargetCue {
    number: f32,
    type: CueType,
    absoluteNumber: i32
}
```

## Audio Cue
- Audio File: File
    - Get Target
    - Set Target
- Now Playing: AtomicU64
    - Start Cue
    - Stop Cue
    - Fade Cue

```rust
AudioFile {
    name: String,
    file_path: String,
    bytes: Arc<[u8]>,
    player: Arc<Player>,
    now_playing: Arc<AtomicU64>,
}
```

## Fade Cue
- Target Number: f32
    - Get Target
    - Set Target
    - Get Fade
    - Set Fade

```rust
Fade {
    duration: f32,
    start_db: f32,
    end_db: f32,
    curve: FadeCurve {Linear, Log},
}
```

## Stop Cue
- Target Number: f32
    - Get Target
    - Set Target

## Pause Cue
- Target Number: f32
    - Get Target
    - Set Target

## Group Cue
- Children: [i32]
    - Add Child (Cue)
    - Remove Child (Cue)

# Misc
## File
- Start Time
    - Get Start Time
    - Set Start Time
- End Time
    - Get End Time
    - Set End Time
- Volume
    - Get Volume
    - Set Volume
- Integrated Fade
    - Set Start Fade
    - Set End Fade
