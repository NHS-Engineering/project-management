{ pkgs, modulesPath, domain, fullstack, ... }:
{
	imports = [ (modulesPath + "/virtualisation/amazon-image.nix") ];
	ec2.hvm = true;

	environment.systemPackages = with pkgs; [
		vim
		htop
	];

	networking.hostName = "nhs-engineering";
	networking.firewall = {
		enable = true;
		allowedTCPPorts = [ 80 443 ];
	};

	users.mutableUsers = false;

	users.users = {
		root = {
			isSystemUser = true;
			openssh.authorizedKeys.keys = [ "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILXu/wKphXadjK3mgrlOEO0Ne1hkXclrt/hDoVZO+NzY james@ragnarok" ];
		};

		engineer.isNormalUser = true;
	};

	security.acme = {
		defaults.email = (pkgs.lib.strings.fileContents ./.letsencrypt_email.txt);
		acceptTerms = (domain == "nhse.zerdle.net");
	};

	services.nginx = {
		enable = true;
		recommendedProxySettings = true;
		recommendedTlsSettings = true;

		virtualHosts."${domain}" = {
			enableACME = true;
			forceSSL = true;

			locations."/" = {
				proxyPass = "http://127.0.0.1:8000";
			};
		};
	};

	systemd.services.fullstack = {
		before = [ "nginx.service" ];

		environment = {
			OVERRIDE_DB = "file:/home/engineer/db.sqlite";
		};
		serviceConfig = {
			User = "engineer";
			ExecStart = "${fullstack}/bin/fullstack";
		};
	};

	system.stateVersion = "22.05";
}
