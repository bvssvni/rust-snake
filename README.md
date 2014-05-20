rust-snake - or - Sea Snake Escape
==========

A sea snake game in Rust using the Piston game engine
To run it you need <a href="http://www.libsdl.org/" target="_blank">SDL2</a>

| Dependency | Online Docs |
|---------|------|------------|
| [piston](https://github.com/bvssvni/piston) | [piston docs](http://pistondevelopers.github.io/docs/piston/piston/) |
| [rust-graphics](https://github.com/bvssvni/rust-graphics) | [rust-graphics docs](http://pistondevelopers.github.io/docs/rust-graphics/graphics/) |
[rust-sdl2](https://github.com/AngryLawyer/rust-sdl2) | [rust-sdl2 docs](http://pistondevelopers.github.io/docs/rust-sdl2/sdl2/) |
| [rust-opengles](https://github.com/mozilla-servo/rust-opengles) | [rust-opengles docs](http://pistondevelopers.github.io/docs/rust-opengles/opengles/) |

## How to play

Use the arrow keys to control the diver.
When you loose or win, hit `Enter` to restart.
You can also exit with `Esc`.

![alt tag](https://raw.githubusercontent.com/bvssvni/rust-snake/master/sea-snake.png)

*Reach the surface before the sea snakes get you!*

This project serves as a test project for the [Rust-Graphics](https://github.com/bvssvni/rust-graphics) API and [Piston](https://github.com/bvssvni/piston) game engine. The main goal is to improve Rust-Graphics. Do not expect it to have high quality as a game.

It was also my entry game for <a href="http://www.ludumdare.com/compo/ludum-dare-29/?action=preview&uid=19918" target="_blank">Ludum Dare 29</a>

## Building Instructions

Add the dependencies and add the '.rlib' files to '/target/cpu-vendor-os/lib/':

In the Terminal window, navigate to the project folder and type:

```
make run
```
