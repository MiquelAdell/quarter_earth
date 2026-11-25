//! Two different interfaces to Hector are provided here: one for native builds,
//! where C++ FFI is ok, and one for web/WASM builds where it isn't. For WASM builds
//! Hector is compiled to it's own WASM binary and called through Javascript.
//! That JS glue code is from the old web version of the game, I kept it mostly intact.
//!
//! The actual `tgav` calculation method is a bit weird because it has to be async for web.
//! So on native you really just call it once and get the temperature anomaly immediately.
//! On web you have to poll it, calling `tgav` each frame until `Some` is returned.

#[cfg(not(target_arch = "wasm32"))]
mod tgav {
    use std::collections::HashMap;

    use hector::{emissions::get_emissions, run_hector};

    pub type EmissionsData = HashMap<String, HashMap<String, Vec<f64>>>;

    const DEFAULT_SCENARIO: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/hector/rcp26.default_emissions.json"
    ));

    pub struct Climate {
        emissions: EmissionsData,
        default_emissions: HashMap<&'static str, f64>,
    }
    impl Climate {
        pub fn new(start_year: usize) -> Self {
            Self {
                emissions: get_emissions(start_year),
                default_emissions: serde_json::from_str(DEFAULT_SCENARIO)
                    .expect("default emissions are valid"),
            }
        }

        pub fn set_emissions_data(&mut self, data: EmissionsData) {
            self.emissions = data;
        }

        pub fn emissions_data(&self) -> EmissionsData {
            self.emissions.clone()
        }

        pub fn add_emissions(&mut self, emissions: HashMap<&'static str, f64>) {
            for (k, val) in self.default_emissions.iter() {
                let val = emissions.get(k).unwrap_or(val);

                // Some sources (NOX, CO, NMVOC) feed more than one Hector
                // section (`OH` and `ozone`). Append to every section that
                // lists the key, as the browser path does. Writing only the
                // first match made the result depend on HashMap order.
                let found = self
                    .emissions
                    .values_mut()
                    .filter_map(|section| section.get_mut(*k))
                    .fold(false, |_, vals| {
                        vals.push(*val);
                        true
                    });

                if !found {
                    panic!("{k} not found");
                }
            }
        }

        pub fn is_ready(&self) -> bool {
            true
        }

        pub fn tgav(&self, year: usize) -> Option<f32> {
            let tgav = unsafe { run_hector(year, &self.emissions) };
            Some(tgav as f32)
        }
    }
}

#[cfg(target_arch = "wasm32")]
mod tgav {
    use poll_promise::Promise;
    use serde::ser::Serialize;
    use serde_wasm_bindgen::Serializer;
    use std::{cell::RefCell, collections::HashMap, rc::Rc};
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::JsFuture;

    pub type EmissionsData = HashMap<String, Vec<f64>>;

    #[wasm_bindgen(module = "/assets/js/dist/tgav.js")]
    extern "C" {
        type Temperature;

        #[wasm_bindgen(constructor)]
        fn new(start_year: usize) -> Temperature;

        #[wasm_bindgen(method, js_name = addEmissions)]
        fn add_emissions(this: &Temperature, emissions: JsValue);

        #[wasm_bindgen(method, js_name = setEmissions)]
        fn set_emissions(this: &Temperature, emissions: JsValue);

        #[wasm_bindgen(method, js_name = getEmissions)]
        fn get_emissions(this: &Temperature) -> JsValue;

        #[wasm_bindgen(method, js_name = isReady)]
        fn is_ready(this: &Temperature) -> bool;

        #[wasm_bindgen(method, js_name = updateTemperature)]
        fn calc_tgav(this: &Temperature) -> wasm_bindgen_futures::js_sys::Promise;
    }

    pub struct Climate {
        inner: Rc<RefCell<Temperature>>,
        task: Option<(usize, Promise<f32>)>,
    }
    impl Climate {
        pub fn new(start_year: usize) -> Self {
            let hector = Temperature::new(start_year);
            Climate {
                inner: Rc::new(RefCell::new(hector)),
                task: None,
            }
        }

        pub fn set_emissions_data(&mut self, data: EmissionsData) {
            let serializer = Serializer::new().serialize_maps_as_objects(true);
            let emissions = data.serialize(&serializer).unwrap();
            self.inner.borrow().set_emissions(emissions);
        }

        pub fn emissions_data(&self) -> EmissionsData {
            let data = self.inner.borrow().get_emissions();
            serde_wasm_bindgen::from_value(data).unwrap()
        }

        pub fn add_emissions(&self, emissions: HashMap<&'static str, f64>) {
            let serializer = Serializer::new().serialize_maps_as_objects(true);
            let emissions = emissions.serialize(&serializer).unwrap();
            self.inner.borrow().add_emissions(emissions);
        }

        pub fn is_ready(&self) -> bool {
            self.inner.borrow().is_ready()
        }

        pub fn tgav(&mut self, year: usize) -> Option<f32> {
            match &self.task {
                Some((y, prom)) if *y == year => prom.ready().cloned(),
                _ => {
                    let inner = self.inner.clone();
                    self.task = Some((
                        year,
                        Promise::spawn_local(async move {
                            let promise = inner.borrow().calc_tgav();
                            let future = JsFuture::from(promise);
                            let result = future.await.unwrap();
                            result.as_f64().unwrap() as f32
                        }),
                    ));
                    None
                }
            }
        }
    }
}

pub use tgav::*;

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use std::collections::HashMap;

    use super::Climate;

    const START_YEAR: usize = 2022;
    /// Values from 1765 up to (excluding) the start year.
    const HISTORY_LEN: usize = START_YEAR - 1765;
    /// `NOX_emissions` in assets/hector/rcp26.default_emissions.json.
    const DEFAULT_NOX: f64 = 29.013509999999997;
    const SHARED_SOURCES: [&str; 3] = ["NOX_emissions", "CO_emissions", "NMVOC_emissions"];

    fn series(climate: &Climate, section: &str, source: &str) -> Vec<f64> {
        climate.emissions_data()[section][source].clone()
    }

    #[test]
    fn test_add_emissions_appends_shared_sources_to_every_section() {
        let mut climate = Climate::new(START_YEAR);
        let year = HashMap::from([
            ("ffi_emissions", 10.0),
            ("CH4_emissions", 300.0),
            ("N2O_emissions", 8.0),
        ]);
        climate.add_emissions(year.clone());
        climate.add_emissions(year);

        for source in SHARED_SOURCES {
            let oh = series(&climate, "OH", source);
            let ozone = series(&climate, "ozone", source);
            assert_eq!(oh.len(), HISTORY_LEN + 2, "{source} in OH");
            assert_eq!(ozone.len(), HISTORY_LEN + 2, "{source} in ozone");
            assert_eq!(oh, ozone, "{source} differs between OH and ozone");
        }
        let nox = series(&climate, "OH", "NOX_emissions");
        assert_eq!(nox[HISTORY_LEN..].to_vec(), vec![DEFAULT_NOX, DEFAULT_NOX]);
        assert_eq!(
            series(&climate, "simpleNbox", "ffi_emissions")[HISTORY_LEN..].to_vec(),
            vec![10.0, 10.0]
        );
    }

    #[test]
    fn test_add_emissions_keeps_every_series_the_same_length() {
        let mut climate = Climate::new(START_YEAR);
        climate.add_emissions(HashMap::new());

        let lengths = climate
            .emissions_data()
            .values()
            .flat_map(|section| section.values().map(Vec::len))
            .collect::<std::collections::BTreeSet<_>>();
        assert_eq!(lengths, std::collections::BTreeSet::from([HISTORY_LEN + 1]));
    }
}
