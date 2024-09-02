{mkMizeRustModule
, buildModule
, findModules
, ...
}: let
  mmeFlake = builtins.getFlake ./.;
  presenters = mmeFlake.inputs.presenters;
in [

mkMizeRustModule {
  name = "mme";
  src = ./.;
}

] ++ map buildModule (findModules presenters)
