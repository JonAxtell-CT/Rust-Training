#!/usr/bin/env bats

@test 'Check that we have a /tmp directory' {
	run stat /tmp
	[ $status = 0 ]
}
