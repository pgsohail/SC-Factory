#![no_std]

use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait CharitySCFactory {
    #[init]
    fn init(&self, charity_sc_code: ManagedBuffer) {
        self.charity_sc_code().set(&charity_sc_code);
    }

    #[only_owner]
    #[endpoint(createCharitySC)]
    fn create_charity_sc(&self, org_name: ManagedBuffer) -> ManagedAddress {
        let code = self.charity_sc_code().get();
        let gas = self.blockchain().get_gas_left() / 2;

        let mut args = ManagedArgBuffer::new();
        args.push_arg(&org_name);

        let (new_address, _) = self.send_raw().deploy_contract(
            gas,        
            &BigUint::zero(),
            &code,          
            CodeMetadata::DEFAULT, 
            &args            
        );

        self.charity_contracts().insert(new_address.clone());
        new_address
    }

    #[view(getCharityContracts)]
    fn get_charity_contracts(&self) -> MultiValueEncoded<ManagedAddress> {
        self.charity_contracts().iter().collect()
    }

    #[storage_mapper("charityContracts")]
    fn charity_contracts(&self) -> SetMapper<ManagedAddress>;

    #[storage_mapper("charityScCode")]
    fn charity_sc_code(&self) -> SingleValueMapper<ManagedBuffer>;
}