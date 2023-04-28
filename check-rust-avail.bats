#!/usr/bin/env bats

@test 'Check that Rust is available' {
    command -v rustup
}