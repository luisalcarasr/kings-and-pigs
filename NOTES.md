# Development Notes

> These are my personal development notes that I've decided to share publicly. This is not a tutorial, but rather a record of my learning journey with Bevy.

## Getting Started

My first steps with Bevy involved installing the framework and importing all available modules through the `prelude`. While this might be considered a poor practice in a larger project, it's a practical starting point for learning and experimentation.

**Adding the dependency:**

```sh
cargo add bevy
```

**Importing the prelude:**

```rust
use bevy::prelude::*;
```

## The Default Plugin

A plugin is a mechanism for modularizing a Bevy application. Through a plugin, you can register components, systems, entities, resources, and much more within the application. Bevy provides several built-in plugins; we use `DefaultPlugins`, which includes everything necessary for developing a 2D game. While it does include more than we strictly need—including support for 3D—it provides a solid foundation.

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

## Understanding ECS Fundamentals

Bevy is built on the Entity-Component-System (ECS) architecture. To work effectively with Bevy, you must understand these core concepts:

- **Components**: Data structures that store information associated with an entity. A component is simply a Rust struct that you define.
- **Systems**: Functions that operate on entities and their components, reading and modifying their state. Systems are the logic of your application.
- **Entities**: Unique objects in your game world that aggregate components together. An entity is identified by a unique ID and serves as a container for components.
- **Resources**: Global application data that isn't associated with a specific entity, but is shared across the entire application.

The distinction between components and resources is conceptual: use components for data that belongs to specific entities, and resources for data you need to access globally throughout your application.

## Configuring ImagePlugin

To preserve the visual quality of pixel art, we configured `ImagePlugin` with `default_nearest()`. This adjustment is crucial because it prevents the bilinear magnification filter from blurring textures when they are scaled. This is especially important in pixel art games where the clarity of each individual pixel is fundamental to the visual presentation.

```rust
.add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
```

## The Player Plugin (PlayerPlugin)

We created a dedicated module for the player in `src/player.rs` that implements the `Plugin` trait. This modular structure allows us to encapsulate all logic related to the player character in a single, cohesive location:

- **`setup_player` system** (runs at startup): Creates the player entity, loads its textures, and configures its animation.
- **`animate_sprite` system** (runs each frame): Updates the texture atlas index to produce the illusion of movement.

```rust
pub struct PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_player);
        app.add_systems(Update, animate_sprite);
    }
}
```

## The Sprite Animation System

To animate the player character, we implemented a system based on two components:

- **`AnimationIndices`**: A component that stores the first and last frame indices of the animation sequence. In our case, the idle animation has 11 frames (indices 0 through 10).
- **`AnimationTimer`**: A component that inherits from `Timer` using `Deref` and `DerefMut`. It tracks the elapsed time between frame changes and is configured with a 0.1-second interval in repeating mode.

The `animate_sprite` system queries all entities possessing these components. When the timer elapses, the system advances to the next frame in the atlas. When it reaches the final frame, it wraps back to the first frame, creating a smooth, looping animation cycle.

## Asset Loading and TextureAtlas

Bevy provides a robust system for managing game assets. We leveraged this system as follows:

- The `AssetServer` loads the player character texture from `characters/playables/king-human/idle.png`.
- We create a `TextureAtlasLayout` that defines a grid of 11 columns × 1 row, where each cell contains a sprite measuring 78 × 58 pixels.
- The sprite is scaled by a factor of 2.0 to improve visibility on screen.

This approach allows us to use a single image containing multiple frames for animation, rather than maintaining separate files for each individual frame. This is both efficient and maintainable.

## Camera Setup

For Bevy to render the 2D scene correctly, at least one camera must exist in the world. In the `setup_camera` system, we spawn a `Camera2d` entity:

```rust
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```

Without this camera, the rendering engine would have no perspective from which to draw the game world. 
