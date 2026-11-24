/// Leaf (ERR-2): a field of the input breaks a rule. `field` names the field
/// of the command, so the edge can pick its code; never the value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Validation {
    pub field: &'static str,
}
