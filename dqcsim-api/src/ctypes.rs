use super::*;
use dqcsim::log::*;

/// Type for a handle.
///
/// Handles are like pointers into DQCsim's internal structures: all API calls
/// use these to refer to objects. Besides the object, they contain type
/// information. This type can be retrieved using `dqcs_handle_type()`.
///
/// Handles are always positive integers, counting upwards from 1 upon
/// allocation, and they are not reused even after being deleted. Thus, every
/// subsequent object allocation returns a handle one greater than the
/// previous. Note however that DQCsim may allocate objects as well without
/// the user specifically requesting this, so external code should generally
/// *not* rely on this behavior unless otherwise noted. The value zero is
/// reserved for invalid references or error propagation.
///
/// Note that the scope for handles is thread-local. That is, data referenced
/// by a handle cannot be shared or moved between threads.
///
/// The value zero is reserved for invalid references or error propagation.
#[allow(non_camel_case_types)]
pub type dqcs_handle_t = c_ulonglong;

/// Type for a qubit reference.
///
/// Qubit references are exchanged between the frontend, operator, and backend
/// plugins to indicate which qubits a gate operates on. Note that this makes
/// them fundamentally different from handles, which are thread-local.
///
/// Qubit references are always positive integers, counting upwards from 1 upon
/// allocation, and they are not reused even after the qubit is deallocated.
/// Thus, every subsequent allocation returns a qubit reference one greater
/// than the previous. This is guaranteed behavior that external code can rely
/// upon. The value zero is reserved for invalid references or error
/// propagation.
#[allow(non_camel_case_types)]
pub type dqcs_qubit_t = c_ulonglong;

/// Type for a simulation cycle timestamp.
///
/// Timestamps count upward from zero. The type is signed to allow usage of -1
/// for errors, and to allow numerical differences to be represented.
#[allow(non_camel_case_types)]
pub type dqcs_cycle_t = c_longlong;

/// Enumeration of types that can be associated with a handle.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_handle_type_t {
    /// Indicates that the given handle is invalid.
    ///
    /// This indicates one of the following:
    ///
    ///  - The handle value is invalid (zero or negative).
    ///  - The handle has not been used yet.
    ///  - The object associated with the handle was deleted.
    DQCS_HTYPE_INVALID = 0,

    /// Indicates that the given handle belongs to an `ArbData` object.
    ///
    /// This means that the handle supports the `handle` and `arb` interfaces.
    DQCS_HTYPE_ARB_DATA = 1,

    /// Indicates that the given handle belongs to an `ArbCmd` object.
    ///
    /// This means that the handle supports the `handle`, `arb`, and `cmd`
    /// interfaces.
    DQCS_HTYPE_ARB_CMD = 2,

    /// Indicates that the given handle belongs to a `Gate` object.
    ///
    /// This means that the handle supports the `handle`, `arb`, and `gate`
    /// interfaces.
    DQCS_HTYPE_GATE = 3,

    /// Indicates that the given handle belongs to a frontend plugin
    /// configuration object.
    DQCS_HTYPE_FRONT_CONFIG = 4,

    /// Indicates that the given handle belongs to an operator plugin
    /// configuration object.
    DQCS_HTYPE_OPER_CONFIG = 5,

    /// Indicates that the given handle belongs to a backend plugin
    /// configuration object.
    DQCS_HTYPE_BACK_CONFIG = 6,

    /// Indicates that the given handle belongs to a simulator configuration
    /// object.
    DQCS_HTYPE_SIM_CONFIG = 7,
}

/// Enumeration of the three types of plugins.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_plugin_type_t {
    /// Invalid plugin type. Used to indicate failure of an API that returns
    /// a plugin type.
    DQCS_PTYPE_INVALID = 0,

    /// Frontend plugin.
    DQCS_PTYPE_FRONT = 1,

    /// Operator plugin.
    DQCS_PTYPE_OPER = 2,

    /// Backend plugin.
    DQCS_PTYPE_BACK = 3,
}

impl From<PluginType> for dqcs_plugin_type_t {
    fn from(x: PluginType) -> Self {
        match x {
            PluginType::Frontend => dqcs_plugin_type_t::DQCS_PTYPE_FRONT,
            PluginType::Operator => dqcs_plugin_type_t::DQCS_PTYPE_OPER,
            PluginType::Backend => dqcs_plugin_type_t::DQCS_PTYPE_BACK,
        }
    }
}

impl Into<PluginType> for dqcs_plugin_type_t {
    fn into(self) -> PluginType {
        match self {
            dqcs_plugin_type_t::DQCS_PTYPE_FRONT => PluginType::Frontend,
            dqcs_plugin_type_t::DQCS_PTYPE_OPER => PluginType::Operator,
            dqcs_plugin_type_t::DQCS_PTYPE_BACK => PluginType::Backend,
            _ => PluginType::Frontend,
        }
    }
}

/// Enumeration of loglevels and logging modes.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_loglevel_t {
    /// Invalid loglevel. Used to indicate failure of an API that returns a
    /// loglevel.
    DQCS_LOG_INVALID = -1,

    /// Turns logging off.
    DQCS_LOG_OFF = 0,

    /// This loglevel is to be used for reporting a fatal error, resulting from
    /// the owner of the logger getting into an illegal state from which it
    /// cannot recover. Such problems are also reported to the API caller via
    /// Result::Err if applicable.
    DQCS_LOG_FATAL = 1,

    /// This loglevel is to be used for reporting or propagating a non-fatal
    /// error caused by the API caller doing something wrong. Such problems are
    /// also reported to the API caller via Result::Err if applicable.
    DQCS_LOG_ERROR = 2,

    /// This loglevel is to be used for reporting that a called API/function is
    /// telling us we did something wrong (that we weren't expecting), but we
    /// can recover. For instance, for a failed connection attempt to something
    /// that really should not be failing, we can still retry (and eventually
    /// report critical or error if a retry counter overflows). Since we're
    /// still trying to rectify things at this point, such problems are NOT
    /// reported to the API/function caller via Result::Err.
    DQCS_LOG_WARN = 3,

    /// This loglevel is to be used for reporting information specifically
    /// requested by the user/API caller, such as the result of an API function
    /// requested through the command line, or an explicitly captured
    /// stdout/stderr stream.
    DQCS_LOG_NOTE = 4,

    /// This loglevel is to be used for reporting information NOT specifically
    /// requested by the user/API caller, such as a plugin starting up or
    /// shutting down.
    DQCS_LOG_INFO = 5,

    /// This loglevel is to be used for reporting debugging information useful
    /// for debugging the user of the API provided by the logged instance.
    DQCS_LOG_DEBUG = 6,

    /// This loglevel is to be used for reporting debugging information useful
    /// for debugging the internals of the logged instance. Such messages would
    /// normally only be generated by debug builds, to prevent them from
    /// impacting performance under normal circumstances.
    DQCS_LOG_TRACE = 7,

    /// This is intended to be used when configuring the stdout/stderr capture
    /// mode for a plugin process. Selecting it will prevent the stream from
    /// being captured; it will just be the same stream as DQCsim's own
    /// stdout/stderr. When used as the loglevel for a message, the message
    /// itself is sent to stderr instead of passing into DQCsim's log system.
    /// Using this for loglevel filters leads to undefined behavior.
    DQCS_LOG_PASS = 8,
}

impl From<StreamCaptureMode> for dqcs_loglevel_t {
    fn from(x: StreamCaptureMode) -> Self {
        match x {
            StreamCaptureMode::Pass => dqcs_loglevel_t::DQCS_LOG_PASS,
            StreamCaptureMode::Null => dqcs_loglevel_t::DQCS_LOG_OFF,
            StreamCaptureMode::Capture(loglevel) => loglevel.into(),
        }
    }
}

impl Into<StreamCaptureMode> for dqcs_loglevel_t {
    fn into(self) -> StreamCaptureMode {
        match self {
            dqcs_loglevel_t::DQCS_LOG_INVALID => StreamCaptureMode::Null,
            dqcs_loglevel_t::DQCS_LOG_OFF => StreamCaptureMode::Null,
            dqcs_loglevel_t::DQCS_LOG_FATAL => StreamCaptureMode::Capture(Loglevel::Fatal),
            dqcs_loglevel_t::DQCS_LOG_ERROR => StreamCaptureMode::Capture(Loglevel::Error),
            dqcs_loglevel_t::DQCS_LOG_WARN => StreamCaptureMode::Capture(Loglevel::Warn),
            dqcs_loglevel_t::DQCS_LOG_NOTE => StreamCaptureMode::Capture(Loglevel::Note),
            dqcs_loglevel_t::DQCS_LOG_INFO => StreamCaptureMode::Capture(Loglevel::Info),
            dqcs_loglevel_t::DQCS_LOG_DEBUG => StreamCaptureMode::Capture(Loglevel::Debug),
            dqcs_loglevel_t::DQCS_LOG_TRACE => StreamCaptureMode::Capture(Loglevel::Trace),
            dqcs_loglevel_t::DQCS_LOG_PASS => StreamCaptureMode::Pass,
        }
    }
}

impl From<Loglevel> for dqcs_loglevel_t {
    fn from(x: Loglevel) -> Self {
        match x {
            Loglevel::Fatal => dqcs_loglevel_t::DQCS_LOG_FATAL,
            Loglevel::Error => dqcs_loglevel_t::DQCS_LOG_ERROR,
            Loglevel::Warn => dqcs_loglevel_t::DQCS_LOG_WARN,
            Loglevel::Note => dqcs_loglevel_t::DQCS_LOG_NOTE,
            Loglevel::Info => dqcs_loglevel_t::DQCS_LOG_INFO,
            Loglevel::Debug => dqcs_loglevel_t::DQCS_LOG_DEBUG,
            Loglevel::Trace => dqcs_loglevel_t::DQCS_LOG_TRACE,
        }
    }
}

impl dqcs_loglevel_t {
    pub fn into_loglevel(self) -> Result<Loglevel> {
        match self.into() {
            StreamCaptureMode::Capture(level) => Ok(level),
            _ => inv_arg(format!("invalid loglevel {:?}", self)),
        }
    }

    pub fn into_loglevel_filter(self) -> Result<LoglevelFilter> {
        match self.into() {
            StreamCaptureMode::Capture(level) => Ok(level.into()),
            StreamCaptureMode::Null => Ok(LoglevelFilter::Off),
            _ => inv_arg(format!("invalid loglevel filter {:?}", self)),
        }
    }
}

impl From<LoglevelFilter> for dqcs_loglevel_t {
    fn from(x: LoglevelFilter) -> Self {
        match x {
            LoglevelFilter::Off => dqcs_loglevel_t::DQCS_LOG_OFF,
            LoglevelFilter::Fatal => dqcs_loglevel_t::DQCS_LOG_FATAL,
            LoglevelFilter::Error => dqcs_loglevel_t::DQCS_LOG_ERROR,
            LoglevelFilter::Warn => dqcs_loglevel_t::DQCS_LOG_WARN,
            LoglevelFilter::Note => dqcs_loglevel_t::DQCS_LOG_NOTE,
            LoglevelFilter::Info => dqcs_loglevel_t::DQCS_LOG_INFO,
            LoglevelFilter::Debug => dqcs_loglevel_t::DQCS_LOG_DEBUG,
            LoglevelFilter::Trace => dqcs_loglevel_t::DQCS_LOG_TRACE,
        }
    }
}

/// Enumeration of the different qubit sets associated with a gate.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_qubit_set_type_t {
    /// The qubit list containing the target qubits.
    ///
    /// The target qubit list is the list of qubits that are affected by the
    /// gate matrix. Thus, the size of this list dictates the correct size of
    /// the gate matrix.
    DQCS_TARGET = 1,

    /// Set containing additional control qubits.
    ///
    /// These qubits are omitted from the gate matrix; the complete gate matrix
    /// including control qubits is inferred by the backend. Of course, it is
    /// also fine for operators or frontend to provide the complete matrix
    /// including control qubits, by putting the control qubits in the target
    /// list instead.
    DQCS_CONTROL = 2,

    /// Set containing all the qubit measurement registers affected by the
    /// associated gate.
    ///
    /// DQCsim uses this set to determine up to what point it needs to
    /// synchronize with the downstream plugins when a measurement register is
    /// read. Therefore, this set must be correctly specified regardless of
    /// whether the backend infers anything from it. For instance, a
    /// `measure_all` gate *must* include all qubits in this set.
    DQCS_MEASURE = 3,
}

/// Default return type for functions that don't need to return anything.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_return_t {
    /// The function has failed. More information may be obtained through
    /// `dqcsim_explain()`.
    DQCS_FAILURE = -1,

    /// The function did what it was supposed to.
    DQCS_SUCCESS = 0,
}

/// Return type for functions that normally return a boolean but can also fail.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[allow(non_camel_case_types)]
pub enum dqcs_bool_return_t {
    /// The function has failed. More information may be obtained through
    /// `dqcsim_explain()`.
    DQCS_BOOL_FAILURE = -1,

    /// The function did what it was supposed to and returned false.
    DQCS_FALSE = 0,

    /// The function did what it was supposed to and returned true.
    DQCS_TRUE = 1,
}

impl From<bool> for dqcs_bool_return_t {
    fn from(b: bool) -> Self {
        if b {
            dqcs_bool_return_t::DQCS_TRUE
        } else {
            dqcs_bool_return_t::DQCS_FALSE
        }
    }
}
