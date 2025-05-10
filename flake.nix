{
  description = "The Nova programming language";

  # Declare inputs to fetch from
  inputs = {
    nixpkgs = {
      url = "github:NixOS/nixpkgs/nixpkgs-unstable";
      flake = true;
    };
  };

  # Declare your Nix environment
  outputs = { self, nixpkgs, flake-utils }: 
    let
     forAllSystems = function:
      nixpkgs.lib.genAttrs [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ] (system: function nixpkgs.legacyPackages.${system});
    in {
      devShell = forAllSystems (pkgs: 
          with pkgs; mkShell {
            buildInputs = [
              # Add your dependencies here
              rustup
              cargo
              rust-analyzer
              # wabt
              #nodejs_22
              #wasm-tools
              http-server
              #clang
              #llvm_18
              #libffi
              #libxml2
              rustc
              darwin.apple_sdk.frameworks.System
            ]; 

            shellHook = ''
              export ENV_NAME="$ENV_NAME nova"

              # If on darwin, set LIBRARY_PATH to xcode lib
              # and download rust-analyer
              if [[ "$(uname)" == "Darwin" ]]; then
                export LIBRARY_PATH="$LIBRARY_PATH:/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/lib"
                rustup component add rust-analyzer
              fi

              exec $SHELL
            '';
          });
    };
}

