//!here we will have 2 connector structs
    //!- Mass Streams
    //!- Energy Streams
pub struct Mconnector{
    pub m_conn_id: String,
    pub m_flow_total: f64
}

impl Mconnector {
    
    pub fn new(id : String) -> Mconnector {
        return Mconnector {
            m_conn_id : id,
            m_flow_total : 0.0
        };
    }
}



pub struct Econnector{
    pub e_conn_id: String,
    pub energy_flow_total: f64
}



impl Econnector {
    
    pub fn new(id : String) -> Econnector {
        return Econnector {
            e_conn_id : id,
            energy_flow_total : 0.0
        };
    }
}
