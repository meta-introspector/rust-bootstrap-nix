last issue
bash-5.3$ nix build
trace: Rust 1.92.0-nightly-2025-10-16:
Pre-aggregated package `rust` is not encouraged for stable channel since it contains almost all and uncertain components.
Consider use `default` profile like `rust-bin.stable.latest.default` and override it with extensions you need.
See README for more information.

this derivation will be built:
  /nix/store/jjy833sc0z7xcl495sfkyx2rcqyfigmi-rust-solana-tools-v1.51.drv
building '/nix/store/jjy833sc0z7xcl495sfkyx2rcqyfigmi-rust-solana-tools-v1.51.drv'...
error: builder for '/nix/store/jjy833sc0z7xcl495sfkyx2rcqyfigmi-rust-solana-tools-v1.51.drv' failed with exit code 1;
       last 25 log lines:
       >    Compiling filetime v0.2.25
       >    Compiling cpufeatures v0.2.14
       >    Compiling itoa v1.0.11
       >    Compiling ryu v1.0.18
       >    Compiling bootstrap v0.0.0 (/tmp/nix-shell.R9IS5s/nix-shell.TZl15H/nix-build-rust-solana-tools-v1.51.drv-0/k7wrn478pqvwbzcr7gkbjghcphp62kxd-source/src/bootstrap)
       >    Compiling tar v0.4.42
       >    Compiling sha2 v0.10.8
       >    Compiling clap_derive v4.5.18
       >    Compiling serde_derive v1.0.210
       >    Compiling ignore v0.4.23
       >    Compiling opener v0.5.2
       >    Compiling fd-lock v4.0.2
       >    Compiling toml v0.5.11
       >    Compiling cmake v0.1.48
       >    Compiling object v0.36.4
       >    Compiling home v0.5.9
       >    Compiling termcolor v1.4.1
       >    Compiling clap v4.5.18
       >    Compiling clap_complete v4.5.29
       >    Compiling build_helper v0.1.0 (/tmp/nix-shell.R9IS5s/nix-shell.TZl15H/nix-build-rust-solana-tools-v1.51.drv-0/k7wrn478pqvwbzcr7gkbjghcphp62kxd-source/src/build_helper)
       >    Compiling xz2 v0.1.7
       >     Finished `dev` profile [unoptimized] target(s) in 1m 55s
       > DEBUG: Entering run function, about to execute command.
       > ERROR: Failed to parse 'config.toml': unknown field `CARGO_HOME`
       > Build completed unsuccessfully in 0:01:55
       For full logs, run:
         nix log /nix/store/jjy833sc0z7xcl495sfkyx2rcqyfigmi-rust-solana-tools-v1.51.drv
bash-5.3$ 
