# Overview

`micrrou` is a Rust crate that aims to wrap and simplify [`nannou`](https://nannou.cc).  While `nannou` has good interfaces on its own, `micrrou` aims to combine common use cases to reduce boilerplate code.

The other objective of `micrrou` is to provide some algorithms that hopefully helps in generating art through code.

## Examples

Examples are provided in the `examples` directory.

Creating a blank canvas example is shown below.

```rust
use std::slice::Iter;

use micrrou::prelude::*;

struct MyModel {
    drawings: Vec<Box<dyn Drawable>>,
    frame_count: usize,
}

impl Model for MyModel {
    fn create() -> Self {
        Self {
            drawings: Vec::new(),
            frame_count: 0,
        }
    }

    fn get_drawings<'a>(&'a self) -> Iter<'a, Box<dyn Drawable>> {
        self.drawings.iter()
    }

    fn update(&mut self) {
        self.frame_count += 1;
        println!("frame count: {}", self.frame_count);
    }
}

pub fn main() {
    nannou_app::launch::<MyModel, 900, 900>();
}
```

Here, we have the `nannou_app::launch` function create an empty blank canvas using `MyModel` as the model with a 900x900 pixel canvas.

## `macrrou`

`maccrou` is an optional procedural macro alternative to setup a `nannou_app`.

```rust
use std::slice::Iter;

use micrrou::launch_nannou_app;
use micrrou::{nannou_app::Model, prelude::Drawable};

struct ModelData {
    drawings: Vec<Box<dyn Drawable>>,
}

impl Default for ModelData {
    fn default() -> Self {
        Self {
            drawings: Vec::new(),
        }
    }
}

impl Model for ModelData {
    fn create() -> Self {
        Self::default()
    }

    fn get_drawings(&self) -> Iter<'_, Box<dyn Drawable>> {
        self.drawings.iter()
    }

    fn update(&mut self) {}
}

pub fn main() {
    launch_nannou_app!(for ModelData, canvas(width=900, height=900));
}
```

Here, we make use of the `launch_nannou_app!` macro using `ModelData` as the `Model` with a 900x900 pixel canvas.
