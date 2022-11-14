{
  description = "Fee Manager";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };
      rustVersion = pkgs.rust-bin.stable.latest.default;

      rustPlatform = pkgs.makeRustPlatform {
        cargo = rustVersion;
        rustc = rustVersion;
      };

      fee-manager = rustPlatform.buildRustPackage {
        pname = "fee-manager";
        version = "0.1.0";
        src = ./.;
        cargoLock.lockFile = ./Cargo.lock;
      };
    in {
      packages."x86_64-linux".fee-manager = fee-manager;
      packages."x86_64-linux".default = self.packages."${system}".fee-manager;
      apps."x86_64-linux".fee-manager = {
        type = "app";
        program = "${self.packages.x86_64-linux.default}/bin/fee-manager";
      };
      apps."x86_64-linux".default = self.apps."${system}".fee-manager;

      devShells."x86_64-linux".default = pkgs.mkShell {
        buildInputs =
          [ (rustVersion.override { extensions = [ "rust-src"]; }) pkgs.rust-analyzer pkgs.cargo-flamegraph];
      };
    };
}
