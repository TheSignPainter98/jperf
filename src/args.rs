use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None, name = "jperf")]
pub struct Args {
	/// Filter results to just one app to monitor
	#[arg(long, value_name = "application name")]
	app: Option<String>,

	/// Specify the Juju cloud where apps to monitor are deployed at
	#[arg(long, value_name = "Juju cloud substrate")]
	cloud: Option<String>,

	/// Specify a single application unit to monitor
	#[arg(long, value_name = "application unit name")]
	unit: Option<String>,

	/// Specify the Juju model name where apps to monitor are at
	#[arg(long, value_name = "Juju model")]
	model: Option<String>,		

	/// Specify a single metric to display
	#[arg(short, value_name = "metric name")]
	metric: Option<String>,	
	
	/// Most recent time window in milliseconds to display
	#[arg(long, default_value_t = 1000, value_name = "time interval")]
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
	fn interval(&self) -> &u32 {
		&self.interval
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
impl Args {
	#[allow(dead_code)]
	fn entity_as_string(&self, entity : &str) -> &Option<String> {		
		match entity {
			"cloud" => self.cloud(),
			"model" => self.model(),
			"app" => self.app(),
			"unit" => self.unit(),
			&_ => &None,
		}
	}
}

#[cfg(test)]
mod test_valid {
	use super::*;	

	fn entity_test_args(entity : &str) -> Args {
		let result = Args::try_parse_from(["jperf", &format!("--{entity}"), "baz"]);
		assert_eq!(result.is_ok(), true);
		result.unwrap_or_default()
	}
	
	fn entity_name(entity : &str) -> String {
		let args = entity_test_args(entity);
		match args.entity_as_string(entity) {
			None => "".to_string(),
			Some(n) => n.to_string(),
		}
	}

	fn test_entity_name(entity : &str) {
		let result = entity_name(entity);
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
