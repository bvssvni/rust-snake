rust-snake - or - Sea Snake Escape
==========

A sea snake game in Rust using the Piston game engine  
To run it you need <a href="http://www.glfw.org/" target="_blank">GLFW 3.0.+</a>  
OSX binary: <a href="http://www.cutoutpro.com/sea-snake-osx.zip" target="_blank">Download</a>  

| Dependency | Online Docs |
|---------|------|------------|
| [piston](https://github.com/bvssvni/piston) | [piston docs](http://bvssvni.github.io/docs/piston/piston/) |
| [rust-graphics](https://github.com/bvssvni/rust-graphics) | [rust-graphics docs](http://bvssvni.github.io/docs/rust-graphics/graphics/) |
| [glfw-rs](https://github.com/bjz/glfw-rs) | [glfw-rs docs](http://bvssvni.github.io/docs/glfw-rs/glfw/) |
| [rust-opengles](https://github.com/mozilla-servo/rust-opengles) | [rust-opengles docs](http://bvssvni.github.io/docs/rust-opengles/opengles/) |

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
