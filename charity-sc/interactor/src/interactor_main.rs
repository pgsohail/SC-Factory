use multiversx_sc_snippets::{
    imports::{ScCallStep, tokio, BytesValue, Bech32Address},
    Interactor,
};
use multiversx_sc::types::Address;

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_address: Address,
}

impl ContractInteract {
    async fn new(contract_address: &str) -> Self {
        let mut interactor = Interactor::new("https://devnet-gateway.multiversx.com").await;
        let wallet_address = interactor
            .register_wallet(multiversx_sc_snippets::test_wallets::alice())
            .await;
        let bech32_address = Bech32Address::from_bech32_string(contract_address.to_string());
        let contract_address = bech32_address.to_address();
        Self {
            interactor,
            wallet_address,
            contract_address,
        }
    }

    async fn set_project_sc_code(&mut self) {
        let project_sc_code = BytesValue::from(
            include_bytes!("../../../project-sc/output/project-sc.wasm").to_vec()
        );
        let tx = ScCallStep::new()
            .from(&self.wallet_address)
            .to(&self.contract_address)
            .function("setProjectScCode")
            .argument(&project_sc_code)
            .gas_limit(20_000_000);
        let _result = self.interactor.sc_call(tx).await;
        println!("Set ProjectSC code");
    }

    async fn create_project_sc(&mut self, _project_name: &str) {
        let tx = ScCallStep::new()
            .from(&self.wallet_address)
            .to(&self.contract_address)
            .function("createProjectSC")
            .argument("0") 
            .gas_limit(20_000_000);
        let _result = self.interactor.sc_call(tx).await;
        println!("Called createProjectSC with argument: 0 (placeholder)");
        println!("Check the Devnet Explorer for the new contract address!");
    }
}

#[tokio::main]
async fn main() {
    let mut interact = ContractInteract::new("erd1qqqqqqqqqqqqqpgqvky2e7wghjzplq2gf89r4gwsuydaz0n0d8sswwkpmt").await;
    interact.set_project_sc_code().await;
    interact.create_project_sc("TestProject").await;
}