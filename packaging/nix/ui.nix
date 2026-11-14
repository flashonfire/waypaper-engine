{
  self,
  lib,
  rustPlatform,
  mold,
  pkg-config,
  fetchPnpmDeps,
  pnpmConfigHook,
  pnpm_10,
  webkitgtk_4_1,
  wrapGAppsHook3,
}:
let
  projectRoot = "waypaper_engine_ui";
  cargoToml = lib.importTOML ../../${projectRoot}/src-tauri/Cargo.toml;
  rev = self.shortRev or self.dirtyShortRev or "dirty";
  pnpm = pnpm_10;
in
rustPlatform.buildRustPackage rec {
  pname = cargoToml.package.name;
  version = "${cargoToml.package.version}+git.${rev}";

  src = lib.cleanSource ../..;
  sourceRoot = "${src.name}/${projectRoot}";

  cargoRoot = "src-tauri";
  buildAndTestSubdir = cargoRoot;

  cargoLock.lockFile = ../../Cargo.lock;

  pnpmDeps = fetchPnpmDeps {
    inherit
      pname
      version
      src
      sourceRoot
      pnpm
      ;
    fetcherVersion = 5;
    hash = "sha256-XtXIJPpDRejANl+GvJWCxH3W04ASgjVVybgcZPdEmB4=";
  };

  nativeBuildInputs = [
    mold
    pkg-config
    pnpm
    pnpmConfigHook
    rustPlatform.bindgenHook
    wrapGAppsHook3
  ];

  buildInputs = [
    webkitgtk_4_1
  ];

  env.RUSTFLAGS = "-C link-arg=-fuse-ld=mold";

  postPatch = ''
    ln -s ${../../Cargo.lock} ${cargoRoot}/Cargo.lock
  '';

  doCheck = false;

  meta = {
    mainProgram = pname;
    platforms = lib.platforms.linux;
  };
}
