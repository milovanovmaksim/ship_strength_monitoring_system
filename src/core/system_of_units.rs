use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(from = "f64")]
pub struct Newton(pub f64);

#[derive(Deserialize, Debug)]
pub struct Tons(pub f64);


impl From<f64> for Newton {
    fn from(value: f64) -> Self {
        let newton = value * 1000.0 * 9.81;
        Newton(newton)
    }
}


impl From<Tons> for Newton {
    fn from(tonn: Tons) -> Newton {
        let newton = tonn.0 * 1000.0 * 9.81;
        Newton(newton)
    }
}

impl From<Newton> for Tons {
    fn from(value: Newton) -> Self {
        Tons((value.0 / 9.81) / 1000.0)
    }
}
