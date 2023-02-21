mod modules;

use anyhow::Result;
use clap::Parser;

use modules::{
	args::Cli, config::Config, display::product::Product, location::Address, params::Params, weather::Weather,
};

#[tokio::main]
async fn main() -> Result<()> {
	let args = Cli::parse();
	let config = Config::get_config_file();
	let params = Params::merge(config.clone(), &args).await?;

	let product = run(&params).await?;
	product
		.render(
			&params.config.forecast,
			&params.config.units,
			&params.config.gui,
			&params.config.language,
			&params.texts.weather,
		)
		.await?;

	params.handle_next(args, config).await?;

	Ok(())
}

pub async fn run(params: &Params) -> Result<Product> {
	let loc = Address::search(&params.config.address, &params.config.language).await?;
	let (lat, lon) = (loc.lat.parse::<f64>().unwrap(), loc.lon.parse::<f64>().unwrap());

	let address = loc.name.to_string();
	let weather = Weather::get(lat, lon, &params.config.units).await?;

	Ok(Product { address, weather })
}
