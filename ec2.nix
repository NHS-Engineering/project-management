### https://nixos.org/channels/nixos-22.05 nixos
{ pkgs, ... }:
{
	imports = [ <nixpkgs/nixos/modules/virtualisation/amazon-image.nix> ];
	ec2.hvm = true;

	environment.systemPackages = with pkgs; [
		vim
		screen
	];

	networking.hostName = "nhs-engineering";
	networking.firewall = {
		enable = true;
		allowedTCPPorts = [ 8000 ];
	};

	users.mutableUsers = false;

	users.users.root = {
		isSystemUser = true;
		openssh.authorizedKeys.keys = [ "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAILXu/wKphXadjK3mgrlOEO0Ne1hkXclrt/hDoVZO+NzY james@ragnarok" ];
	};

	users.users.engineer = {
		isNormalUser = true;
	};

	system.stateVersion = "22.05";
}
