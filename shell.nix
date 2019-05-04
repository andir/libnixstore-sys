with import <nixpkgs> {};
let
  libnixstore-c = callPackage (fetchFromGitHub {
    owner = "andir";
    repo = "libnixstore-c";
    rev = "2ce29c26a4bca55bb5b83fa97f045659675bcfa7";
    sha256 = "1jk798ck2ppkkw9i1p07rqk0xsi9bv89a62idwmzgxw6lniipby0";
  }) {};
in mkShell {
  buildInputs = [ libnixstore-c llvmPackages.clang-unwrapped.lib ];
  nativeBuildInputs = [ gdb pkgconfig ];

  shellHook = ''
    export LIBCLANG_PATH=${llvmPackages.clang-unwrapped.lib}/lib
  '';
}
