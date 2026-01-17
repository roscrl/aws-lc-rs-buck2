{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs, ... } @ inputs:
  let
    supportedSystems = [ "x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin" ];
    forAllSystems = f: nixpkgs.lib.genAttrs supportedSystems (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      in
        f pkgs system inputs
    );
  in {
    devShells = forAllSystems (pkgs: system: inputs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          # Build tools
          buck2
          
          # Rust toolchain
          rustc
          cargo
          rustfmt
          clippy
          
          # C/C++ toolchain
          clang
          llvm
          lld
          
          # Other tools
          go
          python3
          jq
        ];
        
        # Set up environment for Buck2
        shellHook = ''
          export CC=clang
          export CXX=clang++
        '';
      };
    });
  };
}
