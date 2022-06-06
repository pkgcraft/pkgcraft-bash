# pkgcraft-bash

Various rust-based bash plugins leveraging scallop and pkgcraft.

## Plugins

### Bash

- **profile**: benchmark bash functions

### Pkgcraft

- **atom**: simple atom parsing

## Development

Developing pkgcraft-bash assumes that recent versions of meson and bash are
installed along with a C compiler and the regular rust dev environment.

Note that bash must be built with plugin support enabled.

To build pkgcraft-bash, run the following commands:

```bash
git clone --recurse-submodules https://github.com/pkgcraft/scallop.git
git clone https://github.com/pkgcraft/pkgcraft.git
git clone https://github.com/pkgcraft/pkgcraft-bash.git

# build the plugins with pkgcraft support enabled
cd pkgcraft-bash
cargo build --features pkgcraft

# allow the linker to find the scallop library
export LD_LIBRARY_PATH=target/debug/meson
# load the plugins and profile the atom plugin
bash -c "enable -f ./target/debug/libpkgcraft_bash.so atom profile && profile atom '=cat/pkg-1-r2:3/4'"
```
