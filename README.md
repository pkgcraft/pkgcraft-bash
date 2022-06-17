# pkgcraft-bash

Various rust-based bash builtins leveraging scallop and pkgcraft.

## Builtins

### Bash

- **profile**: benchmark bash functions

### Pkgcraft

- **atom**: simple atom parsing

## Development

Requirements: meson, bash, and everything required to build
[pkgcraft](https://github.com/pkgcraft/pkgcraft)

Use the following commands to set up a dev environment:

```bash
# clone the pkgcraft workspace
git clone --recursive-submodules https://github.com/pkgcraft/pkgcraft-workspace.git
cd pkgcraft-workspace

# build shared library for scallop and set shell variables (e.g. $LD_LIBRARY_PATH)
source ./build scallop

# build pkgcraft-bash with pkgcraft support enabled
cargo build --features pkgcraft -p pkgcraft-bash

# load the builtins and profile the atom builtin
bash -c "enable -f ./target/debug/libpkgcraft_bash.so atom profile && profile atom '=cat/pkg-1-r2:3/4'"
```
