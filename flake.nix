{
	inputs = {
		nixpkgs.url = "github:nixos/nixpkgs/release-22.05";
		nixpkgs-unstable.url = "github:nixos/nixpkgs/nixpkgs-unstable";
		mozilla.url = "github:mozilla/nixpkgs-mozilla";
		deploy-rs.url = "github:serokell/deploy-rs";
	};

	outputs = { self, nixpkgs, nixpkgs-unstable, mozilla, deploy-rs }:
		let pkgs = import nixpkgs {
			system = "x86_64-linux";
			overlays = [mozilla.overlay];
		};
		in let unstable-pkgs = import nixpkgs-unstable { system = "x86_64-linux"; };
		in let nightlyRust = (pkgs.rustChannelOf {
			date = "2022-11-07";
			channel = "nightly";
			sha256 = "sha256-Z51XyBXGzDNX5ioVr4JvBnOsD/+iIW1PVIQCdrn82jg=";
		});
		in let rustPlatform = pkgs.makeRustPlatform {
			cargo = nightlyRust.rust;
			rustc = nightlyRust.rust;
		};
		in rec {
			devShells.x86_64-linux.default = pkgs.mkShell {
				buildInputs = [
					nightlyRust.rust
					unstable-pkgs.diesel-cli
					pkgs.sqlite
					pkgs.nodejs
					deploy-rs.defaultPackage.x86_64-linux
				];
			};

			packages.x86_64-linux.default = rustPlatform.buildRustPackage {
				pname = "engineering-web-portal";
				version = "0.1.0";
				buildInputs = [pkgs.sqlite];

				# disable debugging and static stuff
				buildNoDefaultFeatures = true;

				# use production db
				buildFeatures = ["prod_db"];

				src = ./.;

				cargoLock.lockFile = ./Cargo.lock;
			};

			deploy.nodes.aws = {
				hostname = "nhse.zerdle.net";
				profiles.system = {
					sshUser = "root";
					user = "engineer";
					path = deploy-rs.lib.x86_64-linux.activate.custom packages.x86_64-linux.default ''
						screen -XS server quit || true
						ROCKET_ADDRESS=0.0.0.0 screen -L -Logfile /tmp/server.log -S server -m -d ./bin/engineering-web-portal
						curl http://nhse.zerdle.net:8000/api/projects/list
					'';
				};
			};

			checks = builtins.mapAttrs (system: deployLib: deployLib.deployChecks self.deploy) deploy-rs.lib;
		};
}
