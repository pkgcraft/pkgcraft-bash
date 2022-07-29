# pkgcraft-bash

Various rust-based bash builtins leveraging scallop and pkgcraft.

## Builtins

### Bash

- **profile**: benchmark bash functions

### Pkgcraft

- **atom**: simple atom parsing

## Development

Requirements: >=bash-5, [bats](https://github.com/bats-core/bats-core) (for
testing), and everything required to build
[pkgcraft](https://github.com/pkgcraft/pkgcraft) if the related feature is
enabled

Use the following commands to set up a dev environment:

```bash
# clone the pkgcraft workspace and pull the latest project updates
git clone --recurse-submodules https://github.com/pkgcraft/pkgcraft-workspace.git
cd pkgcraft-workspace
git submodule update --recursive --remote

# build pkgcraft-bash with pkgcraft support enabled
cargo build --features pkgcraft -p pkgcraft-bash

# run tests via bats
bats -r pkgcraft-bash/tests
```
