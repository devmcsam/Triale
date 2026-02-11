# Triale

## Important Notes

- This project is pretty much a joke, but I also found when I was making it that it would actually be a good example of
  both overengineering and of good design patterns. With this scope of a project you could get away with much
  less code and probably more efficient methods.
- I’ve also, since that realization, tried to make the project modular and readable, so if someone wants to look at
  this as a reference, in my opinion, it is not a bad example.

- - -

## Installation

- Even though this is a joke project, it could still be useful to someone.

### Pre-requisites

- The rust compiler (preferably cargo)
- A terminal

### Notes

- Inside Cargo.toml, currently the flags for release are set so that it can get maximum optimization possible, if for
  some reason you would like fewer optimizations, you can disable them at will.
- To make a profileable build, you must either create a new cargo profile and select your target flags or
  modify an existing one.

### Git (Only way)

1. Clone the repositor: `git clone https://github.com/devmcsam/Triale.git`
2. run cargo install --path. --force

- - -

## Usage

once installed and in your path, run 'triale'

### Interface

The interface is pretty straightforward, enter coordinates in the cartesian plane in the format of 'x,y' and press
enter. After
repeating three times, it will compute the triangle's summary, and you will get the information.

- - -

## AI Usage

I feel that I should be honest about what I used AI for in this project. It is documented in the source code, but I will
also say here.

1. I used AI to implement the Display trait for the TriangleSummary struct initially, and then I reviewed it and fixed
   it
   to my standards.
2. I also used AI for 2 of the tests, 1 in the geometry.rs file and the other in the point.rs file.

- - -

## License

This project is licensed under the MIT license.