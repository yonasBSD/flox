#! /usr/bin/env bats
# -*- mode: bats; -*-
# ============================================================================ #
#
# `flox-nixpkgs' wrapped input tests.
#
# ---------------------------------------------------------------------------- #

load setup_suite.bash

# bats file_tags=flox-nixpkgs

# ---------------------------------------------------------------------------- #

@test "'github' fetcher does NOT set 'allowUnfree' and 'allowBroken'" {
  run --separate-stderr "$PKGDB_BIN" eval "let
    nixpkgs = builtins.getFlake \"github:NixOS/nixpkgs/$NIXPKGS_REV\";
    inherit (nixpkgs.legacyPackages.x86_64-linux) config;
  in assert ! ( config.allowUnfree || config.allowBroken ); true";
  assert_success;
  assert_output "true";
}


# ---------------------------------------------------------------------------- #

@test "'flox-nixpkgs' fetcher sets 'allowUnfree' and 'allowBroken'" {
  run --separate-stderr "$PKGDB_BIN" eval "let
    nixpkgs = builtins.getFlake
                \"flox-nixpkgs:v$FLOX_NIXPKGS_VERSION/flox/$NIXPKGS_REV\";
    inherit (nixpkgs.legacyPackages.x86_64-linux) config;
  in assert config.allowUnfree && config.allowBroken; true";
  assert_success;
  assert_output "true";
}


# ---------------------------------------------------------------------------- #

@test "'flox-nixpkgs' and 'github' fetchers fingerprints differ" {
  run --separate-stderr "$PKGDB_BIN" eval "let
    fp0 = builtins.getFingerprint
            \"flox-nixpkgs\:v$FLOX_NIXPKGS_VERSION/flox/$NIXPKGS_REV\";
    fp1 = builtins.getFingerprint \"github:NixOS/nixpkgs/$NIXPKGS_REV\";
  in assert fp0 != fp1; true";
  assert_success;
  assert_output "true";
}


# ---------------------------------------------------------------------------- #

@test "'flox-nixpkgs' and 'github' 'outPaths' match" {
  run --separate-stderr "$PKGDB_BIN" eval "let
    fp0 = builtins.getFlake
            \"flox-nixpkgs\:v$FLOX_NIXPKGS_VERSION/flox/$NIXPKGS_REV\";
    op0 = fp0.legacyPackages.x86_64-linux.hello.outPath;

    fp1 = builtins.getFlake \"github:NixOS/nixpkgs/$NIXPKGS_REV\";
    op1 = fp1.legacyPackages.x86_64-linux.hello.outPath;

  in assert op0 == op1; true";
  assert_success;
  assert_output "true";
}


# ---------------------------------------------------------------------------- #
#
#
#
# ============================================================================ #
