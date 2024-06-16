{ lib
, fetchPypi
, buildPythonPackage
, lxml
}:

buildPythonPackage rec {
  pname = "svgutils";
  version = "0.3.4";

  src = fetchPypi {
    inherit pname version;
    hash = "sha256-nvSPRMsdRgp3R90CaUIA/aJeufr23qOSEY3vJpXg4FM=";
  };

  propagatedBuildInputs = [
    lxml
  ];
}