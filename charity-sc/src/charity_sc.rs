#![no_std]

use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait CharitySC {
    #[init]
    fn init(&self, org_name: ManagedBuffer) {
        self.org_name().set(org_name);
    }

    #[only_owner]
    #[endpoint(createProjectSC)]
    fn create_project_sc(&self, project_name: ManagedBuffer) -> ManagedAddress {
        let code = self.project_sc_code().get();
        require!(!code.is_empty(), "Project SC code not set");
        let gas = self.blockchain().get_gas_left() / 2;

        let mut args = ManagedArgBuffer::new();
        args.push_arg(&project_name);
        args.push_arg(&self.org_name().get());

        let (new_address, _) = self.send_raw().deploy_contract(
            gas,
            &BigUint::zero(),
            &code,
            CodeMetadata::DEFAULT,
            &args
        );

        self.project_contracts().insert(new_address.clone());
        new_address
    }

    #[view(getProjectContracts)]
    fn get_project_contracts(&self) -> MultiValueEncoded<ManagedAddress> {
        self.project_contracts().iter().collect()
    }

    #[only_owner]
    #[endpoint(setProjectScCode)]
    fn set_project_sc_code(&self, code: ManagedBuffer) {
        self.project_sc_code().set(code);
    }

    #[storage_mapper("orgName")]
    fn org_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("projectContracts")]
    fn project_contracts(&self) -> SetMapper<ManagedAddress>;

    #[storage_mapper("projectScCode")]
    fn project_sc_code(&self) -> SingleValueMapper<ManagedBuffer>;
}