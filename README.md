# Driving Algorithm
I'm trying to implement an algorithm *without* resorting to machine learning and neural networks, *just math*.

## Current behaviour
Cars are able to drive a constant speed and follow a straight or circular line.

![Demonstrative animation](https://github.com/alexesmet/driving-algorithm/blob/main/demo.gif)

### Plans:
- [ ] lower car speed before turns to increase available steering speed
- [ ] avoid crashes

## Physics
Algorithm can't change speed or steering directly, it only has access to accelleration, brakes, and steering wheel. Maximum speed, accelleration, steering wheel speed are all capped and controlled by physics engine. Steering is a bit simplified: it is expressed as a number of degrees car turns when it drives one pixel forward.

## Choice of the instruments
 - [rust](https://www.rust-lang.org/) is a programming language I'm willing to master
 - [nannou](https://github.com/nannou-org/nannou) library was used to implement simplest graphical representation as fast as possible, in order to focus on the algorithm.
 - [toml](https://github.com/toml-rs/toml) was used to read road structure and initial driving situation. I wanted a short syntax, but didn't want to implement my own file.
