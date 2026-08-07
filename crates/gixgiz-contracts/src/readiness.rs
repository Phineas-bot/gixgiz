use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Overall readiness calculated from explicit service evidence.
#[derive(Clone, Copy, Debug, Eq, JsonSchema, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ReadinessStatus {
    /// Every mandatory service is healthy and no service is impaired.
    Ready,
    /// The core is usable, but at least one capability is impaired.
    Degraded,
    /// A mandatory service is not currently available.
    Unavailable,
    /// A mandatory service reported an explicit failure.
    Failed,
    /// A newer peer supplied an unrecognized readiness value.
    #[serde(other)]
    Unknown,
}

/// Health evidence reported by one provider-neutral service.
#[derive(Clone, Copy, Debug, Eq, JsonSchema, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ServiceHealthStatus {
    /// The service passed its current health check.
    Healthy,
    /// The service is available with reduced capability.
    Degraded,
    /// The service cannot currently be reached or used.
    Unavailable,
    /// The service reported an explicit failure.
    Failed,
    /// A newer peer supplied an unrecognized health value.
    #[serde(other)]
    Unknown,
}

/// Whether a service is required for readiness or only improves capability.
#[derive(Clone, Copy, Debug, Eq, JsonSchema, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(rename_all = "snake_case")]
pub enum ServiceRequirement {
    /// Readiness depends on this service.
    Mandatory,
    /// The platform can remain available without this service.
    Optional,
    /// A newer peer supplied an unrecognized requirement.
    #[serde(other)]
    Unknown,
}

impl ServiceRequirement {
    /// Unknown future requirements are treated conservatively as mandatory.
    #[must_use]
    pub const fn is_mandatory(self) -> bool {
        matches!(self, Self::Mandatory | Self::Unknown)
    }
}

/// Safe, provider-neutral health record for one platform service.
#[derive(Clone, Debug, Eq, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct ServiceHealth {
    /// Stable provider-neutral service identifier.
    pub service_id: String,
    /// Safe user-facing service name.
    pub display_name: String,
    /// Whether the service is required for readiness.
    pub requirement: ServiceRequirement,
    /// Current observed health state.
    pub status: ServiceHealthStatus,
    /// Optional safe explanation containing no raw provider output.
    pub message: Option<String>,
}

impl ServiceHealth {
    /// Creates one explicit service-health record.
    #[must_use]
    pub fn new(
        service_id: impl Into<String>,
        display_name: impl Into<String>,
        requirement: ServiceRequirement,
        status: ServiceHealthStatus,
        message: Option<String>,
    ) -> Self {
        Self {
            service_id: service_id.into(),
            display_name: display_name.into(),
            requirement,
            status,
            message,
        }
    }
}

/// Overall readiness plus the complete evidence used to calculate it.
#[derive(Clone, Debug, Eq, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct ReadinessReport {
    /// Overall readiness calculated from all service evidence.
    pub status: ReadinessStatus,
    /// Safe plain-language explanation of the calculated status.
    pub summary: String,
    /// Complete service evidence used for the calculation.
    pub services: Vec<ServiceHealth>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_wire_enum_values_remain_explicit() {
        let status: ServiceHealthStatus =
            serde_json::from_str("\"future_health_state\"").expect("unknown state is accepted");
        let requirement: ServiceRequirement = serde_json::from_str("\"future_requirement\"")
            .expect("unknown requirement is accepted");

        assert_eq!(status, ServiceHealthStatus::Unknown);
        assert_eq!(requirement, ServiceRequirement::Unknown);
        assert!(requirement.is_mandatory());
    }
}
