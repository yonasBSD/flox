/* ========================================================================== *
 *
 * @file flox/realisepkgs/realise.hh
 *
 * @brief Evaluate an environment definition and realise it.
 *
 *
 * -------------------------------------------------------------------------- */

#pragma once

#include <map>
#include <optional>
#include <string>
#include <vector>

#include <nix/eval.hh>
#include <nix/store-api.hh>

#include "flox/core/exceptions.hh"
#include "flox/realisepkgs/realisepkgs-lockfile.hh"
#include "flox/realisepkgs/realisepkgs.hh"
#include "flox/resolver/lockfile.hh"
#include <nix/build-result.hh>
#include <nix/flake/flake.hh>
#include <nix/get-drvs.hh>
#include <nix/path-with-outputs.hh>

/* -------------------------------------------------------------------------- */

namespace flox::realisepkgs {

/* -------------------------------------------------------------------------- */

/**
 * @class flox::realisepkgs::SystemNotSupportedByLockfile
 * @brief An exception thrown when a lockfile is is missing a package.<system>
 * entry from the requested system.
 * @{
 */
FLOX_DEFINE_EXCEPTION( SystemNotSupportedByLockfile,
                       EC_LOCKFILE_INCOMPATIBLE_SYSTEM,
                       "unsupported system" )
/** @} */


/**
 * @class flox::realisepkgs::PackageUnsupportedSystem
 * @brief An exception thrown when a package fails to evaluate,
 * because the system is not supported.
 * @{
 */
FLOX_DEFINE_EXCEPTION( PackageUnsupportedSystem,
                       EC_PACKAGE_EVAL_INCOMPATIBLE_SYSTEM,
                       "system unsupported by package" )
/** @} */


/**
 * @class flox::realisepkgs::PackageEvalFailure
 * @brief An exception thrown when a package fails to evaluate.
 * @{
 */
FLOX_DEFINE_EXCEPTION( PackageEvalFailure,
                       EC_PACKAGE_EVAL_FAILURE,
                       "general package eval failure" )
/** @} */


/**
 * @class flox::realisepkgs::PackageBuildFailure
 * @brief An exception thrown when a package fails to build.
 * @{
 */
FLOX_DEFINE_EXCEPTION( PackageBuildFailure,
                       EC_PACKAGE_BUILD_FAILURE,
                       "build failure" )
/** @} */


/* -------------------------------------------------------------------------- */

/**
 * @brief Get a cursor pointing at the new attribute or @a std::nullopt. This
 *        is mostly a wrapper around
 *        @a nix::eval_cache::AttrCursor::maybeGetAttr that can't return a
 *        @a nullptr.
 * @param cursor An existing cursor.
 * @param attr The attribute to query under the cursor.
 * @return Either a known non-null reference or @a std::nullopt.
 */
std::optional<nix::ref<nix::eval_cache::AttrCursor>>
maybeGetCursor( nix::ref<nix::EvalState> &              state,
                nix::ref<nix::eval_cache::AttrCursor> & cursor,
                const std::string &                     attr );

/* -------------------------------------------------------------------------- */

/**
 * @brief Get a @a nix::eval_cache::AttrCursor pointing at the given attrpath.
 * @param state A `nix` evaluator.
 * @param flake A locked flake.
 * @param attrpath The attrpath to get in the flake.
 * @return An eval cache cursor pointing at the attrpath.
 */
nix::ref<nix::eval_cache::AttrCursor>
getPackageCursor( nix::ref<nix::EvalState> &      state,
                  const nix::flake::LockedFlake & flake,
                  const flox::AttrPath &          attrpath );


/* -------------------------------------------------------------------------- */

/**
 * @brief Get a string attribute from an attrset using the eval cache.
 * @param cursor A @a nix::eval_cache::AttrCursor.
 * @param attr The name of the attribute to get.
 * @return @a std::nullopt if the cursor doesn't point to an attrset, otherwise
 * the @a std::string representing the attribute.
 */
std::optional<std::string>
maybeGetStringAttr( nix::ref<nix::EvalState> &              state,
                    nix::ref<nix::eval_cache::AttrCursor> & cursor,
                    const std::string &                     attr );


/* -------------------------------------------------------------------------- */

/**
 * @brief Get a list of strings from an attrset using the eval cache.
 * @param cursor A @a nix::eval_cache::AttrCursor.
 * @param attr The name of the attribute to get.
 * @return The list of strings that were present under this attribute, @a
 * std::nullopt if the cursor didn't point to an attrset.
 */
std::optional<std::vector<std::string>>
maybeGetStringListAttr( nix::ref<nix::EvalState> &              state,
                        nix::ref<nix::eval_cache::AttrCursor> & cursor,
                        const std::string &                     attr );


/* -------------------------------------------------------------------------- */

/**
 * @brief Get a boolean attribute from an attrset using the eval cache.
 * @param cursor A @a nix::eval_cache::AttrCursor.
 * @param attr The name of the attribute to get.
 * @return @a std::nullopt if the cursor doesn't point to an attrset, otherwise
 * the @a std::string representing the attribute.
 */
std::optional<bool>
maybeGetBoolAttr( nix::ref<nix::EvalState> &              state,
                  nix::ref<nix::eval_cache::AttrCursor> & cursor,
                  const std::string &                     attr );

/* -------------------------------------------------------------------------- */

using OutputsOrMissingOutput
  = std::variant<std::unordered_map<std::string, std::string>, std::string>;

/**
 * @brief Uses the eval cache to query the store paths of this package's
 * outputs.
 * @param pkgCursor A @a nix::eval_cache::AttrCursor pointing at a package.
 * @param names A @a std::vector<std::string> of the output names.
 * @return A map of output names to store paths or the first missing output.
 */
OutputsOrMissingOutput
getOutputsOutpaths( nix::ref<nix::EvalState> &              state,
                    nix::ref<nix::eval_cache::AttrCursor> & pkgCursor,
                    const std::vector<std::string> &        names );


/* -------------------------------------------------------------------------- */

/**
 * @brief Catch evaluation errors for `outPath` and `drvPath` due to unfree
 * packages, etc.
 * @param state A nix evaluator.
 * @param packageName The name of the package being queried (for the error
 * message).
 * @param system The user's system type (for the error message).
 * @param pkgCursor A @a nix::eval_cache::AttrCursor pointing at a package.
 * @return The @a std::string of the requested store path
 */
std::string
tryEvaluatePackageOutPath( nix::ref<nix::EvalState> &              state,
                           const std::string &                     packageName,
                           const std::string &                     system,
                           nix::ref<nix::eval_cache::AttrCursor> & cursor );


/* -------------------------------------------------------------------------- */

/**
 * @brief Gets an @a nix::eval_cache::AttrCursor pointing at the final attribute
 * of the provided attribute path in the provided input.
 * @param state A nix evaluator.
 * @param input The locked input to look inside.
 * @param attrPath Where inside the locked input to acquire a cursor.
 * @return The cursor.
 */
nix::ref<nix::eval_cache::AttrCursor>
evalCacheCursorForInput( nix::ref<nix::EvalState> &             state,
                         const flox::resolver::LockedInputRaw & input,
                         const flox::AttrPath &                 attrPath );


/* -------------------------------------------------------------------------- */

/**
 * @brief Returns a map from output name to the corresponding outPath.
 * @param state A nix evaluator.
 * @param packageName The package whose outputs we're processing.
 * @param pkgCursor A @a nix::eval_cache::AttrCursor pointing at the package
 * (e.g. `legacyPackages.<system>.foo`).
 * @return The output-to-storePath mapping.
 */
std::unordered_map<std::string, std::string>
outpathsForPackageOutputs( nix::ref<nix::EvalState> &              state,
                           const std::string &                     packageName,
                           nix::ref<nix::eval_cache::AttrCursor> & pkgCursor );


/* -------------------------------------------------------------------------- */

/**
 * @brief Given a map containing all of a package's outputs to install,
 *        collect a list of those outputs as `RealisedPackage`s.
 * @param state A nix evaluator.
 * @param packageName The name of the package whose outputs are being processed.
 * @param lockedPackage The locked package from the lockfile.
 * the outPath of its individual outputs).
 * @param outputsToOutpaths A mapping from output name to outPath for that
 * output.
 * @return The list of packages generated from the locked package.
 */
std::vector<std::pair<realisepkgs::RealisedPackage, nix::StorePath>>
collectRealisedOutputs(
  nix::ref<nix::EvalState> &                     state,
  const std::string &                            packageName,
  const flox::resolver::LockedPackageRaw &       lockedPackage,
  std::unordered_map<std::string, std::string> & outputsToOutpaths );


/* -------------------------------------------------------------------------- */

/**
 * @brief Throws an exception if the package doesn't adhere to the current allow
 * rules.
 * @param state A nix evaluator.
 * @param packageName The name of the package being evaluated.
 * @param allows The user-specific allow rules.
 * @return Returns whether the package was unfree, as this has implications for
 * whether the package is cached.
 */
bool
ensurePackageIsAllowed( nix::ref<nix::EvalState> &              state,
                        nix::ref<nix::eval_cache::AttrCursor> & cursor,
                        const std::string &                     packageName,
                        const flox::resolver::Options::Allows & allows );


/* -------------------------------------------------------------------------- */

/**
 * @brief Collects and builds a list of realised outputs from a locked package
 * in the lockfile.
 * @param state A nix evaluator.
 * @param packageName The name of the package whose outputs are being processed.
 * @param lockedPackage The locked package from the lockfile.
 * @param system The current system.
 * @return The list of packages generated from the locked package.
 */
std::vector<std::pair<realisepkgs::RealisedPackage, nix::StorePath>>
getRealisedOutputs( nix::ref<nix::EvalState> &         state,
                    const std::string &                packageName,
                    const resolver::LockedPackageRaw & lockedPackage );


/* -------------------------------------------------------------------------- */

/**
 * Evaluate an environment definition and realise it.
 * @param state A `nix` evaluator.
 * @param lockfile a resolved and locked manifest.
 * @param system system to build the environment for.
 * @return EXIT_SUCCESS if the packages were realised successfully.
 */
std::vector<RealisedPackage>
realiseFloxEnvPackages( nix::ref<nix::EvalState> & state,
                        const nlohmann::json &     lockfile,
                        const System &             system );


/* -------------------------------------------------------------------------- */

}  // namespace flox::realisepkgs


/* -------------------------------------------------------------------------- *
 *
 *
 *
 * ========================================================================== */
