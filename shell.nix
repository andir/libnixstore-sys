with import <nixpkgs> {};
let
  libnixstore-c = callPackage (fetchFromGitHub {
    owner = "andir";
    repo = "libnixstore-c";
    rev = "16031bdfef9164f528918c1be8fd5ccb39da1fa1";
    sha256 = "0y99ybdpghknrq6vn4m0zd67768rbrvl4fd6k9xkvkzzn9cqsjv9";
  }) {};
in mkShell {
  buildInputs = [ libnixstore-c llvmPackages.clang-unwrapped.lib ];
  nativeBuildInputs = [ gdb pkgconfig ];

  shellHook = ''
    export LIBCLANG_PATH=${llvmPackages.clang-unwrapped.lib}/lib
  '';
}
