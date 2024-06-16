{ lib
, fetchFromGitHub
, buildPythonApplication
, kicad
, wxPython_4_2
, imagemagick
, librsvg
, poppler_utils
, xdg-utils
, kiauto
}:

buildPythonApplication rec {
  pname = "KiDiff";
  version = "2.4.7";
  src = fetchFromGitHub {
    owner = "INTI-CMNB";
    repo = pname;
    rev = "v${version}";
    hash = "sha256-dRhoIzXeOJHZ5w0tGL69YidTHHWQ+rJ8iZHwOTmdtcg=";
  };
  propagatedBuildInputs = [
    kicad
    wxPython_4_2

    imagemagick
    librsvg
    poppler_utils
    xdg-utils

    kiauto
  ];
}
