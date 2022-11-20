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
		in let frontend = pkgs.mkYarnPackage {
			pname = "frontend";
			version = "0.0.0";

			src = ./frontend;

			buildPhase = "yarn --offline build";
			distPhase = "true";
		};
		in let backend = use_prod_db: rustPlatform.buildRustPackage {
			pname = "engineering-web-portal";
			version = "0.1.0";
			buildInputs = [pkgs.sqlite];

			# disable debugging
			buildNoDefaultFeatures = true;

			# use production db
			buildFeatures = pkgs.lib.optional use_prod_db "prod_db";

			src = ./.;

			cargoLock.lockFile = ./Cargo.lock;
		};
		in let fullstack = use_prod_db: pkgs.stdenv.mkDerivation {
			name = "fullstack";
			src = frontend;

			installPhase = ''
				mkdir -p $out/bin
				cp libexec/frontend/deps/frontend/dist $out/frontend -r
				cp ${backend use_prod_db}/bin/engineering-web-portal $out/bin/backend
				echo "#!/bin/bash" > $out/bin/fullstack
				echo "export OVERRIDE_STATIC=$out/frontend" >> $out/bin/fullstack
				echo "export ROCKET_ADDRESS=0.0.0.0" >> $out/bin/fullstack
				echo "$out/bin/backend" >> $out/bin/fullstack
				chmod +x $out/bin/fullstack
			'';
		};
		in {
			devShells.x86_64-linux.default = pkgs.mkShell {
				buildInputs = [
					nightlyRust.rust
					unstable-pkgs.diesel-cli
					pkgs.sqlite
					pkgs.yarn
					deploy-rs.defaultPackage.x86_64-linux
				];
			};

			packages.x86_64-linux.default = fullstack false;

			deploy.nodes.aws = {
				hostname = "nhse.zerdle.net";
				profiles.system = {
					sshUser = "root";
					user = "engineer";
					path = deploy-rs.lib.x86_64-linux.activate.custom (fullstack true) ''
						screen -XS server quit || true
						screen -L -Logfile /tmp/server.log -S server -m -d ./bin/fullstack
						curl -s http://nhse.zerdle.net:8000
						curl -s http://nhse.zerdle.net:8000/api/projects/list
					'';
				};
			};

			checks = builtins.mapAttrs (system: deployLib: deployLib.deployChecks self.deploy) deploy-rs.lib;
		};
}
