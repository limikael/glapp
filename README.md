# glapp
OpenGL meta window and context creation.

The window and graphics context creation pipeline in the Rust ecosystem is experiencing a kind of cambrian explosion at the moment, 
which is why it is good to be flexible possible when it comes to working with different underlying libraries and toolchains.

I created Glapp as a kind of "meta library", wrapping different underlying libraries, each with their respective strength depending
on platform and situation.

The goal is that the code you write should be 100% transferrable from one platform to another. No need to mangle or name your main function differently
depending on platform, glapp takes care of that. It should also be as easy as possible to get up and running. A simple example looks like:

```rust
use glapp::*;

#[glapp_main]
fn main(mut app:App) {
    app.title("Testing Glapp");
    app.run(|window,event|{
        match event {
            AppEvent::Open=>{
                // The OpenGL context is created and made current.
            },
            AppEvent::Render=>{
                // This is where we render our scene.
            },
            // ... see docs for more events, e.g. input and such ...
        }
    });
}
```

## Libraries
Glapp can work with the following underlying libraries. Enable them as features:

- **SDL** - Simple DirectMedia Layer is a cross-platform development library designed to provide low level access to audio, 
  keyboard, mouse, joystick, and graphics hardware via OpenGL and Direct3D. SDL officially supports Windows, macOS, Linux, iOS, and Android.
  SDL has been around since 1998 and is thoroughly battle tested. 
- **Glutin** - Glutin is a pure Rust library with deep roots in the community.
