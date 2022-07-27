load setup

@test "test atom failure" {
	run atom cat/pkg-1
	[[ "$status" -eq 1 ]]
	[[ "${lines[0]}" == 'atom: error: parsing failure: invalid atom: "cat/pkg-1"' ]]
}

@test "test atom simple" {
	atom =cat/pkg-1
	[[ ${ATOM[category]} == cat ]]
	[[ ${ATOM[package]} == pkg ]]
	[[ ${ATOM[version]} == 1 ]]
}

@test "test atom complex" {
	atom '=cat/pkg-1-r2:3/4[a,b,c]'
	[[ ${ATOM[category]} == cat ]]
	[[ ${ATOM[package]} == pkg ]]
	[[ ${ATOM[version]} == 1-r2 ]]
	[[ ${ATOM[revision]} == 2 ]]
	[[ ${ATOM[slot]} == 3 ]]
	[[ ${ATOM[subslot]} == 4 ]]
	[[ ${ATOM[use]} == "a b c" ]]
}
