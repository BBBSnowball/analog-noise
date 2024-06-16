
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
              blender
              git
              imagemagick
              librsvg
              ghostscript
              pandoc
            ]) ++ [
              kikit
              kiauto
            ];
          };