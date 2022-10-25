bats_require_minimum_version 1.7.0

# determine builtin names from test files
shopt -s failglob
ALL_BUILTINS=()
for f in ${BATS_TEST_DIRNAME}/*.bats; do
	f=${f##*/}
	ALL_BUILTINS+=( ${f%%.*} )
done
unset f

# determine pkgcraft shared library path
if [[ ${OSTYPE} == "darwin"* ]]; then
	obj_ext="dylib"
else
	obj_ext="so"
fi
SHARED_LIBRARY=${CARGO_BUILD_DIR:-./target/debug}/libpkgcraft_bash.${obj_ext}
unset obj_ext

# load builtins during test initialization
setup() {
	enable -f ${SHARED_LIBRARY} ${ALL_BUILTINS[*]}
}
