# ray_tracer

## A Rust script for ray path tracing and image generation

Originally inspired by _Ray Tracing in One Weekend_ (Peter Shirley) 

### How to run this program

- Clone this repository
- If `rsmake` is not already executable then run `$ chmod u+x rsmake`
- `$ ./rsmake` will do a few things :
  - compile in release mode (debug mode is useless since rendering an image takes longer than compiling), make sure that `cargo` is in your `$PATH`
  - copy the executable to the root directory of the project as `exec`
  If `rsmake` fails, one common reason is the presence of multiple executables in `./release/deps/`. The problem can be fixed by deleting `./release/` before running `rsmake` again.
- Run `./exec`
- Open the newly generated `img.ppm`

### Creating a new scene

To create and render a scene, edit the `build_world` function in `main.rs`, then run as described above. A list of objects and functions to do so can be found below.

### Creating a new complex object

It is recommended to create a new module :
- Create a new file `composite_<object>.rs`
- Implement `<object>` by providing a `build` method (more information below)
- Integrate the new object with the rest of the program

