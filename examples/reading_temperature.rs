extern crate embedded_hal;
extern crate env_logger;
extern crate linux_embedded_hal as hal;
#[macro_use]
extern crate log;
extern crate bme680_rs;

use bme680_rs::*;
use hal::*;
use embedded_hal::blocking::i2c as i2c;
use std::thread;
use std::result;
use std::time::Duration;

fn main() -> result::Result<(), Bme680Error<<hal::I2cdev as i2c::Read>::Error , <hal::I2cdev as i2c::Write>::Error>>{

    env_logger::init();

    let i2c = I2cdev::new("/dev/i2c-1").unwrap();

    let mut dev = Bme680_dev::init(i2c, Delay{}, 0x76, 25)?;

    let mut sensor_settings: SensorSettings = Default::default();

    sensor_settings.tph_sett.os_hum = Some(BME680_OS_1X);
    sensor_settings.tph_sett.os_pres = Some(BME680_OS_16X);
    sensor_settings.tph_sett.os_temp = Some(BME680_OS_2X);

    sensor_settings.gas_sett.run_gas = Some(0x01);
    sensor_settings.gas_sett.heatr_dur = Some(2000);

    let settings_sel =
        DesiredSensorSettings::OST_SEL |
        DesiredSensorSettings::OSP_SEL |
        DesiredSensorSettings::OSH_SEL |
        DesiredSensorSettings::GAS_SENSOR_SEL;

    debug!("Settings {}", settings_sel.bits());

    debug!("NBCONV_SEL {}", settings_sel == DesiredSensorSettings::NBCONV_SEL);
    debug!("OSH_SEL {}", settings_sel == DesiredSensorSettings::OSH_SEL);

    debug!("NBCONV_SEL {}", settings_sel.intersects(DesiredSensorSettings::NBCONV_SEL));
    debug!("OSH_SEL {}", settings_sel.intersects(DesiredSensorSettings::OSH_SEL));

    debug!("NBCONV_SEL {}", settings_sel.contains(DesiredSensorSettings::NBCONV_SEL));
    debug!("OSH_SEL {}", settings_sel.contains(DesiredSensorSettings::OSH_SEL));

    debug!("NBCONV_SEL {}", settings_sel & DesiredSensorSettings::NBCONV_SEL != DesiredSensorSettings::NBCONV_SEL);
    debug!("OSH_SEL {}", settings_sel & DesiredSensorSettings::OSH_SEL != DesiredSensorSettings::OSH_SEL);

    let profile_dur = dev.get_profile_dur(&sensor_settings)?;
    info!("Duration {}", profile_dur);
    info!("Setting sensor settings");
    dev.set_sensor_settings(settings_sel, &sensor_settings)?;
    info!("Setting forced power modes");
    dev.set_sensor_mode(PowerMode::ForcedMode)?;

    loop {
        thread::sleep(Duration::from_millis(profile_dur as u64));
        info!("Retrieving sensor data");
        let data = dev.get_sensor_data()?;
        info!("Sensor Data {:?}", data);
    }
    Ok(())
}