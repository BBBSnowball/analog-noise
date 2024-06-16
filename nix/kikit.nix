{ lib
, fetchFromGitHub
, buildPythonApplication
, numpy
, shapely
, click
, markdown2
, pybars3
, kicad
, versioneer
, pcbnewTransition
, commentjson
, wxPython_4_2
}:

buildPythonApplication rec {
  pname = "KiKit";
  version = "1.3.0-7";
  src = fetchFromGitHub {
    owner = "INTI-CMNB";
    repo = "KiKit";
    rev = "v${version}";
    hash = "sha256-IIWhBFmEpUWoI27b34OovO/6WnBiB1zENyW/nw0xMec=";
  };
  propagatedBuildInputs = [
    numpy
    shapely
    click
    markdown2
    pybars3
    kicad
    versioneer
    pcbnewTransition
    commentjson
    wxPython_4_2
  ];

  doCheck = false;
}
