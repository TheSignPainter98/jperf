use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None, name = "jperf")]
pub struct Args {
	/// Application name
	#[arg(long)]
	app: Option<String>,

	/// Cloud name
	#[arg(long)]
	cloud: Option<String>,

	/// Unit name
	#[arg(long)]
	unit: Option<String>,

	/// Model name
	#[arg(long)]
	model: Option<String>,		

	/// Show a large set of output metrics
	#[arg(short, long)]
	verbose: bool,
	
	/// Most recent time interval in milliseconds
	#[arg(long, default_value_t = 1000)]
	interval: u32,
}

impl Args {
	#[allow(dead_code)]
	fn app(&self) -> &Option<String>{
		&self.app
	}
	
	#[allow(dead_code)]
	fn unit(&self) -> &Option<String>{
		&self.unit
	}
	
	#[allow(dead_code)]
	fn model(&self) -> &Option<String>{
		&self.model
	}

	#[allow(dead_code)]
	fn cloud(&self) -> &Option<String>{
		&self.cloud
	}

	#[allow(dead_code)]
	fn is_verbose(&self) -> &bool {
		&self.verbose
	}
	
	#[allow(dead_code)]
	fn interval(&self) -> &u32 {
		&self.interval
	}
}

#[cfg(test)]
mod test_valid {
	use super::*;

	#[test]
	fn test_app() {
		let binding = Args::try_parse_from(["jperf", "--app", "foo"]).unwrap();
		let app : &Option<String> = binding.app();
		let name : String = match app {
			None => "".to_string(),
			Some(n) => n.to_string(),
		};
		assert_eq!("foo", name);
	}

	#[test]
	fn test_unit() {
		let result = Args::try_parse_from(["jperf", "--unit", "bar"]);
		assert_eq!(result.is_ok(), true);
		let binding = result.unwrap();
		let unit : &Option<String> = binding.unit();
		let name : String = match unit {
			None => "".to_string(),
			Some(n) => n.to_string(),
		};
		assert_eq!("bar", name);
	}

	#[test]
	fn test_model() {
		let result = Args::try_parse_from(["jperf", "--model", "woo"]);
		assert_eq!(result.is_ok(), true);
		let binding = result.unwrap_or_default();
		let model : &Option<String> = binding.model();
		let name : String = match model {
			None => "".to_string(),
			Some(n) => n.to_string(),
		};
		assert_eq!("woo", name);
	}

	#[test]
	fn test_cloud() {
		let result = Args::try_parse_from(["jperf", "--cloud", "baz"]);
		assert_eq!(result.is_ok(), true);
		let binding = result.unwrap_or_default();
		let cloud : &Option<String> = binding.cloud();
		let name : String = match cloud {
			None => "".to_string(),
			Some(n) => n.to_string(),
		};
		assert_eq!("baz", name);
	}

	fn test_entity(entity: &str) -> String {
		let entity_option : String = "--".to_string() + entity;
		let result = Args::try_parse_from(["jperf", entity_option.as_str(), "baz"]);
		
		assert_eq!(result.is_ok(), true);
		let binding = result.unwrap_or_default();

		let field : &Option<String> = match entity {
			"cloud" => binding.cloud(),
			"model" => binding.model(),
			"app" => binding.app(),
			"unit" => binding.unit(),
			&_ => todo!(),
		};

		let name : String = match field {
			None => "".to_string(),
			Some(n) => n.to_string(),
		};
		name
	}

	#[test]
	fn test_cloud_better() {
		let entity : &str = "cloud";
		let result : String = test_entity(entity);
		assert_eq!("baz", result.as_str());
	}

	#[test]
	fn test_model_better() {
		let entity : &str = "model";
		let result : String = test_entity(entity);
		assert_eq!("baz", result.as_str());
	}

}


