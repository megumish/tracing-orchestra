use tracing_orchestra_development as tracing_orchestra;
use tracing_orchestra_macros_development as tracing_orchestra_macros;

use tracing::{info, span};
use tracing_orchestra_macros::{orchestra, Orchestra};
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Orchestra, Debug)]
struct Cat {
    name: String,
    age: u128,
}

#[orchestra(level = "warn", fields(extra = "neko"))]
impl Cat {
    pub fn new(name: String, age: u128) -> Self {
        Self { name, age }
    }

    // override tracing::instrument
    #[tracing::instrument(level = "error", fields(very_extra = "inu"))]
    // double override
    #[tracing::instrument(level = "warn", fields(very_extra = "nyan"))]
    fn meow(&self) -> String {
        let name = &self.name;
        let age = &self.age;
        format!("{name} says meow! {name}-san is {age} years old.")
    }
}

fn main() {
    tracing_subscriber::fmt::SubscriberBuilder::default()
        .with_env_filter("trace")
        .with_span_events(FmtSpan::NEW | FmtSpan::CLOSE)
        .pretty()
        .init();
    let main_span = span!(tracing::Level::INFO, "main");
    let _enter = main_span.enter();

    let mike = Cat::new("mike".to_string(), 3);
    info!("{}", mike.meow());
}
