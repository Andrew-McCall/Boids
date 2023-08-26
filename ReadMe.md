# Boids

## Description
### Boid Simulation
A boid simulation is a simulation of a flock of birds. Each bird is called a boid. Each boid has a set of rules that it follows and calculates. 

These rules are:
- Separation: Boids try to keep a small distance away from other objects (including other boids).
- Alignment: Boids try to align themselves with nearby boids.
- Cohesion: Boids try to move towards the center of mass of nearby boids.

There are other rules and variations of these rules, but these are the basic rules. I made add more in the future .

Source: [Wikipedia](https://en.wikipedia.org/wiki/Boids)

### My implementation
I used Rust, WGpu and Winit. My goal was to learn a lower level graphics API and to further cement my knowledge of Rust. With WGpu and Winit my code should run on every platform that supports Vulkan, Metal or DX12. (I have only tested it on Windows 10.) 

## Showcase
Here is are some videos I made of the program running (Uploaded to Imgur). 

The first is boids without rules, just a slightly drift the to left. [Imgur Link One](https://i.imgur.com/cFLFrEi.mp4)

The second is with rules. [Imgur Link Two](https://i.imgur.com/GmyLm3g.mp4)

The third is the current state of the program with rules and tweaked settings. [Imgur Link Three](https://i.imgur.com/sBDxgYi.mp4)

Short Gif of the current state.

![Gif](https://github.com/Andrew-McCall/Boids/blob/main/assets/ShortGif.gif?raw=true)

## Improvements
- [ ] Multi-threading
- [ ] Quad-Tree / Sin and Cos Caching (Other Optimization)
- [ ] Add more rules
- [ ] Add a GUI / Menu
- [ ] 3D Refactor

## How to run / build
Cargo is required to build the project. You can download it [here](https://www.rust-lang.org/tools/install).

`cargo run --release`

`cargo build --release --target=[Your system]`

*WASM is not supported currently although the libraries do support it

## License
MIT License

