use warframe::worldstate::{
    Client,
    Opposite,
    TimedEvent,
    WorldstateError,
    queryable::Cetus,
};

#[tokio::main]
async fn main() -> Result<(), WorldstateError> {
    let client = Client::default();

    let cetus = client.fetch::<Cetus>().await?;
    println!(
        "It is currently {} on cetus. It will be {} in {}",
        cetus.state,
        cetus.state.opposite(),
        cetus.eta()
    );

    Ok(())
}
