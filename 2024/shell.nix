{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  name = "go-shell";

  buildInputs = [
    pkgs.go
    pkgs.git # Optional: Add git if your Go projects depend on it
    pkgs.direnv # Optional: For environment variable management
  ];

  # Set environment variables, e.g., for Go paths
  shellHook = ''
    export GOPATH=$HOME/go
    export PATH=$GOPATH/bin:$PATH
  '';
}

