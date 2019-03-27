//! This module defines some types that are shared between other modules, and
//! therefore don't really have a home.

// Sequence numbers used within the gatestreams.
mod sequence_number;
pub use sequence_number::{SequenceNumber, SequenceNumberGenerator};

// Simulation cycle types.
mod cycle;
pub use cycle::{Cycle, CycleDelta, Cycles};

// Qubit references.
mod qubit_ref;
pub use qubit_ref::{QubitRef, QubitRefGenerator};

// User-defined/implementation-specific data.
mod arb_data;
pub use arb_data::ArbData;

// User-defined/implementation-specific commands.
mod arb_cmd;
pub use arb_cmd::ArbCmd;

// Generic representation of a quantum or mixed quantum-classical gate.
mod gate;
pub use gate::Gate;

// Generic representation of a qubit measurement result.
mod measurement;
pub use measurement::{QubitMeasurementResult, QubitMeasurementState};

// Metadata used to identify plugins.
mod plugin_metadata;
pub use plugin_metadata::PluginMetadata;
