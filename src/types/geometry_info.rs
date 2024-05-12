use crate::bindings;

pub type GeometryInfo = bindings::GeometryInfo;

impl GeometryInfo {
    pub fn new() -> GeometryInfo {
        return GeometryInfo {
            rho: 0.0,
            sigma: 0.0,
            xi: 0.0,
            psi: 0.0,
            chi: 0.0,
        };
    }

    pub fn set_rho(&mut self, rho: f64) {
        self.rho = rho;
    }
    pub fn set_sigma(&mut self, sigma: f64) {
        self.sigma = sigma;
    }
    pub fn set_xi(&mut self, xi: f64) {
        self.xi = xi;
    }
    pub fn set_psi(&mut self, psi: f64) {
        self.psi = psi;
    }
    pub fn set_chi(&mut self, chi: f64) {
        self.chi = chi;
    }
}

impl Default for GeometryInfo {
    fn default() -> Self {
        Self::new()
    }
}
