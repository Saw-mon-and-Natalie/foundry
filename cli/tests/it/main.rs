#[cfg(not(feature = "external-integration-tests"))]
mod cast;
#[cfg(not(feature = "external-integration-tests"))]
mod cmd;
#[cfg(not(feature = "external-integration-tests"))]
mod config;
#[cfg(not(feature = "external-integration-tests"))]
mod test;
#[cfg(not(feature = "external-integration-tests"))]
mod verify;

#[cfg(feature = "external-integration-tests")]
mod integration;

fn main() {}
