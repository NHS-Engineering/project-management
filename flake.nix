{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/release-22.11";
		mozilla.url = "github:mozilla/nixpkgs-mozilla";
		deploy-rs.url = "github:serokell/deploy-rs";
		nix-filter.url = "github:numtide/nix-filter";
	};

	outputs = { self, nixpkgs, mozilla, deploy-rs, nix-filter }:
		let pkgs = import nixpkgs {
			system = "x86_64-linux";
			overlays = [
				mozilla.overlay
				nix-filter.overlays.default
			];
		};
		in let nightlyRust = (pkgs.rustChannelOf {
			date = "2022-11-07";
			channel = "nightly";
			sha256 = "sha256-Z51XyBXGzDNX5ioVr4JvBnOsD/+iIW1PVIQCdrn82jg=";
		});
		in let rustPlatform = pkgs.makeRustPlatform {
			cargo = nightlyRust.rust;
			rustc = nightlyRust.rust;
		};
		in let frontend = pkgs.mkYarnPackage {
			pname = "frontend";
			version = "0.0.0";

			src = ./frontend;

			buildPhase = "yarn --offline build";
			distPhase = "true";
		};
		in let backend = rustPlatform.buildRustPackage {
			pname = "engineering-web-portal";
			version = "0.1.0";
			buildInputs = [pkgs.sqlite];

			# disable debugging
			buildNoDefaultFeatures = true;

			src = pkgs.nix-filter {
				root = ./.;

				include = [
					"src"
					"Cargo.toml"
					"Cargo.lock"
					"migrations"
				];
			};

			cargoLock.lockFile = ./Cargo.lock;
		};
		in let fullstack = pkgs.stdenv.mkDerivation {
			name = "fullstack";
			src = frontend;

			installPhase = ''
				mkdir -p $out/bin
				cp libexec/frontend/deps/frontend/dist $out/frontend -r
				cp ${backend}/bin/engineering-web-portal $out/bin/backend
				echo "#!/bin/bash" > $out/bin/fullstack
				echo "export OVERRIDE_STATIC=$out/frontend" >> $out/bin/fullstack
				echo "export ROCKET_ADDRESS=0.0.0.0" >> $out/bin/fullstack
				echo "export ROCKET_LOG_LEVEL=normal" >> $out/bin/fullstack
				echo "$out/bin/backend" >> $out/bin/fullstack
				chmod +x $out/bin/fullstack
			'';
		};
		in {
			devShells.x86_64-linux.default = pkgs.mkShell {
				buildInputs = [
					nightlyRust.rust
					pkgs.diesel-cli
					pkgs.sqlite
					pkgs.yarn
					deploy-rs.defaultPackage.x86_64-linux
				];
			};

			packages.x86_64-linux.default = fullstack;

			deploy.nodes.aws = {
				hostname = "nhse.zerdle.net";

				profiles.system = {
					sshUser = "root";
					user = "root";

					path = deploy-rs.lib.x86_64-linux.activate.nixos (nixpkgs.lib.nixosSystem {
						system = "x86_64-linux";

						modules = [ ./ec2.nix ];
						specialArgs = {
							inherit fullstack;
						};
					});
				};
			};

			checks = builtins.mapAttrs (system: deployLib: deployLib.deployChecks self.deploy) deploy-rs.lib;
		};
}
