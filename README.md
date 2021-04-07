# Gym-rs

A collection of RL envs and interfaces.
- Does not bind to Python like other RL crates. You can run faster and parallel computations.
- Provides an unified interface to various envs. Verify your algorithm on classic envs and apply to your domain without code change.

Example of running envs in separate threads without vectorized envs:
![](pongpong.png)

Crates             | Role
-------------------|-----------------------------------------------------------------------
`gyms`             | main crate including all sub-crates
`gym-core`         | provides `GymEnv`, `ContinuousEnv`, `DiscreteEnv`, etc
`atari-env`        | [ALE](https://github.com/mgbellemare/Arcade-Learning-Environment) envs
`openspiel-env`    | (TODO) [OpenSpiel](https://github.com/deepmind/open_spiel) envs
`retro-env`        | (TODO) [libretro](https://www.libretro.com/) envs
`deepmind-lab-env` | (TODO) [DeepMind Lab](https://github.com/deepmind/lab) envs

Unrelated crates: `gym`, `gym-rs`

## Example
```
cargo run --example pong
cargo run --example pongpong
cargo run --example gym (requires the environmental variable `ATARI_ROMS_DIR`)
cargo run --example ppo (WIP)
```

## Feature gates

Crates          | Default | Available
----------------|---------|------------
`gyms`          | `atari` | `openspiel`
`gym-core`      |         |
`atari-env`     |         | `sdl`
`atari-env-sys` |         |
`openspiel-env` |         |

Example: `gyms = {version = "*", features = ["atari", "openspiel", "atari_env/sdl"]} `

## ROMs
The easiest way to obtain ALE-compatible ROMs is `pip install atari-py`.
You will have ROM files in `~/.local/lib/python3.x/site-packages/atari_py/atari_roms/`.

## License
Crates             | License
-------------------|--------
`gyms`             | MIT
`gym-core`         | MIT
`atari-env`        | GPL v2
`atari-env-sys`    | GPL v2
`openspiel-env`    | MIT
`retro-env`        | MIT
`deepmind-lab-env` | GPL v2
