use std::path;
use std::fs;
use serde::Deserialize;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn example_tests() -> TestResult {
	test_source_files("examples")
}

fn test_source_files<P: AsRef<path::Path>>(directory: P) -> TestResult {
	std::env::set_current_dir(path::Path::new(env!("CARGO_MANIFEST_DIR")))?;
	let test_config_files = fs::read_dir(&directory)? // Get the given directory
		.filter_map(|entry| // For each file, keep if
			entry.as_ref()
				.ok()
				.filter(|entry|
					entry.file_name()
						.into_string()
						.unwrap_or_else(|_| "".into())
						.ends_with(".toml")
				)
				.map(|entry| {
					entry.path()
				})
		);

	let test_configs = test_config_files
		.map(|file| {
			let file_contents = &fs::read(&file).unwrap_or_else(
				|_| panic!("Can't read file: {}", file.as_path().to_string_lossy())
			);
			let file_contents = String::from_utf8_lossy(file_contents);
			
			toml::from_str::<TestConfig>(&file_contents).unwrap_or_else(
				|_| panic!("Invalid file contents: {}", file.as_path().to_string_lossy())
			)
		});
	
	test_configs.for_each(|config| {
		todo!("Run the source file and compare the output with the expected output")
	});
	
	Ok(())
}

#[derive(Deserialize)]
struct TestConfig {
	source_file: String,
	expected_out: String,
}