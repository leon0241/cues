# Message
- Navigation: NavigationCmd
- Playback: PlaybackCmd

- Combo: ComboCmd
- Input: InputCmd

- Editor: EditorCmd
    - Quit
    - EnterInsertStart
    - EnterInsertEnd
    - CancelInsertEnterNormal
    - SaveInsertEnterNormal
    - EnterVisual

- **ModKey: ModKeyCmd**
    - Yank: CueCmd
    - Delete: CueCmd
    - Change: CueCmd
    **CueCmd**
        - Cue
        - Name
        - Number
        - Target
        - PreWait
        - PostWait
        - Follow
- **Create: CreateCmd**
    - CreateAudio
    - CreateFade
    - CreateStart
    - CreateStop
    - CreatePause

# Jump List

Message - ModKey -  - Name(String)
Message - ModKey - Delete - Cue(String) -> 
Message - Create - CreateFade
