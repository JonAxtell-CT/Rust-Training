#!/usr/bin/env bats

@test 'Check that the mysql client is available' {
    command -v mysql
}

