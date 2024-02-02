#![no_std]
#![no_main]

use defmt::{info, unwrap};
use embassy_executor::Spawner;
use embassy_nrf::{bind_interrupts, peripherals, radio};
use embassy_time::Timer;
use jewel::ll::parse;
use jewel::phy::{BleRadio, MAX_PDU_LENGTH};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    RADIO => radio::InterruptHandler<peripherals::RADIO>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = embassy_nrf::config::Config::default();
    config.hfclk_source = embassy_nrf::config::HfclkSource::ExternalXtal;
    let p = embassy_nrf::init(config);

    info!("Starting radio");

    let mut radio = radio::ble::Radio::new(p.RADIO, Irqs);

    let mut buffer = [0u8; MAX_PDU_LENGTH];
    unwrap!(radio.set_buffer_mut(buffer.as_mut()));

    loop {
        info!("Receiving packet");
        radio.receive().await;
        if let Ok(packet) = parse(&buffer) {
            info!("Received packet: {:?}", &packet);
        } else {
            info!("Received packet: {:?}", &buffer);
        }
        Timer::after_millis(500).await;
    }
}
