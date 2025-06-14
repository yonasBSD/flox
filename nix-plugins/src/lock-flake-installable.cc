/* ========================================================================== *
 *
 * @file lock-flake-installable.hh
 *
 * @brief Executable command helper and `flox::lockFlakeInstallable`.
 *
 *
 * -------------------------------------------------------------------------- */

#include <fstream>

#include <nix/cmd/installable-flake.hh>
#include <nix/expr/attr-path.hh>
#include <nix/expr/eval-cache.hh>
#include <nix/expr/eval.hh>
#include <nix/expr/json-to-value.hh>
#include <nix/expr/primops.hh>
#include <nix/expr/value-to-json.hh>
#include <nix/flake/flake.hh>
#include <nix/util/json-utils.hh>
#include <nlohmann/json.hpp>

#include "flox/core/util.hh"
#include "flox/lock-flake-installable.hh"


/* -------------------------------------------------------------------------- */

namespace flox {

/* -------------------------------------------------------------------------- */

/**
 * @brief Parse the installable string into a flake reference, fragment and
 * extended outputs spec.
 * @param state The nix evaluation state
 * @param installableStr The installable string
 * @return A tuple containing the flake reference, fragment and extended outputs
 * @throws LockFlakeInstallableException if the installable string could not be
 * parsed
 */
static std::tuple<nix::FlakeRef, std::string, nix::ExtendedOutputsSpec>
parseInstallable( const nix::ref<nix::EvalState> & state,
                  const std::string &              installableStr )
{
  try
    {
      return nix::parseFlakeRefWithFragmentAndExtendedOutputsSpec(
        state->fetchSettings,
        installableStr,
        nix::absPath( std::string_view( "." ) ) );
    }
  catch ( const nix::Error & e )
    {
      throw nix::Error( "could not parse installable: %s", e.what() );
    }
}

/**
 * @brief Locate the installable in the flake and return a locked installable.
 * Locks the referenced flake if necessary, but does not apply updates
 * or writes any local state.
 * @param state The nix evaluation state
 * @param flakeRef The flake reference
 * @param fragment The attrpath fragment e.g. everything right of the `#` in a
 * flake installable (excluding output specifiers)
 * @param extendedOutputsSpec The outputs specified with `^<outputs>` in a flake
 * installable
 * @return A locked @a nix::InstallableFlake
 * @throws @a LockFlakeInstallableException if the installable could not be
 * located or the flakeref could not be locked
 */
static nix::ref<nix::eval_cache::AttrCursor>
getDerivationCursor( nix::EvalState &        state,
                     nix::InstallableFlake & installable )
{
  try
    {
      auto cursor = installable.getCursor( state );
      return cursor;
    }
  catch ( const nix::Error & e )
    {
      throw nix::Error( "could not locate derivation for installable: %s",
                        e.what() );
    }
}

/**
 * @brief Lock the flake referenced by the installable.
 * @note This function ensures that potential nix exceptions are caught and
 * rethrown as @a LockFlakeInstallableException.
 * @param installable The installable to lock
 * @return A locked flake
 * @throws LockFlakeInstallableException if the flake could not be locked
 */
static std::shared_ptr<nix::flake::LockedFlake>
getLockedFlake( nix::InstallableFlake & installable )
{
  try
    {
      return installable.getLockedFlake();
    }
  catch ( const nix::Error & e )
    {
      debugLog( nix::fmt( "error locking flake: %s", e.what() ) );
      throw;
    }
}

/**
 * @brief Read a license string or id from a nix value.
 * @note The license can be either a string or an attribute set with a `spdxId`
 * if `<nixpkgs>.lib.licenses.<license>` is used.
 * @param state The nix evaluation state
 * @param licenseValue The value to read the license from
 * @return The license string or id if found or `std::nullopt` otherwise
 */
static std::optional<std::string>
readLicenseStringOrId( nix::EvalState & state, nix::Value * licenseValue )
{
  if ( licenseValue->type() == nix::ValueType::nString )
    {
      return std::string( licenseValue->string_view() );
    }
  else if ( licenseValue->type() == nix::ValueType::nAttrs )
    {
      auto licenseIdValue
        = licenseValue->attrs()->find( state.symbols.create( "spdxId" ) );

      if ( licenseIdValue != licenseValue->attrs()->end()
           && licenseIdValue->value->type() == nix::ValueType::nString )
        {
          return std::string( licenseIdValue->value->string_view() );
        };
    }

  return std::nullopt;
}

LockedInstallable
lockFlakeInstallable( const nix::ref<nix::EvalState> & state,
                      const std::string &              system,
                      const std::string &              installableStr )
{
  debugLog( nix::fmt( "original installable: %s", installableStr ) );

  auto parsed = parseInstallable( state, installableStr );

  nix::FlakeRef            flakeRef            = std::get<0>( parsed );
  std::string              fragment            = std::get<1>( parsed );
  nix::ExtendedOutputsSpec extendedOutputsSpec = std::get<2>( parsed );

  debugLog(
    nix::fmt( "original flakeRef: '%s'", flakeRef.to_string().c_str() ) );
  debugLog( nix::fmt( "original fragment: '%s'", fragment ) );
  debugLog( nix::fmt( "original extendedOutputsSpec: '%s'",
                      extendedOutputsSpec.to_string() ) );

  auto lockFlags = nix::flake::LockFlags {
    .recreateLockFile      = false,
    .updateLockFile        = false,
    .writeLockFile         = false,
    .useRegistries         = false,
    .applyNixConfig        = false,
    .allowUnlocked         = true,
    .commitLockFile        = false,
    .referenceLockFilePath = std::nullopt,
    .outputLockFilePath    = std::nullopt,
    .inputOverrides = std::map<nix::flake::InputAttrPath, nix::FlakeRef> {},
    .inputUpdates   = std::set<nix::flake::InputAttrPath> {}
  };


  nix::InstallableFlake installable = nix::InstallableFlake(
    // The `cmd` argument is only used in nix to raise an error
    // if `--arg` was used in the same command.
    // The argument is never stored on the `InstallableFlake` struct
    // or referenced outside of the constructor.
    // We can safely pass a nullptr here, as the constructor performs a null
    // check before dereferencing the argument:
    // <https://github.com/NixOS/nix/blob/509be0e77aacd8afcf419526620994cbbbe3708a/src/libcmd/installable-flake.cc#L86-L87>
    static_cast<nix::SourceExprCommand *>( nullptr ),
    state,
    std::move( flakeRef ),
    fragment,
    extendedOutputsSpec,
    // Defaults from nix:
    // <https://github.com/NixOS/nix/blob/142e566adbce587a5ed97d1648a26352f0608ec5/src/libcmd/installables.cc#L231>
    nix::Strings {
      "packages." + system + ".default",
      "defaultPackage." + system,
    },
    // Defaults from nix:
    // <https://github.com/NixOS/nix/blob/142e566adbce587a5ed97d1648a26352f0608ec5/src/libcmd/installables.cc#L236>
    nix::Strings {
      "packages." + system + ".",
      "legacyPackages." + system + ".",
    },
    lockFlags );


  debugLog(
    nix::fmt( "original installable: '%s'", installable.what().c_str() ) );

  auto lockedFlake = getLockedFlake( installable );

  auto lockedUrl = lockedFlake->flake.lockedRef.to_string();

  debugLog( nix::fmt( "locked url: '%s'", lockedUrl ) );

  auto flakeDescription = lockedFlake->flake.description;

  auto cursor = getDerivationCursor( *state, installable );

  auto lockedAttrPath = cursor->getAttrPathStr();
  debugLog( nix::fmt( "locked attr path: '%s'", lockedAttrPath ) );

  debugLog( nix::fmt( "locked outputs: '%s'",
                      installable.extendedOutputsSpec.to_string() ) );

  // check if the output is a derivation (not a just a store path)
  if ( ! cursor->isDerivation() )
    {
      auto v = cursor->forceValue();
      throw nix::EvalError(
        *state,
        "expected flake output attribute '%s' to be a derivation, but found %s",
        lockedAttrPath,
        nix::showType( v ) );
    }

  // read the drv path
  std::string derivation;
  {
    auto derivationCursor
      = cursor->findAlongAttrPath( nix::parseAttrPath( *state, "drvPath" ) );
    if ( ! derivationCursor )
      {
        throw nix::EvalError( *state,
                              "could not find '%s.%s' in derivation",
                              lockedAttrPath,
                              "drvPath" );
      }
    derivation = ( *derivationCursor )->getStringWithContext().first;
  }

  // map output names to their store paths
  std::map<std::string, std::string> outputs;
  std::vector<std::string>           outputNames;
  {
    auto maybe_outputs_cursor
      = cursor->findAlongAttrPath( nix::parseAttrPath( *state, "outputs" ) );
    if ( ! maybe_outputs_cursor )
      {
        throw nix::EvalError( *state,
                              nix::fmt( "could not find '%s.%s' in derivation",
                                        lockedAttrPath,
                                        "outputs" ) );
      }
    outputNames = ( *maybe_outputs_cursor )->getListOfStrings();

    for ( auto output : outputNames )
      {
        auto outputCursor = cursor->findAlongAttrPath(
          nix::parseAttrPath( *state, output + ".outPath" ) );
        if ( ! outputCursor )
          {
            throw nix::EvalError( *state,
                                  "could not find '%s.%s' in derivation",
                                  lockedAttrPath,
                                  output + ".outPath" );
          }
        auto outputValue = ( *outputCursor )->getStringWithContext();
        outputs[output]  = outputValue.first;
      }
  }

  // try read `meta.outputsToInstall` field
  std::optional<std::set<std::string>> outputsToInstall;
  {
    std::set<std::string> outputsToInstallFound;
    auto metaOutputsToInstallCursor = cursor->findAlongAttrPath(
      nix::parseAttrPath( *state, "meta.outputsToInstall" ) );
    if ( metaOutputsToInstallCursor )
      {
        for ( auto output :
              ( *metaOutputsToInstallCursor )->getListOfStrings() )
          {
            outputsToInstallFound.insert( output );
          }

        outputsToInstall = outputsToInstallFound;
      }
  }

  // the requested outputs to install by means of the extended outputs spec
  // i.e. `#^<outputs>` in the flake installable
  std::optional<std::set<std::string>> requestedOutputs;
  {
    requestedOutputs = std::visit(
      overloaded {
        [&]( const nix::ExtendedOutputsSpec::Default & )
          -> std::optional<std::set<std::string>> { return std::nullopt; },
        [&]( const nix::ExtendedOutputsSpec::Explicit & e )
          -> std::optional<std::set<std::string>>
        {
          return std::visit(
            overloaded {
              [&]( const nix::OutputsSpec::Names & n ) -> std::set<std::string>
              { return n; },
              [&]( const nix::OutputsSpec::All & ) -> std::set<std::string>
              {
                std::set<std::string> outputNamesSet;
                for ( auto output : outputNames )
                  {
                    outputNamesSet.insert( output );
                  }
                return outputNamesSet;
              } },
            e.raw );
        },
      },
      extendedOutputsSpec.raw );
  }

  std::string systemAttribute;
  {
    auto systemCursor
      = cursor->findAlongAttrPath( nix::parseAttrPath( *state, "system" ) );

    if ( ! systemCursor )
      {
        throw nix::EvalError( *state,
                              "could not find '%s.%s' in derivation",
                              lockedAttrPath,
                              "system" );
      }
    systemAttribute = ( *systemCursor )->getString();
  }

  // Read `name` field - field is implied by the derivation
  std::string name;
  {
    auto nameCursor
      = cursor->findAlongAttrPath( nix::parseAttrPath( *state, "name" ) );

    if ( ! nameCursor )
      {
        throw nix::EvalError( *state,
                              "could not find '%s.%s' in derivation",
                              lockedAttrPath,
                              "name" );
      }
    name = ( *nameCursor )->getString();
  }

  // Read `pname` field
  std::optional<std::string> pname;
  {
    auto pnameCursor
      = cursor->findAlongAttrPath( nix::parseAttrPath( *state, "pname" ) );

    if ( pnameCursor ) { pname = ( *pnameCursor )->getString(); }
  }

  // Read `version` field
  std::optional<std::string> version;
  {
    auto versionCursor
      = cursor->findAlongAttrPath( nix::parseAttrPath( *state, "version" ) );

    if ( versionCursor ) { version = ( *versionCursor )->getString(); }
  }

  // Read `meta.description` field
  std::optional<std::string> description;
  {
    auto descriptionCursor
      = cursor->findAlongAttrPath( { state->sMeta, state->sDescription } );

    if ( descriptionCursor )
      {
        description = ( *descriptionCursor )->getString();
      }
  }

  std::optional<std::vector<std::string>> licenses;
  {
    auto licenseCursor = cursor->findAlongAttrPath(
      nix::parseAttrPath( *state, "meta.license" ) );

    if ( licenseCursor )
      {
        auto licenseValue = ( *licenseCursor )->forceValue();
        std::vector<std::string> licenseStrings;
        if ( licenseValue.isList() )
          {
            for ( auto licenseValueInner : licenseValue.listItems() )
              {
                state->forceValueDeep( *licenseValueInner );
                if ( auto licenseString
                     = readLicenseStringOrId( *state, licenseValueInner ) )
                  {
                    licenseStrings.push_back( *licenseString );
                  }
              }
          }
        else if ( auto licenseString
                  = readLicenseStringOrId( *state, &licenseValue ) )
          {
            licenseStrings.push_back( *licenseString );
          }
        if ( ! licenseStrings.empty() ) { licenses = licenseStrings; }
      }
  }

  std::optional<bool> broken;
  {
    auto brokenCursor = cursor->findAlongAttrPath(
      nix::parseAttrPath( *state, "meta.broken" ) );

    if ( brokenCursor ) { broken = ( *brokenCursor )->getBool(); }
  }

  std::optional<bool> unfree;
  {
    auto unfreeCursor = cursor->findAlongAttrPath(
      nix::parseAttrPath( *state, "meta.unfree" ) );

    if ( unfreeCursor ) { unfree = ( *unfreeCursor )->getBool(); }
  }

  std::optional<int64_t> priority;
  {

    auto priorityCursor = cursor->findAlongAttrPath(
      nix::parseAttrPath( *state, "meta.priority" ) );
    if ( priorityCursor ) { priority = ( *priorityCursor )->getInt().value; }
  }


  LockedInstallable lockedInstallable = {
    .lockedUrl                 = lockedUrl,
    .flakeDescription          = flakeDescription,
    .lockedFlakeAttrPath       = lockedAttrPath,
    .derivation                = derivation,
    .outputs                   = outputs,
    .outputNames               = outputNames,
    .outputsToInstall          = outputsToInstall,
    .requestedOutputsToInstall = requestedOutputs,
    .packageSystem             = systemAttribute,
    .system                    = system,
    .name                      = name,
    .pname                     = pname,
    .version                   = version,
    .description               = description,
    .licenses                  = licenses,
    .broken                    = broken,
    .unfree                    = unfree,
    .priority                  = priority,
  };

  return lockedInstallable;
}


void
to_json( nlohmann::json & jto, const LockedInstallable & from )
{
  jto = nlohmann::json {
    { "locked-url", from.lockedUrl },
    { "flake-description", from.flakeDescription },
    { "locked-flake-attr-path", from.lockedFlakeAttrPath },
    { "derivation", from.derivation },
    { "outputs", from.outputs },
    { "output-names", from.outputNames },
    { "outputs-to-install",
      from.outputsToInstall.value_or( nix::StringSet {} ) },
    { "requested-outputs-to-install",
      from.requestedOutputsToInstall.value_or( nix::StringSet {} ) },
    { "package-system", from.packageSystem },
    { "system", from.system },
    { "name", from.name },
    { "pname", from.pname },
    { "version", from.version },
    { "description", from.description },
    { "licenses", from.licenses },
    { "broken", from.broken },
    { "unfree", from.unfree },
    { "priority", from.priority }
  };
}

void
from_json( const nlohmann::json & jfrom, LockedInstallable & to )
{
  to.lockedUrl = jfrom.at( "locked-url" );
  if ( jfrom.contains( "flake-description" ) )
    {
      to.flakeDescription = jfrom.at( "flake-description" );
    };
  to.lockedFlakeAttrPath = jfrom.at( "locked-flake-attr-path" );
  to.derivation          = jfrom.at( "derivation" );
  to.outputs             = jfrom.at( "outputs" );
  if ( jfrom.contains( "outputs-to-install" ) )
    {
      to.outputsToInstall = jfrom.at( "outputs-to-install" );
    };
  if ( jfrom.contains( "requested-outputs-to-install" ) )
    {
      to.requestedOutputsToInstall = jfrom.at( "requested-outputs-to-install" );
    };
  to.packageSystem = jfrom.at( "package-system" );
  to.system        = jfrom.at( "system" );
  to.name          = jfrom.at( "name" );
  if ( jfrom.contains( "pname" ) ) { to.pname = jfrom.at( "pname" ); };
  if ( jfrom.contains( "version" ) ) { to.version = jfrom.at( "version" ); };
  if ( jfrom.contains( "description" ) )
    {
      to.description = jfrom.at( "description" );
    };
  if ( jfrom.contains( "licenses" ) ) { to.licenses = jfrom.at( "licenses" ); };
  if ( jfrom.contains( "broken" ) ) { to.broken = jfrom.at( "broken" ); };
  if ( jfrom.contains( "unfree" ) ) { to.unfree = jfrom.at( "unfree" ); };
  if ( jfrom.contains( "priority" ) ) { to.priority = jfrom.at( "priority" ); }
}


void
prim_lockFlakeInstallable( nix::EvalState &  state,
                           const nix::PosIdx pos,
                           nix::Value **     args,
                           nix::Value &      value )
{


  nix::NixStringContext context;

  if ( args[0]->isThunk() && args[0]->isTrivial() )
    {
      state.forceValue( *args[1], pos );
    }

  auto state2 = nix::make_ref<nix::EvalState>( state.getLookupPath(),
                                               state.store,
                                               state.fetchSettings,
                                               state.settings );

  auto system = nix::settings.thisSystem.get();

  auto lockedInstallable
    = lockFlakeInstallable( state2,
                            system,
                            std::string( args[0]->string_view() ) );

  auto lockedInstallableJson = nlohmann::json( lockedInstallable );
  nix::parseJSON( state, lockedInstallableJson.dump(), value );
}


static const nix::RegisterPrimOp
  primop_lockFlakeInstallable( { .name  = "__lockFlakeInstallable",
                                 .args  = { "flakeInstallable" },
                                 .arity = 0,
                                 .doc   = R"(    )",
                                 .fun   = prim_lockFlakeInstallable,
                                 .experimentalFeature = nix::Xp::Flakes } );

/* -------------------------------------------------------------------------- */

}  // namespace flox


/* -------------------------------------------------------------------------- *
 *
 *
 *
 * ========================================================================== */
