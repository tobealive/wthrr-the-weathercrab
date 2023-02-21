pub mod gui;
pub mod units;

mod address;

use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use optional_struct::Applyable;
use serde::{Deserialize, Serialize};

use crate::modules::{
	args::Cli,
	config::Config,
	locales::{ConfigLocales, Locales},
};

use self::units::Units;

#[derive(Serialize, Deserialize, Debug)]
pub struct Params {
	pub config: Config,
	pub texts: Locales,
}

impl Params {
	pub async fn merge(mut config: Config, args: &Cli) -> Result<Self> {
		if let Some(language) = &args.language {
			config.language = language.to_string()
		}

		let texts = Locales::get(&config.language).await?;

		if args.reset {
			Self::reset(&texts.config).await?;
			std::process::exit(1);
		}

		if !args.forecast.is_empty() {
			config.forecast = args.forecast.to_vec();
		}

		config.units = Units::get(&args.units, &config.units);

		let mut params = Self { config, texts };

		params
			.resolve_address(args.address.as_deref().unwrap_or_default())
			.await?;

		Ok(params)
	}

	pub async fn handle_next(mut self, args: Cli, mut config_file: Config) -> Result<()> {
		if !args.save && !config_file.address.is_empty() {
			return Ok(());
		}

		if config_file.address.is_empty() {
			// offer to save
			self.config.apply_to(&mut config_file);
			self.config = config_file;
			self.save_prompt(&args.address.unwrap_or_default()).await?;
		} else {
			// handle explicit save call
			self.config.apply_to(&mut config_file);
			config_file.store();
			self.texts.store(&config_file.language);
		}

		Ok(())
	}

	async fn save_prompt(mut self, arg_address: &str) -> Result<()> {
		let mut items = vec![
			self.texts.config.confirm.clone(),
			self.texts.config.next_time.clone(),
			self.texts.config.deny.clone(),
		];

		if arg_address.is_empty() || arg_address == "auto" {
			items.push(self.texts.config.always_auto.clone());
		}

		let selection = Select::with_theme(&ColorfulTheme::default())
			.with_prompt(&self.texts.config.save_as_default)
			.items(&items)
			.default(0)
			.interact()?;

		match selection {
			0 => {}
			1 => return Ok(()),
			2 => self.config.address = "arg_input".to_string(),
			3 => self.config.address = "auto".to_string(),
			_ => println!("{}", self.texts.config.no_selection),
		}

		self.config.store();
		self.texts.store(&self.config.language);

		Ok(())
	}

	pub async fn reset(t: &ConfigLocales) -> Result<()> {
		let confirmation = Confirm::with_theme(&ColorfulTheme::default())
			.with_prompt(t.reset_config.clone())
			.interact()?;

		if confirmation {
			let path = Config::get_path();

			std::fs::remove_dir_all(path.parent().unwrap()).with_context(|| "Error resetting config file.")?;
		}

		Ok(())
	}
}
