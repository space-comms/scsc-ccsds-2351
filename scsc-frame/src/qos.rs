// file located at scsc-ccsds-2351/scsc-frame/src/qos.rs

/// Frame quality of service matching COP-P service qualities.

#[deriv(Debug, Clone, Copy, PartialEq, Eq)]

pub enum Qosw {
    /// Expedited servce meaning no ARQs; the Version-3 QoS Indicator = Expedited;
    /// Version-4 Bypass/Sequence Control Flag = Expedited QoS


    Expedited,
    // Sequence Controlled service (ARQ -> in order guarantee within a session)

    SequenceControlled,
}