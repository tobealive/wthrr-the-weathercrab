use regex::Regex;

use crate::modules::{display::product::Product, units::Time, weather::Weather};

pub struct Times {
	pub current_hour: usize,
	pub night: bool,
	pub sunrise: String,
	pub sunset: String,
}

impl Weather {
	pub fn get_times(&self, time_unit: Time, day_index: usize) -> Times {
		let (current_hour, sunrise_hour, sunset_hour) = (
			self.current_weather.time[11..13].parse::<usize>().unwrap_or_default(),
			self.daily.sunrise[day_index][11..13].parse::<usize>().unwrap_or_default(),
			self.daily.sunset[day_index][11..13].parse::<usize>().unwrap_or_default(),
		);
		let sunrise = match time_unit {
			Time::am_pm => format!("{}:{}am", sunrise_hour, &self.daily.sunrise[day_index][14..16]),
			_ => self.daily.sunrise[day_index][11..16].to_string(),
		};
		let sunset = match time_unit {
			Time::am_pm => format!("{}:{}pm", sunset_hour - 12, &self.daily.sunset[day_index][14..16]),
			_ => self.daily.sunset[day_index][11..16].to_string(),
		};
		let night = current_hour < sunrise_hour || current_hour > sunset_hour;

		Times { current_hour, sunrise, sunset, night }
	}
}

impl Product {
	pub fn trunc_address(mut address: String, max_width: usize) -> String {
		let address_len = address.chars().count();

		address = if address_len > max_width {
			// For most locations with overly long addresses, the results seem to be better if
			// truncated between the first and second comma instead the penultimate and last comma.
			// let last_comma = title.matches(',').count();
			let prep_re = format!("^((?:[^,]*,){{{}}})[^,]*,(.*)", 1);
			let re = Regex::new(&prep_re).unwrap();

			re.replace(&address, "$1$2").to_string()
		} else {
			address
		};

		if address_len > max_width {
			address = Self::trunc_address(address, max_width);
		}

		address
	}
}

pub fn lang_len_diff(input: &str, lang: &str) -> usize {
	match &lang[..2] {
		"zh" => {
			let re = Regex::new(r"\p{Han}").unwrap();
			re.find_iter(input).count()
		}
		"ko" => {
			let re = Regex::new(r"[\u3131-\uD79D\w]").unwrap();
			let nu = Regex::new(r"[-]?\d+(\.\d+)?").unwrap();
			re.find_iter(input).count() - nu.find_iter(input).count()
		}
		"ja" => {
			let re = Regex::new(r"[ぁ-んァ-ン\w]").unwrap();
			let nu = Regex::new(r"[-]?\d+(\.\d+)?").unwrap();
			re.find_iter(input).count() - nu.find_iter(input).count()
		}
		_ => 0,
	}
}

pub fn style_number(mut num: i32, sub: bool) -> String {
	const SUPERSCRIPT_DIGITS: [char; 10] = ['⁰', '¹', '²', '³', '⁴', '⁵', '⁶', '⁷', '⁸', '⁹'];
	const SUBSCRIPT_DIGITS: [char; 10] = ['₀', '₁', '₂', '₃', '₄', '₅', '₆', '₇', '₈', '₉'];

	let mut result = String::new();

	if num == 0 {
		result.push(match sub {
			true => SUBSCRIPT_DIGITS[0],
			_ => SUPERSCRIPT_DIGITS[0],
		});
		return result;
	}

	if num < 0 {
		num = -num;
		result.push(if sub { '₋' } else { '⁻' });
	}

	let mut started = false;
	let mut power_of_ten = 1_000_000_000;
	for _ in 0..10 {
		let digit = num / power_of_ten;
		num -= digit * power_of_ten;
		power_of_ten /= 10;
		if digit != 0 || started {
			started = true;
			result.push(match sub {
				true => SUBSCRIPT_DIGITS[digit as usize],
				_ => SUPERSCRIPT_DIGITS[digit as usize],
			})
		}
	}

	result
}
