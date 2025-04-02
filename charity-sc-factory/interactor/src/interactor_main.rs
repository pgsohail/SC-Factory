use multiversx_sc_snippets::{
    imports::{ScCallStep, ScDeployStep, tokio, BytesValue, Bech32Address},
    Interactor,
};
use multiversx_sc::types::Address;

pub struct ContractInteract {
    interactor: Interactor,
    wallet_address: Address,
    contract_address: Option<Address>,
}

impl ContractInteract {
    async fn new() -> Self {
        let mut interactor = Interactor::new("https://devnet-gateway.multiversx.com").await;
        let wallet_address = interactor
            .register_wallet(multiversx_sc_snippets::test_wallets::alice())
            .await;
        Self {
            interactor,
            wallet_address,
            contract_address: None,
        }
    }

    async fn deploy(&mut self) {
        let code = BytesValue::from(include_bytes!("../../../charity-sc/output/charity-sc.wasm").to_vec());
        let tx = ScDeployStep::new()
            .from(&self.wallet_address)
            .code(&code)
            .gas_limit(50_000_000)
            .argument("0"); 

        let _result = self.interactor.sc_deploy(tx).await;
        let bech32_address = Bech32Address::from_bech32_string(
            String::from("erd1qqqqqqqqqqqqqpgqvky2e7wghjzplq2gf89r4gwsuydaz0n0d8sswwkpmt")
        );
        let new_address = bech32_address.to_address();

        self.contract_address = Some(new_address.clone());
        println!("Deployed at: {}", bech32_address.to_string());
    }

    async fn set_project_sc_code(&mut self) {
        let address = self.contract_address.clone().unwrap();
        // i have to replace later 
        let project_code = BytesValue::from(include_bytes!("../../../project-sc/output/project-sc.wasm").to_vec());
        let tx = ScCallStep::new()
            .from(&self.wallet_address)
            .to(&address)
            .function("setProjectScCode")
            .argument(&project_code)
            .gas_limit(20_000_000);

        let _result = self.interactor.sc_call(tx).await;
        println!("Set project SC code");
    }

    async fn create_project_sc(&mut self, project_name: &str) {
        let address = self.contract_address.clone().unwrap();
        let tx = ScCallStep::new()
            .from(&self.wallet_address)
            .to(&address)
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
    let mut interact = ContractInteract::new().await;
    interact.deploy().await;
    interact.set_project_sc_code().await; //
    interact.create_project_sc("TestProject").await;
    interact.create_project_sc("AnotherProject").await;
}