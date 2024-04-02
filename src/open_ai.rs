use anyhow::Result;

pub fn setup(secrets: &SecretStore) -> Result<()> {

    let openai_key = secrets
       .get("OPENAAI_API_KEY")
       .ok_or(SetupError("OPENAI key not available "))?;
    openai::set_key(openai_key);
    Ok(())
}
