load setup

@test "test atom_version failures" {
	# no args
	run -1 atom_version
	[[ "${lines[0]}" =~ ^"atom_version: error: requires 1 arg" ]]

	# too many args
	run -1 atom_version 1.2.3 ">" 1.2.1
	[[ "${lines[0]}" =~ ^"atom_version: error: requires 1 arg" ]]

	# incorrect number of substrings in arg string
	run -1 atom_version "1.2.3 <"
	[[ "${lines[0]}" =~ ^"atom_version: error: invalid argument format" ]]

	# invalid versions
	run -1 atom_version "1.2.a < 1.2.4"
	[[ "${lines[0]}" == 'atom_version: error: parsing failure: invalid version: "1.2.a"' ]]
	run -1 atom_version "1.2.3 < 1.2.b"
	[[ "${lines[0]}" == 'atom_version: error: parsing failure: invalid version: "1.2.b"' ]]

	# invalid operator
	run -1 atom_version "1.2.3 -gt 1.2.1"
	[[ "${lines[0]}" == 'atom_version: error: invalid operator: -gt' ]]
}

@test "test atom_version comparisons" {
	# successes
	run -0 atom_version "1.2.3 < 1.2.4"
	run -0 atom_version "1.2.3 <= 1.2.4"
	run -0 atom_version "1.2.3 == 1.2.3"
	run -0 atom_version "1.2.3 != 1.2.4"
	run -0 atom_version "1.2.4 >= 1.2.3"
	run -0 atom_version "1.2.4 > 1.2.3"

	# failures
	run -1 atom_version "1.2.4 < 1.2.3"
	run -1 atom_version "1.2.4 <= 1.2.3"
	run -1 atom_version "1.2.4 == 1.2.3"
	run -1 atom_version "1.2.4 != 1.2.4"
	run -1 atom_version "1.2.3 >= 1.2.4"
	run -1 atom_version "1.2.3 > 1.2.4"
}
