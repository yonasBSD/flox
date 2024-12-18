/* ========================================================================== *
 *
 * @file flox/realisepkgs/realisepkgs-lockfile.hh
 *
 * @brief The subset of a lockfile that realisepkgs needs in order to build an
 *        environment.
 *
 * -------------------------------------------------------------------------- */

#pragma once

#include "flox/core/types.hh"
#include "flox/resolver/lockfile.hh"
#include "flox/resolver/manifest-raw.hh"

/* -------------------------------------------------------------------------- */

namespace flox::realisepkgs {

/* -------------------------------------------------------------------------- */


/** @brief A mapping of output name to outpath. */
typedef std::unordered_map<std::string, std::string> OutputsToOutpaths;

/** @brief The components of a package that realisepkgs needs to realise it. */
struct RealisepkgsLockedPackage
{
  std::string system;
  std::string installId;
  // TODO: `storePath` is technically mutually exclusive
  // with `input` and `attrPath`
  // but we hope this wont be around for much longer to warrant the effort
  // of representing this as a variant.
  // <https://github.com/flox/flox/issues/2423>
  std::optional<std::string> storePath;

  // TODO: this could probably just be attrs
  resolver::LockedInputRaw           input;
  AttrPath                           attrPath;
  unsigned                           priority;
  std::shared_ptr<OutputsToOutpaths> outputsToOutpaths;
};


/* -------------------------------------------------------------------------- */

struct RealisepkgsLockfile
{
  // TODO: we don't need the packages inside the manifest
  resolver::ManifestRaw                 manifest;
  std::vector<RealisepkgsLockedPackage> packages;

  /** @brief Loads a JSON object to @a flox::realisepkgs::RealisepkgsLockfile
   *
   * The JSON object can be either a V0 or V1 lockfile, which is read from the
   * `lockfile-version` field.
   *
   * Differences between different types of descriptors are handled here:
   * - attr_path is defaulted
   * - inputs are transformed to flox-nixpkgs inputs
   * */
  void
  load_from_content( const nlohmann::json & jfrom );

  /** @brief Helper to convert a JSON object to a
   *         @a flox::realisepkgs::RealisepkgsLockfile assuming the content is a
   * V0 lockfile.
   * */
  void
  from_v0_content( const nlohmann::json & jfrom );

  /** @brief Helper to convert a JSON object to a
   *         @a flox::realisepkgs::RealisepkgsLockfile assuming the content is a
   * V1 lockfile.
   * */
  void
  from_v1_content( const nlohmann::json & jfrom );
};


/* -------------------------------------------------------------------------- */

}  // namespace flox::realisepkgs


/* -------------------------------------------------------------------------- *
 *
 *
 *
 * ========================================================================== */
