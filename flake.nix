{
  description = "GUI for the Sinara 8452 Stabilizer";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";

  outputs =
    {
      self,
      nixpkgs,
    }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
      };

      pystabilizer = pkgs.python3Packages.buildPythonPackage {
        pname = "pystabilizer";
        version = "0.0.0";
        format = "pyproject";
        src = "${self}/pystabilizer";

        nativeBuildInputs = [
          pkgs.python3Packages.setuptools
          pkgs.qt6.wrapQtAppsHook
        ];
        propagatedBuildInputs =
          [ pkgs.qt6.qtbase ]
          ++ (with pkgs.python3Packages; [
            numpy
            matplotlib
            pyqtgraph
            pyqt6
            qasync
            pglive
          ]);

        dontWrapQtApps = true;
        postFixup = ''
          wrapQtApp "$out/bin/stabilizer_control_panel"
        '';
      };

      pglive = pkgs.python3Packages.buildPythonPackage rec {
        pname = "pglive";
        version = "0.7.2";
        format = "pyproject";
        src = pkgs.fetchPypi {
          inherit pname version;
          hash = "sha256-jqj8X6H1N5mJQ4OrY5ANqRB0YJByqg/bNneEALWmH1A=";
        };
        buildInputs = [ pkgs.python3Packages.poetry-core ];
        propagatedBuildInputs = with pkgs.python3Packages; [
          pyqtgraph
          numpy
        ];
      };
    in
    {
      packages.x86_64-linux = {
        default = pystabilizer;
      };

      apps.x86_64-linux.control_panel = {
        type = "app";
        program = "${pystabilizer}/bin/stabilizer_control_panel";
      };

      formatter.x86_64-linux = nixpkgs.legacyPackages.x86_64-linux.nixfmt-rfc-style;
    };
}
