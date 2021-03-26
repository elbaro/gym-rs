# Gym

```
cargo run --example pong
```

`gym-core` crate provides `GymEnv` trait. (WIP)
Each env crate provides the environment-specific interface, such as `AtariEnv`'s `render_rgb24` or `render_ram`.
They also implements `GymEnv`. You can write the training code for `GymEnv` and plug in `AtariEnv` or `OpenspielEnv`.
`gym` crate provides all these crates with feature gates, for example  `gym::AtariEnv`.

## License
Crates          | License
----------------|--------
`gym`           | MIT
`gym-core`      | MIT
`atari-env`     | GPL v2
`atari-env-sys` | GPL v2
`openspiel-env` | wip

## ROMs
The easiest way to obtain ALE-compatible ROMs is `pip install atari-py`.
You will have ROM files in `~/.local/lib/python3.x/site-packages/atari_py/atari_roms/`.
