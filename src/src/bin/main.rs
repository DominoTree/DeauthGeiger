#![no_std]
#![no_main]

use core::time::Duration;

use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_hal::timer::timg::TimerGroup;
use esp_hal::{delay::Delay, time::Duration};

use esp_wifi::wifi::{ScanConfig, ScanTypeConfig};
use log::info;

extern crate alloc;

#[entry]
fn main() -> ! {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_println::logger::init_logger_from_env();

    esp_alloc::heap_allocator!(72 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let wifi_init = esp_wifi::init(
        timg0.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();

    let wifi_config = ScanConfig {
        ssid: None,
        bssid: None,
        channel: Some(0),
        scan_type: ScanTypeConfig::Passive(Duration::new(0, 10000)),
        show_hidden: true,
    };

    let (wifi_device, mut wifi_controller): (WifiDevice<WifiStaDevice>, _) =
        wifi::new_with_config(&wifi_init, peripherals.WIFI, wifi_config).unwrap();
    wifi_controller.start().unwrap();

    let delay = Delay::new();
    loop {
        info!("Hello world!");
        delay.delay(500.millis());
    }

    // for inspiration have a look at the examples at https://github.com/esp-rs/esp-hal/tree/v0.22.0/examples/src/bin
}
