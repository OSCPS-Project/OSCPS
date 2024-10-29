//!here we will have 2 connector structs
    //!- Mass Streams
    //!- Energy Streams
pub struct Mconnector{
    pub m_conn_id: String
}

impl Mconnector {
    
    pub fn new(id : String) -> Mconnector {
        return Mconnector {
            m_conn_id : id
        };
    }
}



pub struct Econnector{
    pub e_conn_id: String
}



impl Econnector {
    
    pub fn new(id : String) -> Econnector {
        return Econnector {
            e_conn_id : id
        };
    }
}
