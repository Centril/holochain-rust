use crate::{conductor::Conductor, NEW_RELIC_LICENSE_KEY};
use holochain_core::state_dump::StateDump;
use holochain_core_types::error::HolochainError;
use holochain_persistence_api::cas::content::Address;

pub trait ConductorDebug {
    fn running_instances(&self) -> Result<Vec<String>, HolochainError>;
    fn state_dump_for_instance(&self, instance_id: &String) -> Result<StateDump, HolochainError>;
    fn get_type_and_content_from_cas(
        &self,
        address: &Address,
        instance_id: &String,
    ) -> Result<(String, String), HolochainError>;
}

#[holochain_tracing_macros::newrelic_autotrace(HOLOCHAIN_CONDUCTOR_LIB)]
impl ConductorDebug for Conductor {
    fn running_instances(&self) -> Result<Vec<String>, HolochainError> {
        Ok(self.instances.keys().cloned().collect())
    }

    fn state_dump_for_instance(&self, instance_id: &String) -> Result<StateDump, HolochainError> {
        let hc = self.instances.get(instance_id)?;
        Ok(hc.read().unwrap().get_state_dump()?)
    }

    fn get_type_and_content_from_cas(
        &self,
        address: &Address,
        instance_id: &String,
    ) -> Result<(String, String), HolochainError> {
        let hc = self.instances.get(instance_id)?;
        Ok(hc.read().unwrap().get_type_and_content_from_cas(address)?)
    }
}
