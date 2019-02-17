# Carambolage
![logo small](other/logo-small.png "Carambolage") [![Build Status](https://api.travis-ci.com/K4ugummi/carambolage.svg?branch=master)](https://travis-ci.com/K4ugummi/carambolage) [![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
---

## Contents
1. [License](#license)
2. [Download](#download)
3. [Build](#build-yourself)
3. [Contributing](#contributing)
4. [Just saying thank you](#a-huge-thanks-to)

## A game written in Rust!

[![screenshot](https://media.githubusercontent.com/media/K4ugummi/carambolage/master/other/screen-ingame.PNG "Gameplay video on Youtube")](https://youtu.be/sDVB_pLf2us)
(Click on this screenshot to be redirected to a gameplay video on Youtube)

## Controls
![controls](https://media.githubusercontent.com/media/K4ugummi/carambolage/master/other/controls.jpg "Controls")
Open Menu: `[Esc]`  

**Player 1**  
movement:  `[W]`,`[A]`,`[S]`,`[D]`  
boost: `[Left Shift]`  

**Player 2**  
movement: `[Up]`,`[Left]`,`[Down]`,`[Right]`  
boost: `[Right Shift]`  

---

## License
Carambolage is licensed under [GNU General Public License v3](/LICENSE)

---

## Download
No precompiled binaries yet ¯\\\_(ツ)\_/¯

## Build yourself
1. Download and install the [Rust-lang compiler](https://www.rust-lang.org/).
2. You have to setup [GLFW](https://www.glfw.org/) on your system in order to build the game, because it handles a the OpenGL context window for your platform.
3. Clone this repository `git clone https://github.com/K4ugummi/carambolage.git`
4. `cd carambolage`
5. Build and run the game `cargo run --release`

---

## Contributing
1. Fork the project
2. Create a new branch `git checkout -b <branch>`
3. Make changes
4. Run `cargo fmt`
5. Run `cargo clippy` and make sure it compiles wihtout warnings.
6. Push your changes to your fork and create a pull request

---

## A huge thanks to
- [The Rust Team](https://www.rust-lang.org/en-US/team.html)
- [Joey de Vries](https://joeydevries.com/) Author of learnopengl.com
- **YOU** for spending your time reading this readme, playing the game or even contributing to the project!
