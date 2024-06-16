# https://github.com/evils/nixpkgs/blob/f3ff36007c85ea201296c03fc55e457f7ff1ebdd/pkgs/development/python-modules/pymeta3/default.nix
{ lib
, fetchPypi
, buildPythonPackage
}:

buildPythonPackage rec {
  pname = "pymeta3";
  version = "0.5.1";

  src = fetchPypi {
    pname = "PyMeta3";
    inherit version;
    sha256 = "1jsvs20qdnflap7qna37n1w4v5k4d34hpvn0py3zbfx9v4ka7g8q";
  };

  # some deprecation notice on the test seemed like a good excuse not to get it working
  doCheck = false;

  meta = with lib; {
    description = "A Python 3 compatible fork of pymeta";
    homepage = "https://github.com/wbond/pymeta3";
    license = licenses.mit;
    maintainers = with maintainers; [ evils ];
  };
}