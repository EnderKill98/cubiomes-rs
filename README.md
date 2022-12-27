# cubiomes-rs

This is a pretty basic port of the c library [cubiomes](https://github.com/Cubitect/cubiomes/) to rust.

This repo contains two crates: libcubiomes-sys and cubiomes

The sys-crate (libcubiomes-sys) build and generates the bindings to the c library. Nothing more, nothing less.

The cubiomes crate here, adds a few basic abstractions over it. For now it can query biomes and use the biome cache.

**WARNING**: The BiomeFinder and BiomeCache have not properly been tested, yet! Do test them first, as they may still contain bugs or report invalid biomes!

## How to use

Simply add this to your Cargo.toml to use the crates:

```toml
cubiomes = { git = "https://github.com/EnderKill98/cubiomes-rs", rev = "RECENT_COMMIT_SHA1_HERE" }
libcubiomes-sys = { git = "https://github.com/EnderKill98/cubiomes-rs", rev = "RECENT_COMMIT_SHA1_HERE" }
```

Note: While cubiomes includes a link to the sys-crate as `cubiomes::sys` your ide may not auto-complete the contents unless you addes the sys-crate (2nd line) as well.

This project is a cargo workspace. When you use `name = { git = "...", ... }` it will automatically use the correct folder based on the name you specified. This is because Rust implicitly assumes this to be `name = { package = "name", ... }` which automatically selects the package name for you. You can use the `package` option as seen in the example to alter what library name equals what workspace member / crate here.

As of now, the cubiomes crate has only implemented checking a biome. If you don't need this functionality, just use the sys-crate. **Contributions** to cubiomes (abstractions over libcubiomes-sys) are welcome.

## FAQ

### Why is it not on crates.io?

This project is properly a one-off and might not be maintained properly in the future. I would hate to claim the name `cubiomes` on crates.io when someone could do a far better implementation and claim the name instead.

While this projects uses the same license as [cubiomes](https://github.com/Cubitect/cubiomes/), I'm also not certain about how the included lib / author should be properly credited, yet.

If people want this published, I'd rather only publish the sys-crate though as the cubiomes lib not even implemented a fraction of the library and should therefore be considered an alpha. I also don't plan on changing this anytime soon.

Even when publishing the sys-crate, I'm not sure that I'd be up to updating the module regularily, so the lib on crates.io would probably become outdates in a few months or year from now (Dec 2022).
