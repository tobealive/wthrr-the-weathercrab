use anyhow::{bail, Result};
use chrono::NaiveDate;
use clap::{Parser, ValueEnum};
use clap_complete::Shell;
use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(Parser)]
#[command(author, version, about, long_about = None, next_line_help = true)]
pub struct Cli {
	/// Address to check the weather
	pub address: Option<String>,

	/// [e.g.: -f w,d]
	#[arg(long, short, use_value_delimiter = true, value_name = "FORECAST,...")]
	pub forecast: Vec<Forecast>,

	/// [e.g.: -F 2021-12-31]
	#[arg(long, short = 'F', use_value_delimiter = true, value_name = "%Y-%m-%d,...")]
	pub historical_weather: Vec<NaiveDate>,

	/// [e.g.: -u f,12h,in]
	#[arg(long, short, use_value_delimiter = true, value_name = "UNIT,...")]
	pub units: Vec<UnitArg>,

	/// Output language [e.g.: en_US]
	#[allow(clippy::doc_markdown)]
	#[arg(short, long, value_parser = parse_language_code)]
	pub language: Option<String>,

	/// Save the supplied values as default
	#[arg(short, long, group = "config_file_action")]
	pub save: bool,

	/// Wipe wthrr's configuration data
	#[arg(short, long, group = "config_file_action")]
	pub reset: bool,

	/// Generate shell completions
	#[arg(long, value_name = "SHELL")]
	pub completions: Option<Shell>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, AsRefStr, Serialize, Deserialize, Hash)]
#[allow(non_camel_case_types)]
pub enum Forecast {
	disable,
	#[value(name = "(w)eek", aliases = ["w", "week"])]
	week,
	#[value(name = "to(d)ay", aliases = ["d", "day", "today"])]
	day,
	#[value(name = "(t)omorrow", aliases = ["t", "to", "tom", "tomorrow"])]
	tomorrow,
	#[value(aliases = ["mon", "monday"])]
	mo,
	#[value(aliases = ["tue", "tuesday"])]
	tu,
	#[value(aliases = ["wed", "wednesday"])]
	we,
	#[value(aliases = ["thu", "thursday"])]
	th,
	#[value(aliases = ["fri", "friday"])]
	fr,
	#[value(aliases = ["sat", "saturday"])]
	sa,
	#[value(aliases = ["sun", "sunday"])]
	su,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, AsRefStr, Serialize, Deserialize)]
#[strum(serialize_all = "snake_case")]
pub enum UnitArg {
	// Temperature
	#[value(name = "(c)elsius", aliases = ["c", "celsius"])]
	Celsius,
	#[value(name = "(f)ahrenheit", aliases = ["f", "fahrenheit"])]
	Fahrenheit,
	// Windspeed
	Kmh,
	Mph,
	#[value(name = "(kn)ots", aliases = ["kn", "knots"])]
	// Serialize as kn for open-meteo api call
	#[strum(serialize = "kn")]
	Knots,
	Ms,
	#[value(name = "12h", alias = "am_pm")]
	AmPm,
	#[value(name = "24h", alias = "military")]
	Military,
	#[value(name = "%", alias = "probability")]
	Probability,
	Mm,
	#[value(name = "(in)ch", alias = "in")]
	Inch,
}

fn parse_language_code(s: &str) -> Result<String> {
	if s.len() < 2 {
		bail!("\n  The language code must be at least two characters long.")
	}
	Ok(s.to_string())
}
