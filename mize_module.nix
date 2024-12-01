{


module = { buildMizeForSystem, mizeBuildConfig, mkMizeRustModule, crossSystem, pkgsCross, mkMizeRustShell, pkgs, pkgsNative, buildNpmPackage, mkSelString, ... }: let

  selector_string = mkSelString {
    modName = "mme";
  };
  hash = builtins.substring 0 32 (builtins.hashString "sha256" selector_string);

in mkMizeRustModule ({
  modName = "mme";
  src = ./.;
  cargoExtraArgs = "--no-default-features --lib";

  ## add the js-runtimme
  postInstall = let 
  url = "file:///home/me/work/mme/dist/mize.js";
  html = ''
    <html>
      <head>
        <!-- script src="https://c2vi.dev/mize/wasm32-none-unknown/mize.js"></script --!>
        <script src="${url}"></script>
        <script>
          console.log("from script")
        </script>
      </head>
      <body>
        hello world...............
      </body>  
    </html>
  '';
  in ''
    echo -en "${html}" > $out/index.html
  '';

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

);



lib = { mkMizeModule, buildNpmPackage, ... }: rec {
  mkMmePresenter = attrs: mkMizeModule (attrs // {
    src = attrs.src;
    modName = attrs.name;
    select = {
      mme_presenter = true;
    };
  });

  mkMmeNpmPresenter = attrs: buildNpmPackage (attrs // {
  });

  mkMmeHtmlPresenter = attrs: mkMmePresenter ({
    dontUnpack = true;
    dontPath = true;
    buildPhase = "";
    installPhase = ''
      mkdir -p $out
      cp -r ${attrs.src}/* $out
    '';
  } // attrs);
};




externals = { fetchFromGitHub, ... }: [

  (fetchFromGitHub {
    owner = "c2vi";
    repo = "mme-presenters";
    rev = "master";
    hash = "sha256-FeMBDCJBkw9XOLXC1rfedNk71uyg4OTCHaaO1jAAGto=";
  })

];


}

