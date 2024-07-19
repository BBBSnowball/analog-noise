{
  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    # don't include darwin because some packages are broken for darwin
    # and we would like to use `nix flake check` (and packages that are
    # marked broken will abort before it does checks for the other archs)
    let systems = [ "aarch64-linux" "x86_64-linux" ]; in
    flake-utils.lib.eachSystem systems (system:
      let pkgs = nixpkgs.legacyPackages.${system}; prev = pkgs; in
      {
        # see https://discourse.nixos.org/t/add-python-package-via-overlay/19783/4
        other.pythonPackagesOverlays = (prev.pythonPackagesOverlays or [ ]) ++ [
          (python-final: python-prev: {
            pybars3 = python-final.callPackage nix/pybars3.nix { };
            pymeta3 = python-final.callPackage nix/pymeta3.nix { };
            qrcodegen = python-final.callPackage nix/qrcodegen.nix { };
            svgutils = python-final.callPackage nix/svgutils.nix { };
            pcbnewTransition = python-final.callPackage nix/pcbnewTransition.nix { };
          })
        ];
        other.python3Packages = self.packages.${system}.python3.pkgs;
        packages = with self.other.${system}; rec {
          python3 = prev.python3.override {
            self = python3;
            packageOverrides = prev.lib.composeManyExtensions pythonPackagesOverlays;
          };

          recordmydesktop = pkgs.callPackage nix/recordmydesktop.nix { };
          kikit = python3Packages.callPackage nix/kikit.nix { };
          kiauto = python3Packages.callPackage nix/kiauto.nix { inherit recordmydesktop; };
          kidiff = python3Packages.callPackage nix/kidiff.nix { inherit kiauto; };
          InteractiveHtmlBom = python3Packages.callPackage nix/InteractiveHtmlBom.nix { };

          kicad-vars = pkgs.stdenv.mkDerivation {
            name = "kicad-vars";
            src = null;
            nativeBuildInputs = [ pkgs.makeWrapper ];
            makeWrapperArgs = pkgs.kicad.makeWrapperArgs;
            unpackPhase = "true";
            buildPhase = "";
            # make a dummy wrapper and remove the `exec ...` line from it so it can be sourced in a Nix shell
            installPhase = ''
              dummy=`mktemp tmp.XXXXXXXXXXX`
              chmod +x $dummy
              makeWrapper $dummy $out $makeWrapperArgs
              sed -i "/$dummy/ d" $out
            '';
          };

          kibot = python3Packages.buildPythonApplication {
            pname = "KiBot";
            version = "2";
            src = pkgs.fetchFromGitHub {
              owner = "INTI-CMNB";
              repo = "KiBot";
              rev = "v2";
              hash = "sha256-ArOKcJLwhLKXNRQIWXzWvMsjC7tJQQoi1kfRHNVdQ+o=";
            };
            propagatedBuildInputs = (with python3Packages; [
              pyaml
              requests
              markdown2
              mistune
              qrcode
              colorama
              numpy
              qrcodegen
              XlsxWriter
              lxml

              lark
              svgutils
            ]) ++ (with pkgs; [
              openscad
              xorg.xorgserver
              xvfb-run
              (if blender.meta.unsupported then null else blender)
              git
              imagemagick
              librsvg
              ghostscript
              pandoc
              #helvetica-neue-lt-std
              #tetex
              (texlive.combine {
                inherit (pkgs.texlive) scheme-tetex iftex;
              })
            ]) ++ [
              kikit
              kiauto
              kidiff
              InteractiveHtmlBom
            ];

            # Tests will test more features than we need (and thus need more dependencies) so let's skip them.
            doCheck = false;

            #postPatch = ''
            #  #sed -i 's/from .macros import macros, /from .macros import /' kibot/*.py
            #  echo "def macros():" >>kibot/macros.py
            #  echo '  """dummy"""' >>kibot/macros.py
            #  echo '  pass' >>kibot/macros.py
            #'';

            postInstall = ''
              # KiBot seems to have some way of auto-importing stuff and that breaks if there are .pyc files.
              # (ERROR:Make sure you used `--no-compile` if you used pip for installation (kibot - kiplot.py:79))
              rm $out/lib/python*/site-packages/kibot/__pycache__/*.pyc
            '';
          };

          default = kibot;
        };
        apps = rec {
          kikit = flake-utils.lib.mkApp { drv = self.packages.${system}.kikit; };
          kibot = flake-utils.lib.mkApp { drv = self.packages.${system}.kibot; };
          default = kibot;

          #python-kibot = flake-utils.lib.mkApp { name = "python"; drv = self.devShells.${system}.kibot.passthru.python; };  # doesn't work
          python-rust  = flake-utils.lib.mkApp { name = "python"; drv = self.devShells.${system}.rust.passthru.python; };
        };
        devShells = let
          flakePkgs = self.packages.${system};
        in rec {
          kibot = with flakePkgs; pkgs.mkShell rec {
            passthru.packages = [ flakePkgs.kibot ];
            passthru.pythonPackages = p: [ flakePkgs.kibot ];
            passthru.python = python3.withPackages passthru.pythonPackages;

            packages = passthru.packages ++ [ passthru.python ];

            shellHook = ''
              # KiAuto wants to run stuff inside xvfb so let's make sure that we don't leak windows to our Wayland desktop.
              unset WAYLAND_DISPLAY

              # The Kicad package puts several paths into environment variables but only in the wrapper script.
              # KiAuto needs some of them, e.g. to find footprint that are available locally.
              source ${kicad-vars}

              # Fix crash in Gtk save dialog, e.g. start eeschema, open ERC window, click save.
              # https://github.com/NixOS/nixpkgs/issues/149812#issuecomment-1004387735
              export XDG_DATA_DIRS="${pkgs.gtk3}/share/gsettings-schemas/${pkgs.gtk3.name}:$XDG_DATA_DIRS"

              # The interposer causes a crash and KiBot doesn't seem to have any way to disable it or pass extra parameters to
              # eeschema_do but we can disable it with an environment variable.
              export KIAUTO_INTERPOSER_DISABLE=1

              # Cache 3d models
              # -> README says that the default is not to cache but in fact it uses a sensible default.
              #if [ -z "$KIBOT_3D_MODELS" ] ; then
              #  if [ -n "$XDG_CACHE_HOME" ] ; then
              #    export KIBOT_3D_MODELS="$XDG_CACHE_HOME/kibot/3d"
              #  else
              #    export KIBOT_3D_MODELS="$HOME/.cache/kibot/3d"
              #  fi
              #  if ! mkdir -p "$KIBOT_3D_MODELS" ; then
              #    echo "WARN: I cannot create the cache directory for 3D models so they won't be cached."
              #    unset KIBOT_3D_MODELS
              #  fi
              #fi
            '';
          };

          rust = with flakePkgs; pkgs.mkShell rec {
            passthru.packages = with pkgs; [
              rustup udev.dev pkg-config openssl.dev picotool
              openocd
            ];
            passthru.pythonPackages = p: with p; [ pyserial ];
            passthru.python = python3.withPackages passthru.pythonPackages;

            packages = passthru.packages ++ [ passthru.python ];
          };

          default = pkgs.mkShell rec {
            passthru.platformio-python-fn = pp: let
              default = pp.callPackage "${nixpkgs}/pkgs/development/embedded/platformio/default.nix" {};
              # get src and version out of it with dirty tricks
              default2 = default.platformio-chrootenv.override (p: {
                buildFHSUserEnv = _: { inherit (p) src version; };
              });
              core = pp.callPackage "${nixpkgs}/pkgs/development/embedded/platformio/core.nix" {
                #inherit (default2) version src;
              };
            in core;
            passthru.platformio-python-pkg = pkgs.python3Packages.toPythonModule (passthru.platformio-python-fn pkgs.python3Packages);
            passthru.platformio-python = flakePkgs.python3.withPackages (p: [ (p.toPythonModule (passthru.platformio-python-fn p)) ]);

            passthru.python = flakePkgs.python3.withPackages (p:
              kibot.passthru.pythonPackages p
              ++ rust.passthru.pythonPackages p
              ++ [ (p.toPythonModule (passthru.platformio-python-fn p)) ]
            );

            #packages = kibot.nativeBuildInputs ++ rust.nativeBuildInputs;
            packages = kibot.passthru.packages ++ rust.passthru.packages ++ [ passthru.python (pkgs.platformio-core or pkgs.platformio) ];
            shellHook = kibot.shellHook;
          };
        };
      }
    );
}
