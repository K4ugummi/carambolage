# Carambolage
![logo small](other/logo-small.png "Carambolage") [![Build Status](https://api.travis-ci.com/K4ugummi/carambolage.svg?branch=master)](https://travis-ci.com/K4ugummi/carambolage) [![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
---

## Contents
1. [License](#license)
2. [Download](#download)
3. [Build](#build)
3. [Contributing](#contributing)
4. [Just saying thank you](#a-huge-thanks-to)

## A game written in Rust!

![screenshot](https://media.githubusercontent.com/media/K4ugummi/carambolage/master/other/screen-ingame.PNG "Screenshot")

We are big fans of Rust and have been using this programming language in our working environment for some time now. After getting used to the fact that Rust doesn't let us get away with stupid things that crash our code at runtime (at least that's how we see it at the moment), we've become attached to this modern language. 

Sadly, you're always busy learning good patterns during your work, so you have to think about and rethink sooo much, and the fun is a little bit left behind. That's why we decided to write a little game in Rust. This is all about the fun of programming (if you found this project on GitHub, you can probably understand that). But that also means that we don't think so much about what we're typing down now.

---

## Controls
![controls](https://media.githubusercontent.com/media/K4ugummi/carambolage/master/other/controls.jpg "Controls")
Open Menu: `[Esc]`  

Player 1  
movement:  `[W]`,`[A]`,`[S]`,`[D]`  
boost: `[Left Shift]`  

Player 2  
movement: `[Up]`,`[Left]`,`[Down]`,`[Right]`  
boost: `[Right Shift]`  

---

## License
[GNU General Public License v3](/LICENSE)

---

## Download
You can download precompiled binaries for your system below:
- [Windows v0.1](https://carambolage.k4ugummi.de/wp-content/uploads/2018/10/carambolage-win-v0_1.zip)
- [Linux v0.1](https://carambolage.k4ugummi.de/wp-content/uploads/2018/10/carambolage-linux-v0_1.tar.gz)
- ~~MacOS~~

## Build
First, you have to download and install the [Rust-lang compiler](https://www.rust-lang.org/).

You have to setup [GLFW](https://www.glfw.org/) on your system in order to build the game, because it handles a the OpenGL context window for your platform.

---

## Contributing
1. Fork the project
2. Create a new branch `git checkout -b <branch>`
3. Make changes
4. Run `cargo fmt`
5. Run `cargo clippy` and make sure it compiles wihtout warnings.
6. Push your changes to your fork and create a pull request

### Dependencies
- [GLFW](https://github.com/PistonDevelopers/glfw-rs)
- OpenGL 3.3 compatible hardware


## A huge thanks to
- [The Rust Team](https://www.rust-lang.org/en-US/team.html)
- [Joey de Vries](https://joeydevries.com/) Author of learnopengl.com
- **YOU** for spending your time reading this readme, playing the game or even contributing to the project!
