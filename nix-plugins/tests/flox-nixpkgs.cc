/* ========================================================================== *
 *
 *
 *
 * -------------------------------------------------------------------------- */

#include <fstream>
#include <iostream>

#include <nix/cmd/command.hh>
#include <nix/expr/eval-gc.hh>
#include <nix/expr/eval.hh>
#include <nix/expr/search-path.hh>
#include <nix/fetchers/attrs.hh>
#include <nix/main/shared.hh>
#include <nix/store/store-api.hh>
#include <nix/util/url.hh>
#include <nlohmann/json.hpp>

#include "flox/fetchers/wrapped-nixpkgs-input.hh"
#include "test.hh"


/* -------------------------------------------------------------------------- */

using namespace flox;


/* -------------------------------------------------------------------------- */

/** @brief Test a flox-nixpkgs URL can be parsed and then serialized. */
bool
test_URLRoundtrip( nix::ref<nix::EvalState> & state )
{
  WrappedNixpkgsInputScheme inputScheme;
  auto                      url = "flox-nixpkgs:v0/flox/" + nixpkgsRev;
  auto input = inputScheme.inputFromURL( state->fetchSettings,
                                         nix::parseURL( url ),
                                         true );
  EXPECT( input.has_value() );
  EXPECT_EQ( inputScheme.toURL( *input ).to_string(), url );
  return true;
}


/* -------------------------------------------------------------------------- */

/**
 * @brief Test a flox-nixpkgs input can be created from attrs and then has the
 *        expected URL.
 **/
bool
test_inputFromAttrs( nix::ref<nix::EvalState> & state )
{
  nix::fetchers::Attrs      attrs = { { "version", (uint64_t) 0 },
                                      { "type", "flox-nixpkgs" },
                                      { "owner", "NixOS" },
                                      { "rev", nixpkgsRev } };
  WrappedNixpkgsInputScheme inputScheme;
  auto                      url = "flox-nixpkgs:v0/NixOS/" + nixpkgsRev;
  auto input = inputScheme.inputFromAttrs( state->fetchSettings, attrs );
  EXPECT( input.has_value() );
  EXPECT_EQ( inputScheme.toURL( *input ).to_string(), url );
  return true;
}


/* -------------------------------------------------------------------------- */

bool
test_lockedFromUrl( nix::ref<nix::EvalState> & state )
{
  WrappedNixpkgsInputScheme inputScheme;
  auto                      url = "flox-nixpkgs:v0/flox/" + nixpkgsRev;
  auto                 input = inputScheme.inputFromURL( state->fetchSettings,
                                         nix::parseURL( url ),
                                         true );
  nix::fetchers::Attrs attrs
    = inputScheme.getAccessor( state->store, *input ).second.toAttrs();
  auto owner      = nix::fetchers::getStrAttr( attrs, "owner" );
  auto flake_type = nix::fetchers::getStrAttr( attrs, "type" );
  auto rev        = nix::fetchers::getStrAttr( attrs, "rev" );
  auto version    = nix::fetchers::getIntAttr( attrs, "version" );
  EXPECT_EQ( owner, "flox" );
  EXPECT_EQ( flake_type, "flox-nixpkgs" );
  EXPECT_EQ( rev, nixpkgsRev );
  EXPECT_EQ( version, (uint64_t) 0 );

  return true;
}

/* -------------------------------------------------------------------------- */

/**
 * @brief Test a locked flox-nixpkgs input preserves all information in the
 *        unlocked attrs.
 **/
bool
test_lockedRepresentation( nix::ref<nix::EvalState> & state )
{
  nix::fetchers::Attrs      attrs = { { "version", (uint64_t) 0 },
                                      { "type", "flox-nixpkgs" },
                                      { "owner", "NixOS" },
                                      { "rev", nixpkgsRev } };
  WrappedNixpkgsInputScheme inputScheme;
  auto                      url = "flox-nixpkgs:v0/NixOS/" + nixpkgsRev;
  auto input = inputScheme.inputFromAttrs( state->fetchSettings, attrs );
  EXPECT( input.has_value() );
  auto locked = inputScheme.getAccessor( state->store, *input ).second;
  EXPECT( locked.toAttrs() == attrs );
  return true;
}


/* -------------------------------------------------------------------------- */

int
main()
{
  int exitCode = EXIT_SUCCESS;
  // NOLINTNEXTLINE(cppcoreguidelines-macro-usage)
#define RUN_TEST( ... ) _RUN_TEST( exitCode, __VA_ARGS__ )

  /* Initialize `nix' */
  nix::initNix();
  nix::initGC();
  auto store = nix::openStore();
  auto state = nix::make_ref<nix::EvalState>( nix::LookupPath(),
                                              store,
                                              nix::fetchSettings,
                                              nix::evalSettings );

  RUN_TEST( URLRoundtrip, state );
  RUN_TEST( inputFromAttrs, state );
  RUN_TEST( lockedFromUrl, state );
  RUN_TEST( lockedRepresentation, state );

  return exitCode;
}


/* -------------------------------------------------------------------------- *
 *
 *
 *
 * ========================================================================== */
