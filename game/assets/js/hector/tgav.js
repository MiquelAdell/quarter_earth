import initHector from './init';

// Base emissions scenario for Hector
const baseEmissionsScenario = '/hector/rcp26.to_2050.json';
const defaultEmissionsScenario = '/hector/rcp26.default_emissions.json';
const START_YEAR = 1765; // Should match `baseEmissionsScenario["startYear"]`

const hectorOutputVars = {
  'temperature.Tgav': {
    'component': 'temperature',
    'description': 'global atmospheric temperature anomaly',
    'unit': 'degC',
    'variable': 'Tgav'
  }
};

class Temperature {
  constructor(startYear) {
    this.startYear = startYear;
    this.ready = false;
    this.pendingEmissions = undefined;

    this.emissions = {
      startYear: START_YEAR,
      data: {},
    };

    // Load all climate inputs together. Workshop mode advances the world
    // immediately, so it can reach addEmissions before any individual fetch
    // resolves. `ready` lets the Rust transition wait for a complete model.
    Promise.all([
      fetch('/hector/config.json').then((resp) => resp.json()),
      fetch(defaultEmissionsScenario).then((resp) => resp.json()),
      fetch(baseEmissionsScenario).then((resp) => resp.json()),
    ]).then(([config, defaultEmissions, baseScenario]) => {
        this.config = config;
        this.defaultEmissions = defaultEmissions;
        this.emissions = {
          startYear: baseScenario['startYear'],
          data: {}
        };

        // Only get the base scenario data up to the game starting year
        let baseYears = this.startYear - baseScenario['startYear'];
        Object.keys(baseScenario['data']).forEach((k) => {
          this.emissions['data'][k] = baseScenario['data'][k].slice(0, baseYears);
        });

        if (this.pendingEmissions !== undefined) {
          this.emissions.data = this.pendingEmissions;
        }
        this.ready = true;
      });
  }

  /*
   * Adds a year of emissions data.
   * `emissions` should have keys and values for each required
   * emissions type. Refer to `/assets/hector/rcp.to_2050.json`
   * for the required keys.
   */
  addEmissions(emissions) {
    Object.keys(this.defaultEmissions).forEach((k) => {
      let val = emissions[k] !== undefined ? emissions[k] : this.defaultEmissions[k];
      this.emissions['data'][k].push(val);
    });
  }

  setEmissions(emissions) {
    if (this.ready) {
      this.emissions.data = emissions;
    } else {
      this.pendingEmissions = emissions;
    }
  }

  isReady() {
    return this.ready;
  }

  getEmissions() {
    return this.emissions.data;
  }

  updateTemperature() {
    let ready = this._hector ?
      Promise.resolve(this._hector) :
      initHector(this.config, hectorOutputVars).then((hector) => {
        this._hector = hector;
      });

    return ready.then(() => {
      // Calculate new avg global temp
      // Only compute up to the current year,
      // so the last returned tgav is the current tgav
      let endDate = this.emissions.startYear + this.emissions.data['ffi_emissions'].length;
      let results = this._hector.run(endDate, this.emissions);
      let avgGlobalTemps = results['temperature.Tgav'];
      let avgGlobalTemp = avgGlobalTemps[avgGlobalTemps.length - 1];
      return avgGlobalTemp;
    });
  }
}

export { Temperature };
