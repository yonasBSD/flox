#! /usr/bin/env bats
# -*- mode: bats; -*-
# ============================================================================ #
#
# Test of rust impl of 'flox list'
#
# Tests are tentative, missing spec!
#
# ---------------------------------------------------------------------------- #

load test_support.bash

# ---------------------------------------------------------------------------- #

# Helpers for project based tests.

project_setup() {
  export PROJECT_DIR="${BATS_TEST_TMPDIR?}/test"
  rm -rf "$PROJECT_DIR"
  mkdir -p "$PROJECT_DIR"
  pushd "$PROJECT_DIR" >/dev/null || return

  export _FLOX_USE_CATALOG_MOCK="$GENERATED_DATA/empty.yaml"
}

project_teardown() {
  popd >/dev/null || return
  rm -rf "${PROJECT_DIR?}"
  unset PROJECT_DIR
}

# ---------------------------------------------------------------------------- #

setup() {
  common_test_setup
  project_setup
}
teardown() {
  project_teardown
  common_test_teardown
}

# ---------------------------------------------------------------------------- #

init_env() {
  mkdir -p "$PROJECT_DIR/.flox/env"
  cp --no-preserve=mode "$MANUALLY_GENERATED"/empty/* "$PROJECT_DIR/.flox/env"

  echo '{
    "name": "env",
    "version": 1
  }' >>"$PROJECT_DIR/.flox/env.json"
}

# ---------------------------------------------------------------------------- #

@test "'flox list' lists packages of environment in the current dir; fails if no env found" {
  run "$FLOX_BIN" list
  assert_failure
}

@test "'flox list' lists packages of environment in the current dir; No package" {
  init_env
  run "$FLOX_BIN" list
  assert_success
}

@test "'flox list' lists packages of environment in the current dir; shows different paths" {
  init_env
  cp "$GENERATED_DATA"/envs/pip/* "$PROJECT_DIR/.flox/env"

  run "$FLOX_BIN" list
  assert_success
  assert_output --regexp - <<EOF
pip: python312Packages.pip.*
EOF
}

@test "'flox list' lists packages of environment in the current dir; shows different id" {
  init_env

  # install hello with `greeting` as the iid.
  cp "$GENERATED_DATA"/envs/hello_as_greeting/* "$PROJECT_DIR/.flox/env"

  run "$FLOX_BIN" list
  assert_success
  assert_output --regexp - <<EOF
greeting: hello \([0-9]+\.[0-9]+(\.[0-9]+)?\)
EOF
}

# ---------------------------------------------------------------------------- #

# bats test_tags=list,list:not-applicable
@test "'flox list' hides packages not installed for the current system" {
  init_env

  # Mock env with `hello` installed for all systems
  # and `htop` for no system to emulate a package not installed
  # for the current system on all systems.
  cp "$GENERATED_DATA"/envs/hello_and_htop_for_no_system/* "$PROJECT_DIR/.flox/env"

  run "$FLOX_BIN" list -n
  assert_success
  assert_output "hello"
}

# ---------------------------------------------------------------------------- #

# https://github.com/flox/flox/issues/1039
# bats test_tags=list,list:tolerates-missing-version
@test "'flox list' tolerates missing version" {
  init_env

  # `influxdb2 does not have a version attribute set in nixpkgs (2024-02-19)
  # todo: replace with a more predicatable/smaller example
  cp "$GENERATED_DATA"/envs/influxdb2/* "$PROJECT_DIR/.flox/env"

  run "$FLOX_BIN" list
  assert_success
  assert_output "influxdb2: influxdb2 (influxdb2)"
}

# bats test_tags=list
@test "'flox list' lists packages of environment in the current dir; One package from nixpkgs" {
  "$FLOX_BIN" init
  _FLOX_USE_CATALOG_MOCK="$GENERATED_DATA/resolve/hello.yaml" \
    "$FLOX_BIN" install hello

  run "$FLOX_BIN" list
  assert_success
  assert_output --regexp 'hello: hello \([0-9]+\.[0-9]+(\.[0-9]+)?\)'
}

# bats test_tags=list,list:config
@test "'flox list --config' shows manifest content" {
  "$FLOX_BIN" init
  MANIFEST_CONTENTS="$(
    cat <<-EOF
version = 1

[hook]
on-activate = "something suspicious"
EOF
  )"

  echo "$MANIFEST_CONTENTS" | "$FLOX_BIN" edit -f -

  run "$FLOX_BIN" list --config
  assert_success
  assert_output "$MANIFEST_CONTENTS"
}

# bats test_tags=list,list:config
@test "'flox list --config' shows manifest content for composed environments" {
  "$FLOX_BIN" init -d included
  cat > included/.flox/env/manifest.toml <<-EOF
version = 1

[install]
hello.pkg-path = "hello"
EOF

  "$FLOX_BIN" init -d composer
  cat > composer/.flox/env/manifest.toml <<-EOF
version = 1

[include]
environments = [
  { dir = "../included" },
]
EOF

  # Trigger a lock of included
  _FLOX_USE_CATALOG_MOCK="$GENERATED_DATA/resolve/hello.yaml" \
    "$FLOX_BIN" list -d included

  _FLOX_USE_CATALOG_MOCK="$GENERATED_DATA/resolve/hello.yaml" \
    run --separate-stderr "$FLOX_BIN" list -c -d composer
  assert_success
  # TODO: Unspecified tables and empty vecs should be omitted.
  assert_equal "$output" 'version = 1

[install]
hello.pkg-path = "hello"'
  assert_equal "$stderr" 'ℹ️  Displaying merged manifest.'
}

# bats test_tags=list,list:config
@test "'flox list --config' shows notices about overrides" {
  "$FLOX_BIN" init -d included
  "$FLOX_BIN" edit -d included -f - <<- EOF
version = 1

[vars]
foo = "included"
EOF

  "$FLOX_BIN" init -d composer
  "$FLOX_BIN" edit -d composer -f - <<- EOF
version = 1

[vars]
foo = "composer"

[include]
environments = [
  { dir = "../included" },
]
EOF

  run --separate-stderr "$FLOX_BIN" list -c -d composer
  assert_success
  assert_equal "$stderr" "ℹ️  Displaying merged manifest.
ℹ️  The following manifest fields were overridden during merging:
- This environment set:
  - vars.foo"
}
