bats_require_minimum_version 1.7.0

ALL_BUILTINS=( atom atom_version profile )
if [[ ${OSTYPE} == "darwin"* ]]; then
	OBJ_EXT="dylib"
else
	OBJ_EXT="so"
fi
SHARED_LIBRARY=${CARGO_BUILD_DIR:-./target/debug}/libpkgcraft_bash.${OBJ_EXT}

setup() {
	enable -f ${SHARED_LIBRARY} ${ALL_BUILTINS[*]}
}
