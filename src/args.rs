use clap::Parser;

#[derive(Parser, Debug, Default)]
#[command(author, version, about, long_about = None, name = "jperf")]
pub struct Args {
	/// Specify a single app to monitor
	#[arg(long, value_name = "name")]
	app: Option<String>,

	/// Specify a single cloud to monitor
	#[arg(long, value_name = "name")]
	cloud: Option<String>,

	/// Specify a single app unit to monitor
	#[arg(long, value_name = "name")]
	unit: Option<String>,

	/// Specify a single model to monitor
	#[arg(long, value_name = "name")]
	model: Option<String>,		

	/// Specify a single metric to display
	#[arg(short, value_name = "name")]
	metric: Option<String>,	
	
	/// Most recent time window in milliseconds to display
	#[arg(long, default_value_t = 1000, value_name = "milliseconds")]
	interval: u32,
}

impl Args {
	#[allow(dead_code)]
	fn app(&self) -> Option<&str>{
		self.app.as_deref()
	}

	#[allow(dead_code)]
	fn unit(&self) -> Option<&str>{
		self.unit.as_deref()
	}
	
	#[allow(dead_code)]
	fn model(&self) -> Option<&str>{
		self.model.as_deref()
	}

	#[allow(dead_code)]
	fn cloud(&self) -> Option<&str>{
		self.cloud.as_deref()
	}

	#[allow(dead_code)]
	fn metric(&self) -> Option<&str>{
		self.metric.as_deref()
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
	fn entity_as_string(&self, entity : &str) -> Option<&str> {		
		match entity {
			"cloud" => self.cloud(),
			"model" => self.model(),
			"app" => self.app(),
			"unit" => self.unit(),
			_ => None,
		}
	}
}

#[cfg(test)]
mod test_valid {
	use super::*;	

	#[test]
	fn test_entity_flags() {
		for entity in ["cloud", "model", "app", "unit"] {
			let result = Args::try_parse_from(["jperf", &format!("{entity}"), "baz"]);
			assert_eq!(result.is_ok(), true);
			
			let args = result.unwrap_or_default();
			let result = match args.entity_as_string(entity) {
				None => "".to_string(),
				Some(n) => n.to_string(),
			};
			
			assert_eq!("baz", result.as_str());
		}
	}	
}
