# hcsr04

A platform-agnostic, `no_std` driver for the HC-SR04 ultrasonic distance sensor using [`embedded-hal`] and [`embedded-hal-async`] traits.

This driver allows you to:

- Measure distance using the HC-SR04 sensor.
- Compensate for temperature effects on distance measurement.

[`embedded-hal`]: https://docs.rs/embedded-hal/latest/embedded_hal/
[`embedded-hal-async`]: https://docs.rs/embedded-hal-async/latest/embedded_hal_async/

## Installation

Add `hcsr04` to your project:

```bash
cargo add hcsr04 --features defmt
```

OR add the following to your `Cargo.toml`:

```toml
[dependencies]
hcsr04 = { version = "0.1", features = ["defmt"] }
```

## Optional Cargo features

- **`defmt`**: Use `defmt` logging to print messages.

## Examples

Usage examples can be found in the [examples] folder and run on the `esp32-c3` board.

- [ultrasonic_led]: measure distance and control LED based on the distance.
- [temperature_compensation]: measure distance and compensate for temperature.

[examples]: ./examples
[ultrasonic_led]: ./examples/src/bin/ultrasonic_led.rs
[temperature_compensation]: ./examples/src/bin/temperature_compensation.rs

### Measure distance

Full code see [ultrasonic_led.rs](./examples/src/bin/ultrasonic_led.rs)

```rust
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
```

## Wokwi

Wokwi provides a simulation solution for embedded and IoT system engineers.

- Install the [Wokwi for VS Code] extension.
- Press `F1` OR `⌘ + ⇧ + P` to open the command palette.
- Type in `Wokwi: Start Simulator and Wait for Debugger` and press enter.
- In the Wokwi simulator, click on the `Debug` button to start debugging.

[Wokwi]: https://wokwi.com/
[Wokwi for VS Code]: https://marketplace.visualstudio.com/items?itemName=wokwi.wokwi-vscode

## License

Licensed under either of:

- GPL-3.0 license ([LICENSE-GPLv3] or <https://www.gnu.org/licenses/gpl-3.0.html>)

[LICENSE-GPLv3]: ./LICENSE
