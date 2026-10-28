use candid::{CandidType, Deserialize};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MinistryState {
    pub ministry_id: u32,
    pub ministry_name: String,
    pub project_name: String,
    pub strategy: String,
    pub target_chain: String,
    pub is_active: bool,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AuditRecord {
    pub record_id: String,
    pub timestamp: u64,
    pub state_hash: String,
    pub verified: bool,
}

pub fn get_metadata() -> MinistryState {
    MinistryState {
        ministry_id: 11,
        ministry_name: "Ministry of Corporate Affairs".to_string(),
        project_name: "MCA21 Web3 AI".to_string(),
        strategy: "Soulbound Verifiable Credential -- Corporate Legal Entity Identifier".to_string(),
        target_chain: "Internet Computer (ICP) + Base L2".to_string(),
        is_active: true,
    }
}
