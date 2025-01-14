# shellcheck shell=bash
# shellcheck disable=SC2154

_sed="@gnused@/bin/sed"

# N.B. the output of
# these scripts may be eval'd with backticks which have the effect of removing
# newlines from the output, so we must ensure that the output is a valid shell
# script fragment when represented on a single line.
generate_bash_startup_commands() {
  "$_flox_activate_tracer" "generate_bash_startup_commands()" START

  _flox_activate_tracelevel="${1?}"
  shift
  _FLOX_ACTIVATION_STATE_DIR="${1?}"
  shift
  _activate_d="${1?}"
  shift
  FLOX_ENV="${1?}"
  shift
  _FLOX_ACTIVATION_PROFILE_ONLY="${1?}"
  shift

  if [ "$_flox_activate_tracelevel" -ge 2 ]; then
    echo "set -x;"
  fi

  if [ "${_FLOX_ACTIVATION_PROFILE_ONLY:-}" != true ]; then
    # TODO: should we skip this for in-place activations?
    # We use --rcfile to activate using bash which skips sourcing ~/.bashrc,
    # so source that here, but not if we're already in the process of sourcing
    # bashrc in a parent process.
    if [ -f ~/.bashrc ] && [ -z "${_flox_already_sourcing_bashrc:=}" ]; then
      echo "export _flox_already_sourcing_bashrc=1;"
      echo "source ~/.bashrc;"
      echo "unset _flox_already_sourcing_bashrc;"
    fi

    # Restore environment variables set in the previous bash initialization.
    $_sed -e 's/^/unset /' -e 's/$/;/' "$_FLOX_ACTIVATION_STATE_DIR/del.env"
    $_sed -e 's/^/export /' -e 's/$/;/' "$_FLOX_ACTIVATION_STATE_DIR/add.env"
  fi

  # Propagate $_activate_d to the environment.
  echo "export _activate_d='$_activate_d';"
  # Propagate $_flox_activate_tracer to the environment.
  echo "export _flox_activate_tracer='$_flox_activate_tracer';"
  # Propagate $_flox_env_helper to the environment.
  echo "export _flox_env_helper='$_flox_env_helper';"

  # Set the prompt if we're in an interactive shell.
  echo "if [ -t 1 ]; then source '$_activate_d/set-prompt.bash'; fi;"

  # We already customized the PATH and MANPATH, but the user and system
  # dotfiles may have changed them, so finish by doing this again.
  echo "eval \"\$($_flox_env_helper bash)\";"

  # Source user-specified profile scripts if they exist.
  for i in profile-common profile-bash hook-script; do
    if [ -e "$FLOX_ENV/activate.d/$i" ]; then
      "$_flox_activate_tracer" "$FLOX_ENV/activate.d/$i" START
      echo "source '$FLOX_ENV/activate.d/$i';"
      "$_flox_activate_tracer" "$FLOX_ENV/activate.d/$i" END
    else
      "$_flox_activate_tracer" "$FLOX_ENV/activate.d/$i" NOT FOUND
    fi
  done

  # Disable command hashing to allow for newly installed flox packages
  # to be found immediately. We do this as the very last thing because
  # python venv activations can otherwise return nonzero return codes
  # when attempting to invoke `hash -r`.
  echo "set +h;"

  if [ "$_flox_activate_tracelevel" -ge 2 ]; then
    echo "set +x;"
  fi

  "$_flox_activate_tracer" "generate_bash_startup_commands()" END
}
