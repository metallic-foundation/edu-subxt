use super::CurrentPairSigner;
use crate::chain::metadata::edu_chain;
use crate::chain::EduchainOnlineClient;
use crate::interface::{self, *};

pub type IntakeId = edu_chain::runtime_types::types::intake::IntakeId<university::UniversityId>;
pub type IntakeInfo = edu_chain::runtime_types::types::intake::IntakeInfo<interface::BlockNumber>;
pub type IntakeStatus = edu_chain::runtime_types::types::intake::IntakeStatus;
pub type IntakeApplication =
    edu_chain::runtime_types::types::intake::IntakeApplication<interface::BlockNumber>;
pub type PalletEvent = edu_chain::intake::Event;
type StudentId = crate::chain::AccountId;

#[async_trait::async_trait]
pub trait IntakeCalls {
    /// Get intake info
    fn get_intake_info(&self, intake_id: &IntakeId) -> RequestResult<IntakeInfo>;

    /// Get application info
    fn get_application_info(
        &self,
        intake_id: &IntakeId,
        student_id: &StudentId,
    ) -> RequestResult<IntakeApplication>;

    /// Submit new application for specified intake
    fn submit_application(
        &self,
        sender: &CurrentPairSigner,
        intake_id: &IntakeId,
        application: &IntakeApplication,
    ) -> ExtrinsicResult;
}

#[async_trait::async_trait]
impl IntakeCalls for EduchainOnlineClient {
    fn get_intake_info(&self, intake_id: &IntakeId) -> RequestResult<IntakeInfo> {
        todo!()
    }

    fn get_application_info(
        &self,
        intake_id: &IntakeId,
        student_id: &StudentId,
    ) -> RequestResult<IntakeApplication> {
        todo!()
    }

    fn submit_application(
        &self,
        sender: &CurrentPairSigner,
        intake_id: &IntakeId,
        application: &IntakeApplication,
    ) -> ExtrinsicResult {
        todo!()
    }
}
