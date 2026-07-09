use bdk_testenv::anyhow;
use bdk_testenv::TestEnv;
use corepc_client::client_async::{Auth, Client};

/// This trait is used for testing. It allows creating a new [`Client`] connected
/// to the instance of bitcoind running in the test environment.
pub trait ClientExt {
    /// Creates a new [`Client`] connected to the current node instance.
    fn get_rpc_client(&self) -> anyhow::Result<Client>;
}

impl ClientExt for TestEnv {
    fn get_rpc_client(&self) -> anyhow::Result<Client> {
        Ok(Client::new_with_auth(
            &self.bitcoind.rpc_url(),
            Auth::CookieFile(self.bitcoind.params.cookie_file.clone()),
        )?)
    }
}
