{
  description = "Polkadot Blockchain Academy — dev environment (Node.js + Yarn)";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs =
    { nixpkgs, ... }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];
      forAllSystems = nixpkgs.lib.genAttrs systems;
    in
    {
      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              # .nvmrc says lts/hydrogen (Node 18), but Node 18 is EOL and
              # flagged insecure in nixpkgs. Node 22 is the active LTS and the
              # repo's install/build/test scripts run fine on it.
              nodejs_22
              # Any Yarn 4 shim delegates to the repo-pinned Yarn 3.2.1 via
              # `yarnPath` in .yarnrc.yml, so versions always match the repo.
              yarn-berry
              # Needed by Yarn for any git-hosted dependencies.
              git
            ];

            shellHook = ''
              echo "node $(node --version), yarn $(yarn --version)"
            '';
          };
        }
      );
    };
}
