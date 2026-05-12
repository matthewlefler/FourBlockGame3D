# 3D4BlockGame
A 3d game based around combining blocks to clear lines built using bevy.

### Note
This exists because I want to learn bevy,   
and I might as well create an actually somewhat polished version of "stacking pieces made of 4 blocks game".

Also one without bugs.

## Note 2 Electric Boogaloo
This code base is cursed, first time actually using Rust too, so there are many *things* just off about it.   
Will I fix those things, **no** if it works I'm not touching it.    
You're welcome to make an *issue* if you think it'll help though.   

# TODO
- row clearing
- point system
  - t-spin how detect?
- highscores
  - local
  - global
    - requires backend api tie-in
      - arbitrary url?
    - (i should write a rust library for this)
- input system
  - from data_file? 
    - input.json
    - is there even a point tho?
      - yes there is (custom loading by user etc.)
- UI
  - from data_file??
    - (this sounds really hard (maybe (as i think about it maybe not)))
    - ui.json 
  - main menu
  - tutorial
  - start game
    - input handling
      - whos player 1 v 2 v N
  - settings
    - controls
    - audio
    - looks
      - color pallete
      - block models
- animations
  - from data_file
    - animations.json
- gamemodes
  - from data_file
    - gamemodes.json
  - 1v1
  - ...
  - 1v1v1v1v1v1v1v1v1v1v1v1v1 (would need a like 100_000_000 by 1_000 display but lol)
  - 2v2
  - NvN
  - solo inf (no level)
  - solo (with levels) (look up how they work)
- flatpaking
  - yep ima make a tool for this (hwadiodwoiuhdwaouh why do I do this to myself...)