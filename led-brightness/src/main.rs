use std::sync::{Arc, Mutex};

use anyhow::{bail, Result};
use embedded_svc::http::Method;
use embedded_svc::io::Write;
use embedded_svc::wifi::{AccessPointConfiguration, AuthMethod, ClientConfiguration};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::ledc::{LedcDriver, LedcTimerDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::{modem, prelude::*};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http;
use esp_idf_svc::http::server::EspHttpServer;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use esp_idf_sys as _;
use log::info;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_pwd: &'static str,
}

fn main() -> Result<()> {
    esp_idf_sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    let wifi_conf: Config = CONFIG;

    let _wifi = match wifi(
        wifi_conf.wifi_ssid,
        wifi_conf.wifi_pwd,
        peripherals.modem,
        sysloop,
    ) {
        Ok(inner) => {
            println!("Connected to Wi-Fi network!");
            inner
        }
        Err(err) => {
            bail!("Could not connect to Wi-Fi network: {:?}", err)
        }
    };

    let mut server = EspHttpServer::new(&http::server::Configuration::default())?;

    let timer_driver = LedcTimerDriver::new(
        peripherals.ledc.timer0,
        &TimerConfig::default().frequency(1.kHz().into()),
    )?;

    let driver = Arc::new(Mutex::new(LedcDriver::new(
        peripherals.ledc.channel0,
        timer_driver,
        peripherals.pins.gpio6,
    )?));

    let max_duty = driver.lock().unwrap().get_max_duty();

    let driver_get = driver.clone();
    server.fn_handler("/brightness", Method::Get, move |req| {
        let html = format!(
            r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
            driver_get.lock().unwrap().get_duty()
        );
        let mut res = req.into_ok_response()?;
        res.write_all(html.as_bytes())?;
        Ok(())
    })?;

    // CROS
    server.fn_handler("/brightness", Method::Options, |req| {
        let _ = req.into_response(
            200,
            Some("OK"),
            &[
                ("Access-Control-Allow-Origin", "*"),
                ("Access-Control-Allow-Methods", "GET, PUT"),
                ("Access-Control-Allow-Headers", "Content-Type"),
            ],
        )?;

        Ok(())
    })?;

    let driver_put = driver.clone();
    server.fn_handler("/brightness", Method::Put, move |req| {
        let params: Vec<&str> = req.uri().split('?').collect();

        if params.len() < 2 {
            info!("No params provided");
        }

        if let Ok(value) = params[1].parse::<u32>() {
            if (0..=100).contains(&value) {
                info!("Updating duty to {}%", value);
                driver_put
                    .lock()
                    .unwrap()
                    .set_duty(value * max_duty / 100)?;

                let _ = req.into_response(
                    204,
                    Some("No Content"),
                    &[("Access-Control-Allow-Origin", "*")],
                )?;
            } else {
                info!("Value {} is out of range", value);
            }
        } else {
            info!("Can not parse {}", params[1]);
        }

        Ok(())
    })?;

    loop {
        FreeRtos::delay_ms(20);
    }
}

fn wifi(
    ssid: &str,
    pwd: &str,
    modem: impl Peripheral<P = modem::Modem> + 'static,
    sysloop: EspSystemEventLoop,
) -> Result<Box<EspWifi<'static>>> {
    let mut auth_method = AuthMethod::WPA2Personal;
    if ssid.is_empty() {
        bail!("No WiFi SSID provided");
    }

    if pwd.is_empty() {
        auth_method = AuthMethod::None;
        info!("No WiFi password provided");
    }

    let mut esp_wifi = EspWifi::new(modem, sysloop.clone(), None)?;
    let mut wifi = BlockingWifi::wrap(&mut esp_wifi, sysloop)?;

    wifi.set_configuration(&embedded_svc::wifi::Configuration::Client(
        ClientConfiguration::default(),
    ))?;

    info!("Starting wifi...");
    wifi.start()?;
    info!("Scanning...");

    let ap_infos = wifi.scan()?;
    let ours = ap_infos.into_iter().find(|a| a.ssid == ssid);

    let channel = if let Some(ours) = ours {
        info!("Found configured AP {} on channel {}", ssid, ours.channel);
        Some(ours.channel)
    } else {
        info!(
            "Configured AP {} not found, will go with unknown channel",
            ssid
        );
        None
    };

    wifi.set_configuration(&embedded_svc::wifi::Configuration::Mixed(
        ClientConfiguration {
            ssid: ssid.into(),
            password: pwd.into(),
            channel,
            auth_method,
            ..Default::default()
        },
        AccessPointConfiguration {
            ssid: "aptest".into(),
            channel: channel.unwrap_or(1),
            ..Default::default()
        },
    ))?;

    info!("Connecting wifi...");

    wifi.connect()?;

    info!("Waiting for IP...");

    wifi.wait_netif_up()?;
    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;

    info!("Wifi DHCP info: {:?}", ip_info);

    Ok(Box::new(esp_wifi))
}
