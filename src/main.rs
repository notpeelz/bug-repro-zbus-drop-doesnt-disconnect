use eyre::Result;
use tracing::metadata::LevelFilter;
use tracing::{error, info};
use tracing_subscriber::{util::SubscriberInitExt, EnvFilter};
use zbus::{dbus_interface, Connection};

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt()
    .with_env_filter(
      EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()),
    )
    .finish()
    .init();

  if let Err(err) = listen_dbus().await {
    error!("something bad happened: {err}");
  }
}

#[derive(Default)]
pub(crate) struct MyInterface {}

#[dbus_interface(name = "dev.peelz.FooBar.Baz")]
impl MyInterface {
  fn do_thing(&mut self) {
    info!("doing the thing");
  }
}

async fn listen_dbus() -> Result<()> {
  let connection = Connection::session().await?;
  connection
    .request_name("dev.peelz.foobar")
    .await?;
  {
    let object_server = connection.object_server();

    const PATH: &str = "/dev/peelz/FooBar";
    object_server
      .at(PATH, MyInterface::default())
      .await?;

    connection
      .request_name("dev.peelz.foobar")
      .await?;
  }

  drop(connection);
  info!("dropped!");

  std::future::pending::<()>().await;
  unreachable!();
}
