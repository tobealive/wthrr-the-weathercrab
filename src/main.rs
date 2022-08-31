use anyhow::Result;
use clap::Parser;

use modules::*;
mod modules;

use {args::Args, config::Config, location::Geolocation, weather::Weather};

pub struct Product {
	weather: Weather,
	address: String,
}

#[tokio::main]
async fn main() -> Result<()> {
	let args = Args::parse();
	let config: Config = confy::lib::load("weathercrab", "wthrr")?;

	if args.reset_config {
		Config::reset()?;
		return Ok(());
	}

	greeting(config.greeting.unwrap())?;

	let params = params::get(&args, &config).await?;
	let product = run(&params).await?;
	display::render(&product, args.forecast)?;

	config.handle_next(args, params)?;

	Ok(())
}

pub async fn run(params: &Config) -> Result<Product> {
	let loc = Geolocation::search(params.address.as_ref().unwrap(), params.language.as_ref().unwrap()).await?;
	let (lat, lon) = (loc[0].lat.parse::<f64>().unwrap(), loc[0].lon.parse::<f64>().unwrap());

	let product = Product {
		weather: Weather::get(lat, lon, params.unit.as_ref().unwrap()).await?,
		address: loc[0].display_name.to_string(),
	};

	Ok(product)
}

fn greeting(include: bool) -> Result<()> {
	if !include {
		return Ok(());
	}

	println!("  🦀  Hey friend. I'm glad you are asking.");

	Ok(())
}
