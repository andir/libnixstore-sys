with import <nixpkgs> {};
let
  libnixstore-c = callPackage (fetchFromGitHub {
    owner = "andir";
    repo = "libnixstore-c";
    rev = "95e8f5d51a69ac2325cf7e96595e0752047ef76c";
    sha256 = "0k05ag1673g8dij038sljsch43aanqakk15ixwgibab3cx3v2i86";
  }) {};
in mkShell {
  buildInputs = [ libnixstore-c llvmPackages.clang-unwrapped.lib ];

  shellHook = ''
    export LIBCLANG_PATH=${llvmPackages.clang-unwrapped.lib}/lib
  '';
}
