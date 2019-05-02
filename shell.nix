with import <nixpkgs> {};
let
  libnixstore-c = callPackage (fetchFromGitHub {
    owner = "andir";
    repo = "libnixstore-c";
    rev = "54f9c94f239cc4cf436c942ae708e4b500934cd7";
    sha256 = "0v0zqq1cv7zd5h3dab5pcfr1vbr7qz4qaffxwxjni3vs4va42k0z";
  }) {};
in mkShell {
  buildInputs = [ libnixstore-c llvmPackages.clang-unwrapped.lib ];
  nativeBuildInputs = [ gdb ];

  shellHook = ''
    export LIBCLANG_PATH=${llvmPackages.clang-unwrapped.lib}/lib
  '';
}
