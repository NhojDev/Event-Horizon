# Event Horizon

## Overview
**Event Horizon** is an **n-body simulation game** where you control a **tiny black hole** eating other particles to increase mass. The goal is to absorb surrounding masses to grow bigger. Similar to games like *Slither.io* and *Agar.io*.

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

## Issues Encountered during Development
- **Online Resources**  
  Finding online resources for all the different game engines using Rust proved to be somewhat difficult. Most documentation for certain game engines were already outdated due to the constant updates that each game engine went through. Using GGEZ was fairly easy though.

- **Using the Particular Crate**  
  Initially the project was going to be implemented using the Particular crate, but it had a lot of performance and combabilties issues to that idea was scrapped halfway during development.

- **Initial Physics issues**  
  When switching over to the new O(n^2) physics algorthim, I had to change some of the logic to include padding and softening so the particles won't cause a physics explosion when crash into each other.

## AI Usage
AI was used during the development of the game. However, it was mainly used to quickly look up resources and suggestions in GGEZ and Rust. Other usage include error message clarity and fixes.
## Author
**John Pham**  

