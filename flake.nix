{
  description = "abstwrite Development Environment";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-25.05";
  };

  outputs = { self , nixpkgs ,... }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
    };

  in {

    devShells."${system}".default = pkgs.mkShell {
      
      packages = with pkgs; [
	rustc
	cargo
      ];

      shellHook = ''
        echo 'abstwrite Dev Environment'
      '';

    };

    };

}
