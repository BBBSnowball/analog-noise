{ lib
, fetchFromGitHub
, buildPythonApplication
, psutil
, kicad
, xorg
, xdotool
, libxslt
, imagemagick
, xclip
, x11vnc
, fluxbox
, wmctrl
, xvfbwrapper
, recordmydesktop
}:

buildPythonApplication rec {
  pname = "KiAuto";
  version = "2.2.5";
  src = fetchFromGitHub {
    owner = "INTI-CMNB";
    repo = pname;
    rev = "v${version}";
    hash = "sha256-CovbN8squukC47j4A+SEJKX3R67PK/YvTPj0MoUB/Ps=";
  };
  propagatedBuildInputs = [
    kicad
    psutil
    xvfbwrapper

    xorg.xorgserver
    xdotool
    libxslt
    imagemagick
    xclip

    x11vnc
    fluxbox
    wmctrl
    recordmydesktop
  ];
}