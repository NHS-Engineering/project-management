{ pkgs, modulesPath, fullstack, ... }:
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
		allowedTCPPorts = [ 8000 ];
	};

	users.mutableUsers = false;

	users.users = {
		root = {
			isSystemUser = true;
			openssh.authorizedKeys.keys = [ "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILXu/wKphXadjK3mgrlOEO0Ne1hkXclrt/hDoVZO+NzY james@ragnarok" ];
		};

		engineer = {
			isNormalUser = true;
		};
	};

	systemd.services.fullstack = {
		wantedBy = [ "multi-user.target" ];
		after = [ "network.target" ];

		path = [ fullstack ];
		environment = {
			OVERRIDE_DB = "file:/home/engineer/db.sqlite";
		};
		script = "fullstack";
		serviceConfig.User = "engineer";
	};

	system.stateVersion = "22.05";
}
