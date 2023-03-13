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

	/// Display a large set of metrics
	#[arg(short, long)]
	verbose: bool,

	/// Optional choice of a single metric to display
	#[arg(short, long)]
	metric: Option<String>,	
	
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
	fn metric(&self) -> &Option<String>{
		&self.metric
	}

	#[allow(dead_code)]
	fn is_verbose(&self) -> &bool {
		&self.verbose
	}
	
	#[allow(dead_code)]
	fn interval(&self) -> &u32 {
		&self.interval
	}

	#[allow(dead_code)]
	fn entity_as_string(&self, entity : &str) -> String {		
		let field : &Option<String> = match entity {
			"cloud" => self.cloud(),
			"model" => self.model(),
			"app" => self.app(),
			"unit" => self.unit(),
			&_ => todo!(),
		};

		match field {
			None => "".to_string(),
			Some(n) => n.to_string(),
		}
	}

	#[allow(dead_code)]
	fn metric_as_string(&self) -> String {
		match self.metric() {
			None => "".to_string(),
			Some(n) => n.to_string(),
		}
	}
}

#[cfg(test)]
mod test_valid {
	use super::*;	

	fn entity_test_args(entity : &str) -> Args {
		let entity_option : String = "--".to_string() + entity;
		let result = Args::try_parse_from(["jperf", entity_option.as_str(), "baz"]);
		assert_eq!(result.is_ok(), true);
		result.unwrap_or_default()
	}
	
	fn entity_name(entity : &str) -> String {
		let args = entity_test_args(entity);
		args.entity_as_string(entity)
	}

	fn test_entity_name(entity : &str) {
		let result : String = entity_name(entity);
		assert_eq!("baz", result.as_str());
	}

	#[test]
	fn test_entity_names() {
		let entities = ["cloud", "model", "app", "unit"];
		for entity in entities {
			test_entity_name(entity);
		}
	}	
}

