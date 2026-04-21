# Ray Tracing In a Weekend

This is a Rust learning attempt by reverse engineering the concepts described at [https://raytracing.github.io/](https://raytracing.github.io/) by Peter Shirley, Trevor D Black and Steve Hollasch.

The original tutorials are in C++ and this repo contains my best efforts to convert them into idiomatic Rust.

I prefer to implement everything myself and as a result there are many instances of manual trait implementations.

I have also tried to not use std as much as possible hoping to port this to a microcontroller.

## Build and Run
The tutorial introduces the .ppm format to get the rendered frame as output, which I think is a huge plus for beginners who like to implement all things from scratch.

With the .ppm format at hand, we can drop reliance  on graphics crates which might take away the fun of truly owning the code.

However since many IDEs not supporting .ppm format, I had to rely on ffmpeg to convert it so that I can view it inside my IDE itself.

So, the final command is as follows.
```bash
cargo run > image.ppm && ffmpeg -y -i image.ppm -compression_level 9 output.png
```