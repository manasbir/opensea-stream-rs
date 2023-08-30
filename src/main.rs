use opensea_stream::{Network, Collection, subscribe_to, client, schema::Payload};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut client = client(Network::Mainnet, "91ebfacb37984d9b926f59a045fb9a9c").await;

    let (_, subscription) = subscribe_to(&mut client, Collection::All).await?;

    let stream = BroadcastStream::new(subscription);

    let stream = stream.filter_map(|event| {
        let event = event.ok()?.into_custom_payload()?;
        match event.payload {
            Payload::CollectionOffer(event) => {println!("got offer"); Some(Payload::CollectionOffer(event))},
            Payload::TraitOffer(event) => {Some(Payload::TraitOffer(event))},
            // Payload::ItemMetadataUpdated(event) => {Some(Payload::ItemMetadataUpdated(event))},
            _ => {None}
        }
    });

    let mut stream = Box::pin(stream);

    while let Some(event) = stream.next().await {
        println!("{:?}", event);
    }

    Ok(())
}