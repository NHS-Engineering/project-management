#[cfg(not(windows))]
static NPM_COMMAND: &'static str = "npm";

#[cfg(windows)]
static NPM_COMMAND: &'static str = "npm.cmd";

fn main() {
	println!("cargo:rerun-if-changed=migrations");

	if cfg!(feature = "rebuild-static") {
		println!("cargo:rerun-if-changed=frontend");

		let output = std::process::Command::new(NPM_COMMAND)
			.args(["run", "--prefix", "frontend", "build"])
			.output().expect("failed to run npm");

		if !output.status.success() {
			panic!("failed to build frontend");
		}
	}
}
