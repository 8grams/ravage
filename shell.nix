let
  nixpkgs = fetchTarball "https://github.com/NixOS/nixpkgs/tarball/nixos-24.11";
  pkgs = import nixpkgs { config = { allowUnfree = true; }; overlays = []; };

  redis = pkgs.redis.overrideAttrs (oldAttrs: rec {
    version = "7.2.7";
  });
in

pkgs.mkShell {
  packages = with pkgs; [
    redis
  ];

  shellHook = ''
    export NIX_SHELL_DIR=$PWD/.nix-shell
    export LC_ALL=C
    export LANG=C.utf8

    # Setup data dir
    mkdir -p $NIX_SHELL_DIR

    trap \
    "
      pkill redis-server
    " EXIT

    echo "Run redis.. See log on $NIX_SHELL_DIR/redis.log"
    nohup redis-server > $NIX_SHELL_DIR/redis.log 2>&1 &
  '';
}
