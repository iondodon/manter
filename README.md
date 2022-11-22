[![Contributor Covenant](https://img.shields.io/badge/Contributor%20Covenant-2.1-4baaaa.svg)](https://github.com/iondodon/manter/blob/main/CODE_OF_CONDUCT.md)
[![Contribution rules](https://img.shields.io/badge/Contribution%20rules-1.0-green)](https://github.com/iondodon/manter/blob/main/CONTRIBUTING.md)


# Manter

This project aims to create a terminal with functionalities that improve the experience of using terminals. Examples of useful functionalities would be autocomplete suggestion, showing the current branch of a project, prepared scripts that can be reused, and others. You can come up with your ideas. The project uses technologies such as Svelte on the front-end, which offers flexibility regarding the implementation of the interface. On the back-end side, the most importatn part is the PTY. All the back end is implemented in Rust. The basic framework of the project is Tauri.

A good analogy to understand the purpose of this project is the following - traditional terminals are like simple code editors while this terminal is like an IDE.

Anyone is welcome to contribute to this project with and ideas. Imagine the terminal you would like to use. Feel free to open any issue with suggestions and bugs.

## Community

[Discord server](https://discord.gg/k4FFFPK3ZR)

## Contribution

[Contribution guide](https://github.com/iondodon/manter/blob/main/CONTRIBUTING.md)

## Setup local development environment

*First off, thank you for considering contributing to Manter. It’s people like you that can make Manter a great tool.*

1. install the latest LTS NodeJS

2. install the latest LTS Rust

2. clone the repository

3. install the dependencies ```npm install```

4. run the application in dev environment ```npm run tauri dev```

You can keep the application running in the dev environment and the changes in code will be reflected in the application running application. If the back-end code changes, then the application restarts to reflect the changes.

Build the installer with ```npm run tauri build```.

The installer that has been built will be for the operating system that you are using. The terminal can be built for Linux, MacOS and Windows.
