# shellcheck shell=bash

# ============================================================================ #
#
# Setup Rust
#
# ---------------------------------------------------------------------------- #

# Only run if 'rustLibSrc' is in the environment
if [[ -d "$FLOX_ENV/rustc-std-workspace-std" ]]; then
  export RUST_SRC_PATH="$FLOX_ENV"
fi

# ---------------------------------------------------------------------------- #
#
#
#
# ============================================================================ #
