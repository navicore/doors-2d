Doors
==========

A fun Platformer UI to Kubernetes and other data as I learn Bevy and game programming.

# UNDER CONSTRUCTION
# UNDER CONSTRUCTION

![a demo of the player jumping on platforms](docs/doors-demo-1-feb-25.gif)

install
--------

```bash
cargo install --path .
```

dev
--------

```bash
cargo run --features bevy/dynamic_linking

# for avoiding the expensive kube crate compile
cargo run --features bevy/dynamic_linking --no-default-features
```


