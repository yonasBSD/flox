{ lib }:

let

  /**
    This function collects all package attr paths
    from the recursive structure generated by `lib.nef.dirToAttrs`
    via a conditional fold operation.

    The result is a list of TargetMetadata sets.

    # Example

    ```nix
    collectAttrPaths [] { path = "<expression dir>", type = "directory"; entries = {
      foo = { path = "<expression dir>/foo.nix"; type = "nix"; };
      bar = { path = "<expression dir>/bar", type = "directory"; entries = {
          baz = { path = "<expression dir>/bar/baz.nix"; type = "nix"; };
          bam = { path = "<expression dir>/bar/bam/default.nix"; type = "nix"; };
      }; };
    };}
    =>
    [
      {
        attrPath = ["foo"];
        attrPathStr = "foo";
        relFilePath = "foo";
        absFilePath = "<expression_dir>/foo.nix";
      }
      {
        attrPath = ["bar" "baz"];
        attrPathStr = "bar.baz";
        relFilePath = "bar/baz.nix";
        absFilePath = "<expression_dir>/bar/baz.nix";
      }
      {
        attrPath = ["bar" "bam"];
        attrPathStr = "bar.bam";
        relFilePath = "bar/bam/default.nix";
        absFilePath = "<expression_dir>/bar/bam/default.nix";
      }
    ]
    ```

    # Type

    ```
    collectAttrPaths :: [String] -> Attrs -> [ TargetMetadata ]

    TargetMetadata :: {
      attrPath :: [ String ],
      attrPathStr :: String,
      absFilePath :: Path,
      relFilePath :: Path,
    }
    ```

    # Arguments

    attrsFromDirTop
    : An attribute set created from `lib.nef.dirToAttrs`
      It is expected that the top level is of `type = "directory"`

    :::
  */
  collectAttrPaths =
    attrsFromDirTop:
    let
      do =
        _prefix@{ attrPath }:
        attrsFromDir:
        {
          "nix" = {
            inherit attrPath;
            attrPathStr = lib.showAttrPath attrPath;
            absFilePath = attrsFromDir.path;
            relFilePath = lib.removePrefix "${attrsFromDirTop.path}/" attrsFromDir.path;
          };
          "directory" = lib.flatten (
            lib.mapAttrsToList (name: set: do ({ attrPath = attrPath ++ [ name ]; }) set) (attrsFromDir.entries)
          );
        }
        .${attrsFromDir.type};
    in
    do { attrPath = [ ]; } attrsFromDirTop;

  /*
    This function produces `make` targets from a list of attrPaths.
    The result is a single string with _space separated_ targets,
    where each target contains the string formatted attrPath
    that are defiend by the derivation, i.e.: `<attrPath> <attrPath> ...`.

    :::Note
    Todo: tricky attrs, e.g. containing spaces, although renaming is not possible at this point
    :::

    :::Note
    This function _evaluates_ the package set, and requires the attrPaths to both exist,
    and point to a derivation.
    :::

    # Type

    ```
    makeTargets :: [ [String] ] -> String
    ```
  */
  makeTargets =
    # list of attrpaths e.g. result from `lib.nef.reflect.collectAttrPaths
    collectedAttrPaths:
    let
      mkAttrPathsWithOutputs = map (x: (lib.showAttrPath x.attrPath));
    in
    lib.concatStringsSep " " (mkAttrPathsWithOutputs collectedAttrPaths);

in
{

  inherit
    collectAttrPaths
    makeTargets
    ;

}
