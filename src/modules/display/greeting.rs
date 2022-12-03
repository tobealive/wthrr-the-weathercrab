use anyhow::Result;

use crate::modules::translation::translate;

pub async fn render(greet: bool, lang: &str) -> Result<()> {
	if !greet {
		return Ok(());
	}

	let greeting = translate(lang, "Hey friend. I'm glad you are asking.").await?;

	println!("  🦀  {}", greeting);

	Ok(())
}
