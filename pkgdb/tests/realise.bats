#! /usr/bin/env bats
# --------------------------------------------------------------------------- #
#
# @file tests/realise.bats
#
# @brief Test building environments from lockfiles.
#
# Relies on lockfiles generated by `pkgdb` from flox manifests.
#
# These tests only check the build segment,
# they do not check the resolution of manifests,
# nor the activation of the resulting environments.
# Such tests are found in `pkgdb` and `flox` respectively.
#
#
# --------------------------------------------------------------------------- #
#
# TODO: Allow a path to a file to be passed.
#
#
# --------------------------------------------------------------------------- #

# bats file_tags=build-env

load setup_suite.bash

# --------------------------------------------------------------------------- #

setup_file() {
  : "${CAT:=cat}"
  : "${TEST:=test}"
  : "${MKDIR:=mkdir}"
  export CAT TEST MKDIR
  export LOCKFILES="$BATS_TEST_DIRNAME/data/realise/lockfiles"

  # Always use a consistent `nixpkgs' input.
  export _PKGDB_GA_REGISTRY_REF_OR_REV="${NIXPKGS_REV?}"
}

# ---------------------------------------------------------------------------- #

# bats test_tags=single,smoke
@test "Simple environment realises packages successfully" {
  run "$PKGDB_BIN" realise "$LOCKFILES/single-package/manifest.lock"
  assert_success
}

# bats test_tags=single,smoke
@test "Inline JSON realises packages successfully" {
  run "$PKGDB_BIN" realise "$(< "$LOCKFILES/single-package/manifest.lock")"
  assert_success
}

# ---------------------------------------------------------------------------- #


# ---------------------------------------------------------------------------- #

# bats test_tags=single,binaries
@test "Realised packages containing binaries for v0 lock" {
  run "$PKGDB_BIN" realise \
    "$LOCKFILES/single-package/manifest.lock"
  assert_success
  # The [1] realised package for this environment is the hello package.
  assert_equal "${#lines[@]}" 1 # 1 result
  store_path="${lines[0]}"
  assert "$TEST" -x "${store_path}/bin/vim"
}

# ---------------------------------------------------------------------------- #

# bats test_tags=single,binaries
@test "Realised packages containing binaries for v1 catalog package" {
  run --separate-stderr "$PKGDB_BIN" realise \
    "$GENERATED_DATA/envs/hello/manifest.lock"
  assert_success
  # The [1] realised package for this environment is the hello package.
  assert_equal "${#lines[@]}" 1 # 1 result
  store_path="${lines[0]}"
  assert "$TEST" -x "${store_path}/bin/hello"
}


# ---------------------------------------------------------------------------- #

# bats test_tags=single,binaries
@test "Realised packages containing binaries for v1 flake package" {
  run --separate-stderr "$PKGDB_BIN" realise \
    "${TESTS_DIR?}"/data/realise/manual-lockfiles/flake/manifest.lock
  assert_success
  # The [1] realised package for this environment is the hello package.
  assert_equal "${#lines[@]}" 1 # 1 result
  store_path="${lines[0]}"
  assert "$TEST" -x "${store_path}/bin/hello"
}


# ---------------------------------------------------------------------------- #
#
#
#
# ============================================================================ #