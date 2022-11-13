use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum::VariantNames;
use strum_macros::{AsRefStr, EnumString, EnumVariantNames};

use crate::args::ArgUnits;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct Units {
	pub temperature: Option<Temperature>,
	pub speed: Option<Speed>,
	pub time: Option<Time>,
	pub precipitation: Option<Precipitation>,
}

impl Default for Units {
	fn default() -> Self {
		Self {
			temperature: Some(Temperature::default()),
			speed: Some(Speed::default()),
			time: Some(Time::default()),
			precipitation: Some(Precipitation::default()),
		}
	}
}

#[derive(
	Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, AsRefStr, EnumVariantNames, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum Temperature {
	#[default]
	celsius,
	fahrenheit,
}

#[derive(
	Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, AsRefStr, EnumVariantNames, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum Speed {
	#[default]
	kmh,
	mph,
	knots,
	ms,
}

#[derive(
	Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, AsRefStr, EnumVariantNames, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum Time {
	am_pm,
	#[default]
	military,
}

#[derive(
	Default, Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize, AsRefStr, EnumVariantNames, EnumString,
)]
#[allow(non_camel_case_types)]
pub enum Precipitation {
	#[default]
	mm,
	inch,
}

pub fn get(arg_units: &[ArgUnits], cfg_units: &Units) -> Result<Units> {
	let mut units = assign_arg_units(arg_units)?;

	units.temperature = evaluate_unit(units.temperature, cfg_units.temperature, Temperature::default());
	units.speed = evaluate_unit(units.speed, cfg_units.speed, Speed::default());
	units.time = evaluate_unit(units.time, cfg_units.time, Time::default());
	units.precipitation = evaluate_unit(units.precipitation, cfg_units.precipitation, Precipitation::default());

	Ok(units)
}

fn evaluate_unit<T>(arg_unit: Option<T>, cfg_unit: Option<T>, fallback_unit: T) -> Option<T> {
	match arg_unit {
		Some(unit) => Some(unit), // Some(u) => Some(u + 1),
		None => match cfg_unit {
			Some(unit) => Some(unit), // Some(u) => Some(u + 1),
			_ => Some(fallback_unit),
		},
	}
}

pub fn assign_arg_units(arg_units: &[ArgUnits]) -> Result<Units> {
	let mut units = Units {
		temperature: None,
		speed: None,
		time: None,
		precipitation: None,
	};

	for val in arg_units {
		if Temperature::VARIANTS.as_ref().contains(&val.as_ref()) {
			units.temperature = Some(Temperature::from_str(val.as_ref()).unwrap())
		}
		if Speed::VARIANTS.as_ref().contains(&val.as_ref()) {
			units.speed = Some(Speed::from_str(val.as_ref()).unwrap())
		}
		if Time::VARIANTS.as_ref().contains(&val.as_ref()) {
			units.time = Some(Time::from_str(val.as_ref()).unwrap())
		}
		if Precipitation::VARIANTS.as_ref().contains(&val.as_ref()) {
			units.precipitation = Some(Precipitation::from_str(val.as_ref()).unwrap())
		}
	}

	Ok(units)
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn units_from_args() -> Result<()> {
		let arg_units = [ArgUnits::Fahrenheit, ArgUnits::Mph, ArgUnits::AmPm, ArgUnits::Inch];
		let cfg_units = Units {
			temperature: Some(Temperature::celsius),
			speed: Some(Speed::kmh),
			time: Some(Time::military),
			precipitation: Some(Precipitation::mm),
		};

		assert_eq!(
			get(&arg_units, &cfg_units)?,
			Units {
				temperature: Some(Temperature::fahrenheit),
				speed: Some(Speed::mph),
				time: Some(Time::am_pm),
				precipitation: Some(Precipitation::inch),
			}
		);

		Ok(())
	}

	#[test]
	fn units_from_cfg() -> Result<()> {
		let arg_units = [];
		let cfg_units = Units {
			temperature: Some(Temperature::fahrenheit),
			speed: Some(Speed::knots),
			time: Some(Time::am_pm),
			precipitation: Some(Precipitation::inch),
		};

		assert_eq!(get(&arg_units, &cfg_units)?, cfg_units);

		Ok(())
	}

	#[test]
	fn units_split_from_args_cfg() -> Result<()> {
		let arg_units = [ArgUnits::Fahrenheit, ArgUnits::AmPm];
		let cfg_units = Units {
			temperature: Some(Temperature::celsius),
			speed: Some(Speed::ms),
			time: None,
			precipitation: Some(Precipitation::inch),
		};

		assert_eq!(
			get(&arg_units, &cfg_units)?,
			Units {
				temperature: Some(Temperature::fahrenheit),
				speed: cfg_units.speed,
				time: Some(Time::am_pm),
				precipitation: cfg_units.precipitation,
			}
		);

		Ok(())
	}

	#[test]
	fn units_fallback() -> Result<()> {
		let arg_units = [];
		let cfg_units = Units::default();

		assert_eq!(get(&arg_units, &cfg_units)?, Units::default());

		Ok(())
	}
}
