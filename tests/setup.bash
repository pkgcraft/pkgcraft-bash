bats_require_minimum_version 1.7.0

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
	local builtin=${BATS_TEST_FILENAME##*/}
	builtin=${builtin%%.*}
	enable -f ${SHARED_LIBRARY} ${builtin}
}
