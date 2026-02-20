{
  rustPlatform,
  lib,
  ...
}:

rustPlatform.buildRustPackage rec {
  pname = "i3-cycles";
  version = "0.1.0";

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };

  meta = with lib; {
    description = "Cycle through grouped i3 workspaces with named cycles";
    homepage = "https://github.com/mrvandalo/nixos-config";
    license = licenses.mit;
    maintainers = [ ];
    platforms = platforms.linux;
  };
}
