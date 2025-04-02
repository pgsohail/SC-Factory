use multiversx_sc_snippets::{
    imports::{ScCallStep, tokio, Bech32Address},
    Interactor,
};
use multiversx_sc::types::Address;

pub struct ProjectInteract {
    interactor: Interactor,
    wallet_address: Address,
    project_address: Address,
}

impl ProjectInteract {
    async fn new(project_address: &str) -> Self {
        let mut interactor = Interactor::new("https://devnet-gateway.multiversx.com").await;
        let wallet_address = interactor
            .register_wallet(multiversx_sc_snippets::test_wallets::alice())
            .await;
        let bech32_address = Bech32Address::from_bech32_string(project_address.to_string());
        let project_address = bech32_address.to_address();
        Self {
            interactor,
            wallet_address,
            project_address,
        }
    }

    async fn donate(&mut self, amount: &str) {
        let tx = ScCallStep::new()
            .from(&self.wallet_address)
            .to(&self.project_address)
            .function("donate")
            .egld_value(amount) 
            .gas_limit(10_000_000);
        let _result = self.interactor.sc_call(tx).await;
        println!("Donated {} EGLD (wei) to ProjectSC", amount);
        println!("Check the Devnet Explorer for the tx hash!");
    }

    async fn get_total_donations(&mut self) {
        let tx = ScCallStep::new()
            .from(&self.wallet_address)
            .to(&self.project_address)
            .function("getTotalDonations")
            .gas_limit(5_000_000);
        let _result = self.interactor.sc_call(tx).await;
        println!("Called getTotalDonations");
        println!("Check the Devnet Explorer for the result!");
    }
}

#[tokio::main]
async fn main() {
    let mut interact = ProjectInteract::new("erd1qqqqqqqqqqqqqpgq8lasmtpz7cu0tsklh9z9030faadx8vqed8ssk7sun3").await;
    interact.donate("1000000000000000000").await;
    interact.get_total_donations().await;
}