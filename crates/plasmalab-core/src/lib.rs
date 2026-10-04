//! Scientific meaning, independent of rendering, storage, and execution.
//!
//! This foundation contains a measurement requirement and evidence vocabulary.
//! It does not yet define an experiment save format, a solver, or a SELF engine.
//! Values use the explicitly documented conventions of their experiment.

use std::fmt;

/// A named observable, time horizon, and requested absolute error bound.
///
/// This is a requirement, never a claim of achieved accuracy. The observable's
/// unit and the time unit must be defined by the experiment. Different models
/// must explicitly declare whether they can support the requested observable.
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementRequirement {
    observable: String,
    horizon: f64,
    absolute_tolerance: f64,
}

impl MeasurementRequirement {
    pub fn new(
        observable: impl Into<String>,
        horizon: f64,
        absolute_tolerance: f64,
    ) -> Result<Self, RequirementError> {
        let observable = observable.into();
        if observable.trim().is_empty() {
            return Err(RequirementError::MissingObservable);
        }
        if !horizon.is_finite() || horizon <= 0.0 {
            return Err(RequirementError::InvalidHorizon);
        }
        if !absolute_tolerance.is_finite() || absolute_tolerance <= 0.0 {
            return Err(RequirementError::InvalidTolerance);
        }
        Ok(Self {
            observable,
            horizon,
            absolute_tolerance,
        })
    }

    pub fn observable(&self) -> &str {
        &self.observable
    }

    pub fn horizon(&self) -> f64 {
        self.horizon
    }

    pub fn absolute_tolerance(&self) -> f64 {
        self.absolute_tolerance
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequirementError {
    MissingObservable,
    InvalidHorizon,
    InvalidTolerance,
}

impl fmt::Display for RequirementError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::MissingObservable => "an observable must be named",
            Self::InvalidHorizon => "the horizon must be finite and positive",
            Self::InvalidTolerance => "the absolute tolerance must be finite and positive",
        })
    }
}

impl std::error::Error for RequirementError {}

/// Categories are deliberately not ordered: passing one is not passing another.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvidenceKind {
    VisualPlausibility,
    ImplementationAgreement,
    NumericalConvergence,
    PredictiveValidity,
}

/// A projection or interpretation must not masquerade as the simulated state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultKind {
    SimulationObservation,
    DetectedStructure,
    Prediction,
    PresentationProjection,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requirement_retains_the_question_without_claiming_accuracy() {
        let requirement = MeasurementRequirement::new("mean-displacement", 10.0, 1e-3).unwrap();
        assert_eq!(requirement.observable(), "mean-displacement");
        assert_eq!(requirement.horizon(), 10.0);
        assert_eq!(requirement.absolute_tolerance(), 1e-3);
    }

    #[test]
    fn unnamed_observables_are_rejected() {
        for name in ["", " ", "\n\t"] {
            assert_eq!(
                MeasurementRequirement::new(name, 1.0, 0.01),
                Err(RequirementError::MissingObservable)
            );
        }
    }

    #[test]
    fn invalid_numerical_requirements_are_rejected() {
        for value in [0.0, -0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(
                MeasurementRequirement::new("energy", value, 0.01),
                Err(RequirementError::InvalidHorizon)
            );
            assert_eq!(
                MeasurementRequirement::new("energy", 1.0, value),
                Err(RequirementError::InvalidTolerance)
            );
        }
    }
}
