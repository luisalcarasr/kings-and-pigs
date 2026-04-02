# Development Notes

> Esto no es un tutorial, nos notas personales que he decidido hacer publicas. 

## Primeros pasos
Mis primero pasos en Bevy ha sido instalar e importar TODO, quiza esto es una mala practica
pero por ahora creo que me sirve asi.

**Agregar dependencia:**
```sh
cargo add bevy
```

**Importar todo**
```rust
use bevy::prelude::*;
```

## El plugin por defecto
Un plugin no es mas que una forma de modularizar la app, en un plugin puedes
agregar componentes, systemas, entidades, recursos, etc. Bevy tiene un par de plugins por defecto, nosotros usaremos el DefalutPlugin que contiene todo lo que necesitamos para hacer un juego 2D, quiza es un poco overkill porque creo que contiene tambien las preparaciones para 3D. 

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}
```

El Quick Start de Bevy explica bien lo que es ECS (Entities-Components-Systems), pero burdamente los componentes son las estructuras que contienen datos, los sistemas son fuciones que las manejan. Aun no entiendo las entidades muy bien pero creo que estos usan los componentes. Tambien estan los recursos, que es como informacion global, aun no entiendo bien la diferencia entre componentes y recursos. 
