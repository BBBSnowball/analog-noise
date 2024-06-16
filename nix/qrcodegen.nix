{ lib
, fetchPypi
, fetchFromGitHub
, buildPythonPackage
}:

buildPythonPackage rec {
  pname = "qrcodegen";
  version = "1.8.0";

  #src = fetchPypi {
  #  inherit pname version;
  #};
  src = let fullSrc = fetchFromGitHub {
    owner = "nayuki";
    repo = "QR-Code-generator";
    rev = "v${version}";
    hash = "sha256-aci5SFBRNRrSub4XVJ2luHNZ2pAUegjgQ6pD9kpkaTY=";
  }; in "${fullSrc}/python";
}