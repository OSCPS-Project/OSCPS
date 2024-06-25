
use crate::connector;

trait MassBal {
    fn overall_mass_bal_calc() {}
    fn comp_mass_bal_calc(cmp: Component) {}
}

trait EnergyBal{
    fn energy_bal_calc(){}
}

pub struct Mixer{
    pub block_id: String,
    pub input_stream: Vec<connector::Mconnector>,
    pub output_stream: connector::Mconnector
}

impl MassBal for Mixer{

}

impl EnergyBal for Mixer{
    
}

