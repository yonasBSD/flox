# ============================================================================ #
#
# Patches `include/nlohmann/json.hpp' to use IWYU pragmas.
#
#
# ---------------------------------------------------------------------------- #
{nlohmann_json}:
nlohmann_json.overrideAttrs (prev: {
  postFixup = ''
    sed -i                                                                  \
        's,^\(#include \+<nlohmann/[^>]\+>\)$,\1  // IWYU pragma: export,'  \
        "$out/include/nlohmann/json.hpp";
  '';
})
# ---------------------------------------------------------------------------- #
#
#
#
# ============================================================================ #
