use miette::{Context as _, IntoDiagnostic as _};

#[tokio::main]
async fn main() -> std::process::ExitCode {
    tracing_subscriber::fmt().json().init();

    match run().await {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            let causes = error
                .chain()
                .skip(1)
                .map(|error| format!("{error}"))
                .collect::<Vec<_>>();
            tracing::error!(?causes, "{error}");
            std::process::ExitCode::FAILURE
        }
    }
}

async fn run() -> miette::Result<()> {
    tracing::info!("Hello, world!");

    Err(std::io::Error::other("oh no"))
        .into_diagnostic()
        .wrap_err("uh oh")
}
