# pkgcraft-bash

Various bash builtins leveraging [scallop](https://github.com/pkgcraft/scallop)
and [pkgcraft](https://github.com/pkgcraft/pkgcraft).

## Builtins

### Bash

- **profile**: benchmark bash functions

### Pkgcraft

- **atom**: simple atom parsing
- **atom_version**: compare package versions

## Development

Requirements: >=bash-5 and [bats](https://github.com/bats-core/bats-core) (for
testing)

Use the following commands to set up a dev environment:

```bash
# clone repo
git clone https://github.com/pkgcraft/pkgcraft-bash.git
cd pkgcraft-bash

# build with pkgcraft support enabled
cargo build --features pkgcraft

# run tests via bats
bats -r tests
```
