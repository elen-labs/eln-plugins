# elf-plugin

Plugin API platform for the [elendirna](https://github.com/elen-labs/elendirna) project.

This crate defines the *interface contract* between elendirna core and plugin modules — function call shape, hook points, permissions.

## Status

Scaffold. Trait definitions are deferred pending design decisions tracked in the elendirna workbench vault.

## Layout

- `elf-plugin` (this crate) — interface contract
- `elen-labs/elendirna` — core elendirna (host)
- `elen-labs/plugins/{plugin_name}` — concrete plugin modules

## License

MIT
