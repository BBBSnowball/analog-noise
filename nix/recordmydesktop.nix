{ stdenv
, fetchzip
, gnumake
, automake
, autoconf
, zlib
, xorg
, libvorbis
, libogg
, libtheora
, alsa-lib
}:

stdenv.mkDerivation rec {
    pname = "recordmydesktop";
    version = "0.3.8.1";

    src = fetchzip {
        url = "https://downloads.sourceforge.net/project/${pname}/${pname}/${version}/${pname}-${version}.tar.gz";
        hash = "sha256-X9/xCQUf8bA1jg+V0Wa+a6puHhB7M6tqe4s8QbtNCHw=";
    };

    nativeBuildInputs = [ gnumake automake autoconf ];
    buildInputs = [
        zlib
        xorg.libICE
        xorg.libSM
        xorg.libX11
        xorg.libXext
        xorg.libXfixes
        xorg.libXdamage
        libvorbis
        libogg
        libtheora
        alsa-lib
    ];
}
