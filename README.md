# Event Horizon

## Overview
**Event Horizon** is an **n-body simulation game** where you control a **tiny black hole** eating other particles to increase mass. The goal is to consume surrounding masses and other players to grow larger and stronger. Similar to games like *Slither.io* and *Agar.io*.

Gameplay includes:
- **Player movement** via WASD controls  
- **Mass consumption** to increase size and gravitational strength  
- **n-body simulation mechanics**, all entities exert gravitational force on each other  

## How to Run
To run, you can just launch the game using the .exe file.

Alternatively, you can clone the entire project and in the Event-Horizon folder you can run the following commands.
```bash
cargo build
cargo run
```
If on Linux, install the required dev packages with the following command before the previously mentioned commands:
```bash
apt install libasound2-dev libudev-dev pkg-config
```

## Lessons Learned
- **WASD Movement:**  
  Movement controls was the hardest to implement/tweak. Using WASD isn't really smooth and I wish I had more knowledge be able to make it more seamless.


## Author
**John Pham**  

