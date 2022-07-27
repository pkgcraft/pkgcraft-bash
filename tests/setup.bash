ALL_BUILTINS=( atom profile )
if [[ ${OSTYPE} == "darwin"* ]]; then
	OBJ_EXT="dylib"
else
	OBJ_EXT="so"
fi
SHARED_LIBRARY=${CARGO_BUILD_DIR:-./target/debug}/libpkgcraft_bash.${OBJ_EXT}

setup() {
	enable -f ${SHARED_LIBRARY} ${ALL_BUILTINS[*]}
}
