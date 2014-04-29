rust-snake - or - Sea Snake Escape
==========

A sea snake game in Rust using the Piston game engine  
To run it you need <a href="http://www.glfw.org/" target="_blank">GLFW 3.0.+</a>  
OSX binary: <a href="http://www.cutoutpro.com/sea-snake-osx.zip" target="_blank">Download</a>  

## How to play

Use the arrow keys to control the diver.  
When you loose or win, hit `Enter` to restart.  
You can also exit with `Esc`.  

![alt tag](https://raw.githubusercontent.com/bvssvni/rust-snake/master/sea-snake.png)

*Reach the surface before the sea snakes get you!*

This project serves as a test project for the [Rust-Graphics](https://github.com/bvssvni/rust-graphics) API and [Piston](https://github.com/bvssvni/piston) game engine. The main goal is to improve Rust-Graphics. Do not expect it to have high quality as a game.

It was also my entry game for <a href="http://www.ludumdare.com/compo/ludum-dare-29/?action=preview&uid=19918" target="_blank">Ludum Dare 29</a>

## Building Instructions

Add the following '.rlib' files to '/target/cpu-vendor-os/lib/':

* libglfw-38369174-0.1.rlib (https://github.com/bvssvni/glfw-rs)
* libgraphics-587c2edd-0.0.rlib (https://github.com/bvssvni/rust-graphics)
* libopengles-73387c6a-0.1.rlib (https://github.com/bvssvni/rust-opengles)
* libpiston-a1b791b5-0.0.rlib (https://github.com/bvssvni/piston)

If you have trouble building with Rust nightly, try the original repos. There are currently many breaking changes in Rust, so please help the maintainers to keep them up with master!  

In the Terminal window, navigate to the project folder and type:

```
make run
```
