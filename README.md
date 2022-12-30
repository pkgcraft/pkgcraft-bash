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

Requirements: >=bash-5, [bats](https://github.com/bats-core/bats-core) (for
testing), and everything required to build
[pkgcraft](https://github.com/pkgcraft/pkgcraft) if the related feature is
enabled

Use the following commands to set up a dev environment:

```bash
# clone the git repo
git clone https://github.com/pkgcraft/pkgcraft-bash.git
cd pkgcraft-bash

# build pkgcraft-bash with pkgcraft support enabled
cargo build --features pkgcraft

# run tests via bats
bats -r pkgcraft-bash/tests
```
