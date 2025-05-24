{
  system,
  fenix,
}:
fenix.packages.${system}.fromToolchainFile {
  file = ./rust-toolchain.toml;
  # just copy the hash for now and nix will tell you the right hash
  sha256 = "sha256-gh/xTkxKHL4eiRXzWv8KP7vfjSk61Iq48x47BEDFgfk=";
}
