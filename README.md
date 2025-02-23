Doors
==========

A Platformer Game UI to [Kubernetes](https://kubernetes.io/) and other data as I
learn [Rust](https://www.rust-lang.org) [Bevy game
programming](https://github.com/bevyengine/bevy).

------------
# UNDER CONSTRUCTION - Alpha Alpha
------------

My interest in this learning exercise is exploring the [ECS - Entity Component
System](https://en.wikipedia.org/wiki/Entity_component_system) programming
paradigm. Game programming can teach us a lot about software architectures that
coordinate huge numbers of lively objects all advancing their state at different
rates.

The world in this game is a graph of rooms connected by doors.  Every door is on
a platform.  The player moves left and right with arrow keys and jumps with the
space bar.  Once on a platform, the player enters the next room through the door
by using the up arrow.

| key | description | action |
| --- | --- | --- |
| -> | right arrow | run to the right |
| <- | left arrow | run to the left |
| sp | space bar | jump |
| ^ | up arrow | enter a room |
| q | 'q' key| end the game |

The rooms and their connections via doors are a directed graph.  The world of
doors and rooms is built from external data converted into triples stored in a
graph implemented by the [petgraph](https://github.com/petgraph/petgraph) crate.

The initial world is generated from your live Kubernetes cluster.  Make sure you
have access to a cluster via [kubectl](https://kubernetes.io/docs/reference/kubectl/) and `doors` will use the same
authentication to query your cluster and build the game world using [kube-rs
crate](https://github.com/kube-rs/kube) API results.  Rooms are `namespaces`,
`deployments`, `replicasets`, `pods`, and `containers` - all connected by doors.

The graph naturally uses `nodes` as rooms and `edges` as doors.  It decides to put a
door between two rooms based on their relationship found in the Kubernetes API
results (normally seen as yaml by devops engineers).

Roadmap
==========

![a demo of the player jumping on platforms](docs/doors-demo-1-feb-25.gif)

* proper player graphics and animation
* animate the doors
* read RDF for world generation
* sound
* swag and power-ups
* multi-player
* re-implement in isometric style
* re-re-implement in 3D

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


