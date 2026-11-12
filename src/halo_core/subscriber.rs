use std::net::SocketAddr;
use moq_native_ietf::{quic, reexports::anyhow::Context, tls};
use url::Url;
use moq_transport::{serve, session::{Subscriber}};
use std::time::Duration;
use anyhow::{Result};
use tracing::{event, Level};

/// Connect to a MoQ publisher over raw QUIC, subscribe to a stream
/// and output video to stdout.
pub async fn subscribe(addr: SocketAddr) -> Result<()> {
    // The TLS config we load disables verification entirely.
    // This way, self-signed certs are valid. We'll take care of TLS later.
    let tls = tls::Args {disable_verify: true, ..Default::default()}.load()?;

    // Make a raw QUIC endpoint. We'll bind it to localhost port "0" which means any random (high) ephemeral port
    let bind: SocketAddr = if addr.is_ipv4() {
        "0.0.0.0:0".parse()?
    } else {
        "[::]:0".parse()?
    };
    let quic_config = quic::Config::new(bind, None, tls).context("QUIC config creation failed")?;
    let endpoint = quic::Endpoint::new(quic_config).context("QUIC endpoint creation failed")?;

    // moqt:// is for raw QUIC, https:// would be for WebTransport
    let url = Url::parse(&format!("moqt://{addr}"))?;
    event!(Level::INFO, "connecting to {url}");

    // Try to connect to the moqt:// url
    let (session, cid, transport) =
        endpoint.client.connect(&url, None).await.with_context(|| {
            format!("failed to connect")
        })?;
    event!(Level::INFO, "connected, cid={cid}");

    let (session, subscriber) = Subscriber::connect(session, transport).await.context("MoQ setup failed")?;

    // For now all video we recieve will be outputted to standard output.
    // Right now the display setup will be to pipe halod's stdout to
    // ffplay, which will display the video in a new window.
    // Soon(tm) we want to have some kind of IPC with the Opsys frontend
    // and give the video there. 
    let output = Box::new(tokio::io::stdout());

    let receive = async move {
        // The publisher announces its namespace right after setup. Wait for it, accept it,
        // and subscribe to that namespace.
        let mut announces = subscriber.clone();
        // moq-transport gives us a good abstraction to just fetch the last published namespace
        let mut announced = tokio::time::timeout(Duration::from_secs(10), announces.published_namespace())
            .await
            .context("timed out waiting for the publisher to announce a stream")?
            .context("connection closed before the publisher announced a stream")?;
        announced.ok().context("failed to accept the announce")?;

        let namespace = announced.info.namespace.clone();
        tracing::info!(%namespace, "publisher announced stream; receiving");

        let tracks = serve::Tracks::new(namespace);
        let mut media = moq_sub::media::Media::new(subscriber, tracks, output, false).await?;
        let res = media.run().await.context("media error");
        drop(announced); // held until here so the announce stays accepted
        res
    };

    tokio::select! {
        res = session.run() => res.context("session error"),
        res = receive => res,
        _ = tokio::signal::ctrl_c() => Ok(()),
    }
}