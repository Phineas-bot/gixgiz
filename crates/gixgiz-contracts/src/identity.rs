use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::ReadinessReport;

/// Stable user-facing application name.
pub const APPLICATION_NAME: &str = "GixGiz";

/// Stable Windows application identity shared with the Flutter desktop shell.
pub const APPLICATION_ID: &str = "ai.gixgiz.desktop";

/// Version of the current contracts package and platform foundation.
pub const APPLICATION_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initial protocol generation reserved for the Task 04 transport boundary.
pub const PROTOCOL_VERSION: u32 = 1;

/// Initial schema generation for serialized Task 03 contracts.
pub const SCHEMA_VERSION: u32 = 1;

/// Application and boundary-version information exposed by the platform core.
#[derive(Clone, Debug, Eq, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct ApplicationInfo {
    /// User-facing product name.
    pub name: String,
    /// Stable application identity used by desktop packaging.
    pub application_id: String,
    /// Semantic version of the running platform core.
    pub version: String,
    /// Transport contract version expected by compatible clients.
    pub protocol_version: u32,
    /// Serialized contract schema version.
    pub schema_version: u32,
}

impl ApplicationInfo {
    /// Returns the identity and versions compiled into this workspace.
    #[must_use]
    pub fn current() -> Self {
        Self {
            name: APPLICATION_NAME.to_owned(),
            application_id: APPLICATION_ID.to_owned(),
            version: APPLICATION_VERSION.to_owned(),
            protocol_version: PROTOCOL_VERSION,
            schema_version: SCHEMA_VERSION,
        }
    }
}

/// Combined version and readiness response produced by the platform core.
#[derive(Clone, Debug, Eq, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct PlatformStatus {
    /// Identity and version information for the responding core.
    pub application: ApplicationInfo,
    /// Current readiness plus its supporting service evidence.
    pub readiness: ReadinessReport,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_application_info_uses_stable_contract_values() {
        let info = ApplicationInfo::current();

        assert_eq!(info.name, "GixGiz");
        assert_eq!(info.application_id, "ai.gixgiz.desktop");
        assert_eq!(info.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(info.protocol_version, 1);
        assert_eq!(info.schema_version, 1);
    }

    #[test]
    fn application_info_serializes_contract_field_names() {
        let value = serde_json::to_value(ApplicationInfo::current()).expect("serializes");

        assert_eq!(value["application_id"], "ai.gixgiz.desktop");
        assert_eq!(value["protocol_version"], 1);
        assert_eq!(value["schema_version"], 1);
    }
}
