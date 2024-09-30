{mkMizeRustModule
, buildModule
, findModules
, mkMizeModule
, buildNpmPackage
, crossSystem
, pkgsCross
, pkgsNative
, pkgs
, mkMizeRustShell
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
  cargoExtraArgs = "--no-default-features --lib";
}

// (if crossSystem.kernel.name == "linux" then builtins.trace "adding linux stuff" {
  nativeBuildInputs = with pkgsCross.buildPackages; [
    pkg-config
  ];
  buildInputs = with pkgsNative; [
    webkitgtk_4_1
  ];
} else {})

# add the devShell
// {
  devShell = mkMizeRustShell {
    nativeBuildInputs = with pkgs; [
      #emscripten
      wasm-pack
      pkg-config 
      webkitgtk_4_1
      libsForQt5.full
      cmake
      nasm
      pkg-config
      nodejs
    ];

    buildInputs = with pkgs; [
      openssl
      #pango
      #libsoup_3
      webkitgtk_4_1
      # gobject-introspection gtk4 atkmm
    ];

    shellHook = ''
      echo hiiiiiiiiiiiiii
      export LD_LIBRARY_PATH=${pkgs.webkitgtk_4_1}/lib:${pkgs.libsoup_3}/lib:${pkgs.glib.out}/lib:${pkgs.gtk3}/lib:${pkgs.cairo}/lib:${pkgs.gdk-pixbuf}/lib:${pkgs.libxkbcommon}/lib:${pkgs.fontconfig.lib}/lib:${pkgs.libsForQt5.full}/lib:${pkgs.stdenv.cc.cc.lib}/lib:${pkgs.libsForQt5.qt5.qtwebengine}/lib
      export CPLUS_INCLUDE_PATH=${pkgs.libsForQt5.full}/include:${pkgs.libsForQt5.qt5.qtwebengine}/include
      export MME_QT_LIB=${pkgs.libsForQt5.full}/lib

      # i found that this is the env war to set where QT looks for platform plugins
      # at: https://forums.fedoraforum.org/showthread.php?326508-How-to-set-QT_QPA_PLATFORM_PLUGIN_PATH
      export QT_QPA_PLATFORM_PLUGIN_PATH=${pkgs.libsForQt5.full}/lib/qt-5.15.14/plugins/platforms/
      
      alias run="${./.}/run.sh"
    '';

  };
}

)


# build all presenters as submodules
#] ++ map (mod: buildModule mod extraArgs) (findModules presenters)
#]
