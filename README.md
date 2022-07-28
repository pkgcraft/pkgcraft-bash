# pkgcraft-bash

Various rust-based bash builtins leveraging scallop and pkgcraft.

## Builtins

### Bash

- **profile**: benchmark bash functions

### Pkgcraft

- **atom**: simple atom parsing

## Development

Requirements: bash, [bats](https://github.com/bats-core/bats-core) (for
testing), and everything required to build
[pkgcraft](https://github.com/pkgcraft/pkgcraft) if the related feature is
enabled

Use the following commands to set up a dev environment:

```bash
# clone the pkgcraft workspace
git clone --recurse-submodules https://github.com/pkgcraft/pkgcraft-workspace.git
cd pkgcraft-workspace

# build pkgcraft-bash with pkgcraft support enabled
cargo build --features pkgcraft -p pkgcraft-bash

# run tests via bats
bats -r pkgcraft-bash/tests
```
