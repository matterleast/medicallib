//! Generalized tissue injury and perfusion framework
//!
//! This module provides a reusable framework for simulating tissue-level
//! pathophysiology across all organ systems. Injuries emerge from actual
//! physiologic mechanisms rather than hardcoded thresholds.

/// Universal tissue state representing cellular health
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TissueState {
    /// Healthy, well-perfused tissue
    Healthy,
    /// Hypoperfused but viable (reversible)
    Hypoperfused {
        duration_seconds: f64,
    },
    /// Ischemic - O2 delivery critically low
    Ischemic {
        duration_seconds: f64,
    },
    /// Injured - cellular damage occurring
    Injured {
        duration_seconds: f64,
        severity: f64,  // 0.0-1.0
    },
    /// Necrotic - cell death
    Necrotic {
        days_old: f64,
        extent: f64,  // Fraction of tissue dead (0.0-1.0)
    },
}

impl TissueState {
    /// Progress tissue state based on perfusion adequacy
    pub fn progress(&mut self, oxygen_delivery: f64, oxygen_demand: f64, delta_time_s: f64) {
        let supply_ratio = if oxygen_demand > 0.0 {
            oxygen_delivery / oxygen_demand
        } else {
            1.0
        };

        match self {
            TissueState::Healthy => {
                if supply_ratio < 0.9 {
                    *self = TissueState::Hypoperfused { duration_seconds: 0.0 };
                }
            }
            TissueState::Hypoperfused { duration_seconds } => {
                if supply_ratio >= 0.9 {
                    // Recovered
                    *self = TissueState::Healthy;
                } else if supply_ratio < 0.6 {
                    // Progressed to ischemia
                    *self = TissueState::Ischemic { duration_seconds: 0.0 };
                } else {
                    *duration_seconds += delta_time_s;
                    // Prolonged hypoperfusion → ischemia
                    if *duration_seconds > 600.0 {  // 10 minutes
                        *self = TissueState::Ischemic { duration_seconds: 0.0 };
                    }
                }
            }
            TissueState::Ischemic { duration_seconds } => {
                if supply_ratio >= 0.8 {
                    // Reperfusion
                    if *duration_seconds < 300.0 {  // < 5 min, full recovery possible
                        *self = TissueState::Healthy;
                    } else {
                        *self = TissueState::Injured {
                            duration_seconds: 0.0,
                            severity: (*duration_seconds / 1800.0).min(0.5)
                        };
                    }
                } else {
                    *duration_seconds += delta_time_s;
                    // Prolonged ischemia → injury
                    if *duration_seconds > 1800.0 {  // 30 minutes
                        let severity = (*duration_seconds / 3600.0).min(1.0);
                        *self = TissueState::Injured {
                            duration_seconds: 0.0,
                            severity
                        };
                    }
                }
            }
            TissueState::Injured { duration_seconds, severity } => {
                if supply_ratio >= 0.8 {
                    // Slow healing with adequate perfusion
                    *severity = (*severity - 0.0001 * delta_time_s).max(0.0);
                    if *severity < 0.1 {
                        *self = TissueState::Healthy;
                    }
                } else {
                    // Continued ischemia worsens injury
                    *duration_seconds += delta_time_s;
                    *severity = (*severity + 0.0002 * delta_time_s).min(1.0);

                    // Severe prolonged injury → necrosis
                    if *duration_seconds > 3600.0 && *severity > 0.8 {
                        *self = TissueState::Necrotic {
                            days_old: 0.0,
                            extent: *severity
                        };
                    }
                }
            }
            TissueState::Necrotic { days_old, .. } => {
                // Dead tissue ages but doesn't recover
                *days_old += delta_time_s / 86400.0;
            }
        }
    }

    /// Get functional capacity (0.0-1.0)
    pub fn functional_capacity(&self) -> f64 {
        match self {
            TissueState::Healthy => 1.0,
            TissueState::Hypoperfused { duration_seconds } => {
                (1.0 - duration_seconds / 600.0 * 0.2).max(0.7)
            }
            TissueState::Ischemic { duration_seconds } => {
                (0.7 - duration_seconds / 1800.0 * 0.5).max(0.2)
            }
            TissueState::Injured { severity, .. } => {
                1.0 - severity
            }
            TissueState::Necrotic { extent, .. } => {
                1.0 - extent
            }
        }
    }

    /// Check if tissue is producing inflammatory cytokines
    pub fn inflammation_level(&self) -> f64 {
        match self {
            TissueState::Healthy | TissueState::Hypoperfused { .. } => 0.0,
            TissueState::Ischemic { duration_seconds } => {
                (duration_seconds / 1800.0).min(0.5)
            }
            TissueState::Injured { severity, .. } => *severity,
            TissueState::Necrotic { extent, days_old } => {
                // Necrotic tissue is highly inflammatory in first few days
                if *days_old < 3.0 {
                    extent * (1.0 - days_old / 3.0)
                } else {
                    0.1 * extent  // Chronic low-grade inflammation
                }
            }
        }
    }

    /// Get oxygen consumption rate (relative to baseline)
    pub fn oxygen_consumption_rate(&self) -> f64 {
        match self {
            TissueState::Healthy => 1.0,
            TissueState::Hypoperfused { .. } => 0.9,
            TissueState::Ischemic { .. } => 0.3,  // Anaerobic metabolism
            TissueState::Injured { severity, .. } => 0.8 - severity * 0.5,
            TissueState::Necrotic { extent, .. } => 1.0 - extent,
        }
    }

    /// Check if tissue is producing lactic acid (anaerobic metabolism)
    pub fn lactate_production_rate(&self) -> f64 {
        match self {
            TissueState::Healthy | TissueState::Hypoperfused { .. } => 0.0,
            TissueState::Ischemic { duration_seconds } => {
                (duration_seconds / 600.0).min(2.0)
            }
            TissueState::Injured { severity, .. } => severity * 0.5,
            TissueState::Necrotic { .. } => 0.0,  // Dead cells don't metabolize
        }
    }
}

/// Tissue perfusion metrics
#[derive(Debug, Clone)]
pub struct TissuePerfusion {
    pub blood_flow_ml_per_min: f64,
    pub baseline_flow_ml_per_min: f64,
    pub oxygen_delivery_ml_per_min: f64,
    pub oxygen_consumption_ml_per_min: f64,
    pub tissue_mass_grams: f64,
    pub state: TissueState,
}

impl TissuePerfusion {
    pub fn new(tissue_mass_grams: f64, flow_per_gram: f64) -> Self {
        let baseline_flow = tissue_mass_grams * flow_per_gram;
        Self {
            blood_flow_ml_per_min: baseline_flow,
            baseline_flow_ml_per_min: baseline_flow,
            oxygen_delivery_ml_per_min: 0.0,
            oxygen_consumption_ml_per_min: tissue_mass_grams * 0.05,  // ~5% baseline
            tissue_mass_grams,
            state: TissueState::Healthy,
        }
    }

    /// Update tissue state based on current perfusion
    pub fn update(&mut self, blood_flow_ml_per_min: f64, arterial_o2_content_ml_per_dl: f64,
                  metabolic_rate: f64, delta_time_s: f64) {
        self.blood_flow_ml_per_min = blood_flow_ml_per_min;

        // O2 delivery = flow × O2 content
        self.oxygen_delivery_ml_per_min = (blood_flow_ml_per_min / 100.0) * arterial_o2_content_ml_per_dl;

        // O2 consumption scales with metabolic rate and tissue state
        let state_consumption_factor = self.state.oxygen_consumption_rate();
        self.oxygen_consumption_ml_per_min = self.tissue_mass_grams * 0.05 * metabolic_rate * state_consumption_factor;

        // Progress tissue state
        self.state.progress(
            self.oxygen_delivery_ml_per_min,
            self.oxygen_consumption_ml_per_min,
            delta_time_s
        );
    }

    /// Get perfusion adequacy (0.0-1.0+)
    pub fn perfusion_ratio(&self) -> f64 {
        if self.baseline_flow_ml_per_min > 0.0 {
            self.blood_flow_ml_per_min / self.baseline_flow_ml_per_min
        } else {
            1.0
        }
    }
}
