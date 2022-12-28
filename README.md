# Introduction
 
RustyCatan, to be built using bevy -- an ECS (Entity Controller System)-based game engine.

## Running

Dependencies should auto-install (except for OS dependencies) when running via `cargo run`.

To run with fast compiles enabled: `cargo run --features bevy/dynamic`

# TODO

## Development

- take notes on bevy throughout development process
- move task lisk to some other medium (trello, google sheets, idk)

## GUI

- spawn main window
- create main UI chunks
    - board
        - initially, colored hexagons for tiles?
    - card stacks
    - player cards area
- insert some basic ui text

## Art

- create a list of what art we need
- how do we handle dice?
- create environment cards
- create environment tiles (the hexagons)

## Logic

- figure out how single player will work
    - NPC players, filling the role of real players
- concept of 'the game' (main func)
- 'turns'
- capture board state
- tiles give 'resources'

## Avoiding Scope Creep

For initial development, pretending these things don't exist:

- ports (resource ports)
    - see A
- leave out cities
    - see A
- networking
- multiplayer
- anything unique to expansions
- largest army
- longest road
- development cards

A: Initially, introduce resource multiplier to get around lack of city/ports
