use serde::{Deserialize, Serialize};

use crate::{ids, secp256k1fx};

/// ref. https://pkg.go.dev/github.com/ava-labs/avalanchego/vms/avm#FxCredential
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub struct Credential {
    pub fx_id: ids::Id, // skip serialization due to serialize:"false"
    pub cred: secp256k1fx::Credential,
}

impl Default for Credential {
    fn default() -> Self {
        Self::default()
    }
}

impl Credential {
    pub fn default() -> Self {
        Self {
            fx_id: ids::Id::empty(),
            cred: secp256k1fx::Credential::default(),
        }
    }
}
