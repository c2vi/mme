{mkMizeRustModule
, buildModule
, findModules
, mkMizeModule
, buildNpmPackage
, crossSystem
, pkgsCross
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
in #[

mkMizeRustModule ({
  modName = "mme";
  src = ./.;
}

// (if crossSystem.kernel.name == "linux" then builtins.trace "adding linux stuff" {
  nativeBuildInputs = with pkgsCross.buildPackages; [
    pkg-config
  ];
  buildInputs = with pkgsCross; [
    webkitgtk_4_1
  ];
} else {})

)


# build all presenters as submodules
#] ++ map (mod: buildModule mod extraArgs) (findModules presenters)
#]
