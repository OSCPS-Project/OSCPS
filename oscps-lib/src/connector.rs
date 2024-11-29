//! # Connector
//!
//! Two types of connectors represent different energy types of streams
//! One is a mass connector, Mconnector, which represents mass flow rates
//! 
//! The other is Econnector, which represents energy flow rates.

#[allow(dead_code)]
/// A connector for storing mass information. This includes an ID and a 
/// total mass flow rate.
///
/// TODO: These should be consolidated into a single connector.
pub struct Mconnector {
    /// Mass connector ID
    pub m_conn_id: String,
    /// Total mass flow rate
    pub m_flow_total: f64,
}

#[allow(dead_code)]
/// Functions implemented on Mconnectors.
impl Mconnector {
    /// Constructor for a connector.
    pub fn new(id: String) -> Mconnector {
        return Mconnector {
            m_conn_id: id,
            m_flow_total: 0.0,
        };
    }
}

#[allow(dead_code)]
/// A connector for storing energy information. This includes an ID and a 
/// total energy flow rate.
///
/// TODO: These should be consolidated into a single connector.
pub struct Econnector {
    /// Energy connector ID.
    pub e_conn_id: String,
    /// Total energy flow rate.
    pub energy_flow_total: f64,
}

#[allow(dead_code)]
/// Functions implemented on Econnectors.
impl Econnector {
    /// Constructor for a connector.
    pub fn new(id: String) -> Econnector {
        return Econnector {
            e_conn_id: id,
            energy_flow_total: 0.0,
        };
    }
}
