#![no_std]

use multiversx_sc::imports::*;

#[multiversx_sc::contract]
pub trait ProjectSC {
    #[init]
    fn init(&self, project_name: ManagedBuffer, org_name: ManagedBuffer) {
        self.project_name().set(project_name);
        self.org_name().set(org_name);
    }

    #[payable("EGLD")]
    #[endpoint(donate)]
    fn donate(&self) {
        let payment = self.call_value().egld().clone_value();
        self.total_donations().update(|total| *total += payment);
    }

    #[view(getProjectInfo)]
    fn get_project_info(&self) -> MultiValue2<ManagedBuffer, BigUint> {
        (self.project_name().get(), self.total_donations().get()).into()
    }

    #[storage_mapper("projectName")]
    fn project_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("orgName")]
    fn org_name(&self) -> SingleValueMapper<ManagedBuffer>;

    #[storage_mapper("totalDonations")]
    fn total_donations(&self) -> SingleValueMapper<BigUint>;
}