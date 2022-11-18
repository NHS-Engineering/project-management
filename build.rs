fn main() {
	println!("cargo:rerun-if-changed=migrations");

	if cfg!(feature = "rebuild-static") {
		println!("cargo:rerun-if-changed=frontend");

		let output = std::process::Command::new("yarn")
			.args(["--cwd", "frontend", "run", "build"])
			.output().expect("failed to run yarn");

		if !output.status.success() {
			panic!("failed to build frontend");
		}
	}
}
