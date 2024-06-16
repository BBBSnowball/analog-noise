{ lib
, pkgs
, buildPythonApplication
, fetchFromGitHub
, wxPython_4_2
, kicad
, which
}:

buildPythonApplication rec {
  pname = "InteractiveHtmlBom";
  version = "2.6.0-1";

  src = fetchFromGitHub {
    owner = "INTI-CMNB";
    repo = pname;
    rev = "v${version}";
    hash = "sha256-/TMF9dulK+JYsNGktuAw87u5jimUzNg8jpPJX1YPUFc=";
  };

  propagatedBuildInputs = [ wxPython_4_2 kicad pkgs.git ];
  nativeBuildInputs = [ which ];

  doCheck = false;

  wrapper = ''
    #!@python3@
    import sys, subprocess
    sys.exit(subprocess.run(["@out@/bin/generate_interactive_bom.py.sh"] + sys.argv[1:]).returncode)
  '';
  passAsFile = [ "wrapper" ];

  postFixup = ''
    # Our main binary is a .py file and KiBot will try to call it as `python3 $(which generate_interactive_bom.py)`
    # so better make it a Python file and not a wrapper script.
    mv $out/bin/generate_interactive_bom.py $out/bin/generate_interactive_bom.py.sh
    substitute $wrapperPath $out/bin/generate_interactive_bom.py --subst-var out --subst-var-by python3 $(which python3)
    chmod +x $out/bin/generate_interactive_bom.py
  '';
}
