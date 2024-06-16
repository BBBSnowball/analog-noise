{ lib
, fetchPypi
, buildPythonPackage
, versioneer
, kicad
}:

buildPythonPackage rec {
  pname = "pcbnewTransition";
  version = "0.3.4";

  src = fetchPypi {
    inherit pname version;
    hash = "sha256-3CJUG1kd63Lg0r9HpJRIvttHS5s2EuZRoxeXrqsJ/kQ=";
  };

  propagatedBuildInputs = [
    versioneer
    kicad
  ];
}