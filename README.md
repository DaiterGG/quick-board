## Quick Board (WIP)
A fast, lightweight, single executable program for sketching and drawing, written in Rust.

![qb](https://github.com/user-attachments/assets/e7f89053-c8b6-4cc4-af34-4124c2246761)

## Technical details:
* This app uses the [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2) for its GUI and drawing functionality.
* It has its own GUI ~~library~~ implementation.
* Support canvas size up to 2<sup>32</sup> x 2<sup>32</sup> pixels (by implementing infinitely sized textures).
* Draw history, that can be edited in any order.
