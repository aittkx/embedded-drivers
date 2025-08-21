#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::{error, info};
use embassy_executor::Spawner;
use embassy_time::{Delay, Timer};
use esp_hal::{
    clock::CpuClock,
    gpio::{AnyPin, Input, InputConfig, Level, Output, OutputConfig, Pull},
    timer::systimer::SystemTimer,
};
use hcsr04::{Hcsr04, NoTemperatureCompensation};
use panic_rtt_target as _;

extern crate alloc;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    rtt_target::rtt_init_defmt!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 64 * 1024);

    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let trig = peripherals.GPIO19;
    let echo = peripherals.GPIO18;
    let led = peripherals.GPIO0;

    info!("Peripherals initialized!");

    spawner.must_spawn(ultrasonic_led(trig.into(), echo.into(), led.into()));
}

#[embassy_executor::task]
/// Task to measure distance and control LED based on the distance.
///
/// # Arguments
///
/// * `trig` - The trigger pin of the HC-SR04 sensor.
/// * `echo` - The echo pin of the HC-SR04 sensor.
/// * `led` - The LED pin to control based on the distance.
async fn ultrasonic_led(trig: AnyPin<'static>, echo: AnyPin<'static>, led: AnyPin<'static>) {
    info!("Starting ultrasonic led task");
    // Init GPIO
    let mut led = Output::new(led, Level::Low, OutputConfig::default());
    let trig = Output::new(trig, Level::Low, OutputConfig::default());
    let echo = Input::new(echo, InputConfig::default().with_pull(Pull::Down));
    // Create a HC-SR04 sensor instance
    let mut hcsr04 = Hcsr04::builder()
        .trig(trig)
        .echo(echo)
        .delay(Delay)
        .temperature(NoTemperatureCompensation)
        .build();
    loop {
        // Measure distance
        let distance = match hcsr04.measure_distance().await {
            Ok(distance) => distance,
            Err(e) => {
                error!("Fail to measure {}", e);
                continue;
            }
        };
        info!("measure distance value is {} cm.", distance);
        if distance < 30.0 {
            led.set_high();
        } else {
            led.set_low();
        }
        Timer::after_millis(500).await;
    }
}
