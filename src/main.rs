use esp32_nimble::{ uuid128, BLEDevice, NimbleProperties };
use esp_idf_hal::prelude::*;
use std::sync::{ Arc, Mutex };
use std::thread;
use std::time::Duration;

// Smart led (ws2812) drivers
use ws2812_esp32_rmt_driver::driver::color::{ LedPixelColor, LedPixelColorGrb24 };
use ws2812_esp32_rmt_driver::driver::Ws2812Esp32RmtDriver;

fn main() -> anyhow::Result<()> {
    // Initialize logger and system patches (required for Rust + ESP32)
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    // Take the BLE device and get its advertising interface
    let ble_device = BLEDevice::take();
    let advertising = ble_device.get_advertising();

    // Get peripherals (GPIO pins, etc.)
    let peripherals = Peripherals::take().unwrap();
    // Select the GPIO pin for your LED
    let led_pin = peripherals.pins.gpio48;
    // Select RMT channel for controlling the WS2812 LED
    let channel = peripherals.rmt.channel0;
    // Create the WS2812 driver for ESP32 using RMT and GPIO
    let driver = Ws2812Esp32RmtDriver::new(channel, led_pin).unwrap();

    // Create BLE server
    let server = ble_device.get_server();

    // Log client connections and disconnections
    server.on_connect(|_, desc| ::log::info!("Client connected: {:?}", desc));
    server.on_disconnect(|_, reason| ::log::info!("Client disconnected: {:?}", reason));

    // Create a BLE service with a unique UUID
    let ble_service = server.create_service(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa"));

    // Create a writable characteristic for sending motor commands
    let command_char = ble_service
        .lock()
        .create_characteristic(
            uuid128!("3c9a3f00-8ed3-4bdf-8a39-a01bebede295"),
            NimbleProperties::WRITE
        );

    const NUM_LEDS: usize = 10;

    let driver = Arc::new(Mutex::new(driver));

    // Handle incoming BLE write commands
    command_char.lock().on_write(move |args| {
        let data = args.recv_data();
        if let Ok(cmd) = core::str::from_utf8(data) {
            match cmd.trim() {
                "on" => {
                    let on_frame: Vec<LedPixelColorGrb24> = (0..NUM_LEDS)
                        .map(|_| LedPixelColorGrb24::new_with_rgb(255, 255, 255))
                        .collect();

                    driver
                        .lock()
                        .unwrap()
                        .write_blocking(on_frame.iter().flat_map(|c| c.as_ref().iter().copied()))
                        .unwrap();

                    ::log::info!("On");
                }
                "off" => {
                    let off_frame: Vec<LedPixelColorGrb24> = (0..NUM_LEDS)
                        .map(|_| LedPixelColorGrb24::new_with_rgb(0, 0, 0))
                        .collect();

                    driver
                        .lock()
                        .unwrap()
                        .write_blocking(off_frame.iter().flat_map(|c| c.as_ref().iter().copied()))
                        .unwrap();

                    ::log::info!("Off");
                }

                _ => ::log::info!("Unknown command: {}", cmd),
            }
        }
    });

    // Start BLE advertising so other devices can see the ESP32
    advertising
        .lock()
        .set_data(
            esp32_nimble::BLEAdvertisementData
                ::new()
                .name("ble_esp_prazval")
                .add_service_uuid(uuid128!("fafafafa-fafa-fafa-fafa-fafafafafafa"))
        )?;
    advertising.lock().start()?;

    // Show BLE GATT server structure in logs (optional, for debugging)
    server.ble_gatts_show_local();

    // Keep the program alive
    loop {
        thread::sleep(Duration::from_secs(1));
    }
}