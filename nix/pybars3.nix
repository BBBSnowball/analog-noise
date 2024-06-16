# copied from https://github.com/evils/nixpkgs/commit/f3ff36007c85ea201296c03fc55e457f7ff1ebdd
{ lib
, fetchPypi
, buildPythonPackage
, pymeta3
}:

buildPythonPackage rec {
  pname = "pybars3";
  version = "0.9.7";

  src = fetchPypi {
    inherit pname version;
    sha256 = "0lp2q0gazy8nii9g8ybzfszjfpj7234i5wbajddrqfz50pllgj3a";
  };

  propagatedBuildInputs = [ pymeta3 ];

  # some deprecation notice on the test seemed like a good excuse not to get it working
  doCheck = false;

  meta = with lib; {
    description = "Handlebars.js template support";
    homepage = "https://github.com/wbond/pybars3";
    license = licenses.lgpl3;
    maintainers = with maintainers; [ evils ];
  };
}