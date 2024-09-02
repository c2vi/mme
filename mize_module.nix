{mkMizeRustModule
, buildModule
, findModules
, mkMizeModule
, buildNpmPackage
, ...
}: let
  mmeFlake = builtins.getFlake ./.;
  presenters = mmeFlake.inputs.presenters;

  mkMmePresenter = attrs: mkMizeModule ({
    select = {
      inherit (attrs) name;
      MmePresenter = true;
    };
  } // attrs);

  mkMmeNpmPresenter = attrs: buildNpmPackage attrs // {
  };

  mkMmeHtmlPresenter = attrs: mkMmePresenter {
    dontUnpack = true;
    dontPath = true;
    buildPhase = "";
    installPhase = ''
      mkdir -p $out
      cp -r ${attrs.src}/* $out
    '';
  };

  extraArgs = {
    inherit mkMmePresenter mkMmeHtmlPresenter;
  };
in [

mkMizeRustModule {
  modName = "mme";
  src = ./.;
}

# build all presenters as submodules
] ++ map (mod: buildModule mod extraArgs) (findModules presenters)
