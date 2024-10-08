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
              nodejs_22
              wasm-tools
              http-server
            ]; 

            shellHook = ''
              export ENV_NAME=nova
              exec $SHELL
            '';
          });
    };
}

