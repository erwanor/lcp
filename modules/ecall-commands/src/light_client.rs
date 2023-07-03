use crate::{prelude::*, EnclaveKeySelector};
use commitments::{StateCommitmentProof, UpdateClientCommitmentProof};
use crypto::Address;
use lcp_types::{Any, ClientId, Height, Time};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum LightClientCommand {
    InitClient(InitClientInput),
    UpdateClient(UpdateClientInput),

    VerifyMembership(VerifyMembershipInput),
    VerifyNonMembership(VerifyNonMembershipInput),

    QueryClient(QueryClientInput),
}

impl EnclaveKeySelector for LightClientCommand {
    fn get_enclave_key(&self) -> Option<Address> {
        match self {
            Self::InitClient(input) => Some(input.signer),
            Self::UpdateClient(input) => Some(input.signer),
            Self::VerifyMembership(input) => Some(input.signer),
            Self::VerifyNonMembership(input) => Some(input.signer),
            _ => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitClientInput {
    pub any_client_state: Any,
    pub any_consensus_state: Any,
    pub current_timestamp: Time,
    pub signer: Address,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateClientInput {
    pub client_id: ClientId,
    pub any_header: Any,
    pub include_state: bool,
    pub current_timestamp: Time,
    pub signer: Address,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyMembershipInput {
    pub client_id: ClientId,
    pub prefix: Vec<u8>,
    pub path: String,
    pub value: Vec<u8>,
    pub proof: CommitmentProofPair,
    pub signer: Address,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyNonMembershipInput {
    pub client_id: ClientId,
    pub prefix: Vec<u8>,
    pub path: String,
    pub proof: CommitmentProofPair,
    pub signer: Address,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommitmentProofPair(pub Height, pub Vec<u8>);

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryClientInput {
    pub client_id: ClientId,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum LightClientResult {
    InitClient(InitClientResult),
    UpdateClient(UpdateClientResult),

    VerifyMembership(VerifyMembershipResult),
    VerifyNonMembership(VerifyNonMembershipResult),

    QueryClient(QueryClientResult),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitClientResult {
    pub client_id: ClientId,
    pub proof: UpdateClientCommitmentProof,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
pub struct UpdateClientResult(pub UpdateClientCommitmentProof);

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyMembershipResult(pub StateCommitmentProof);

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifyNonMembershipResult(pub StateCommitmentProof);

#[derive(Serialize, Deserialize, Debug)]
pub struct QueryClientResult {
    pub any_client_state: Any,
    pub any_consensus_state: Any,
}
