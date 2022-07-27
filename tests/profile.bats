load setup

@test "test profile" {
	run profile sleep 1
	[[ "$status" -eq 0 ]]
	[[ "${lines[0]}" == "profiling: sleep 1" ]]
}
