use super::CurrentPairSigner;
use crate::chain::metadata::edu_chain;
use crate::chain::metadata::edu_chain::runtime_types::sp_runtime::DispatchError;
use crate::chain::EduchainOnlineClient;
use crate::interface::{self, *};

pub type IntakeId = edu_chain::runtime_types::types::intake::IntakeId<university::UniversityId>;
pub type IntakeInfo = edu_chain::runtime_types::types::intake::IntakeInfo<interface::BlockNumber>;
pub type IntakeStatus = edu_chain::runtime_types::types::intake::IntakeStatus;
pub type IntakeApplication =
    edu_chain::runtime_types::types::intake::IntakeApplication<interface::BlockNumber>;
pub type PalletEvent = edu_chain::intake::Event;
type StudentId = crate::chain::AccountId;
type ExtrinsicResult = crate::interface::ExtrinsicResult<Error>;

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

/// Error type in intake
// Reflection of: https://github.com/metallic-foundation/edu-chain/blob/main/pallets/intake/src/lib.rs
pub enum Error {
    /// Cannot perform this action due to mismatched permission
    InsufficientPermission,
    /// Intake already exists
    IntakeExists,
    /// No such university
    NonExistentUniversity,
    /// Invalid parameter
    InvalidParamater,
    /// Intake does not exists
    NonExistentIntake,
    /// Intake closed
    IntakeClosed,
    /// Intake is still ongoing
    IntakeOngoing,
    /// application does not exsist
    NonExistentApplication,
    /// Intake is not closed
    IntakeNotClosed,
}

impl TryFrom<DispatchError> for Error {
    type Error = String;

    fn try_from(dispatch_error: DispatchError) -> Result<Self, Self::Error> {
        match dispatch_error {
            _ => todo!(),
        }
    }
}
