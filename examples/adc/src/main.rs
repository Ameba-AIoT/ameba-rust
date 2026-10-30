#![no_std]
#![no_main]

use ameba_rt as _;
use cortex_m as _;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};

use ameba_hal::adc::{Adc, AdcInitParams, AdcOpMode};
use ameba_hal::println;
use ameba_hal::{
    pinmux::{self, Pin, PinFunction, Pull},
    rcc,
};

// ADC channel 3 is routed to pad PA_17 (AD_3) with pinmux function ADC.
// (PA20/CH0 is unavailable on this board per the schematic.)
// Jumper PA17 to 3.3V (or GND) to feed the sampled voltage.
const ADC_CH: u8 = 3; // SDK: ameba_adc.h : ADC_CH3=0x03; ADC_CH3_PIN=_PA_17

// Acceptable window for a 3.3V tie point: 2.8V .. 3.6V (in mV).
const VMIN_MV: i32 = 2800;
const VMAX_MV: i32 = 3600;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    println!("[ADC] start");
    ameba_hal::init(ameba_hal::Config::default());

    // Enable the LS-ADC function and clock.
    rcc::enable(rcc::Peripheral::ADC);

    // Configure the analog pad: ADC function, no pull, digital input path off.
    pinmux::set(Pin::PA17, PinFunction::ADC);
    pinmux::set_pull(Pin::PA17, Pull::NoPull);
    pinmux::set_input(Pin::PA17, false);

    // Single-channel, software-triggered sampling of channel 3 (PA17).
    let adc = unsafe { Adc::steal() };
    let mut cvlist = [0u8; 16];
    cvlist[0] = ADC_CH;
    let params = AdcInitParams {
        op_mode: AdcOpMode::SoftwareTrigger,
        rx_threshold_level: 0,
        cvlist,
        cvlist_len: 1,
        special_ch: 0xFF,
        sample_period_us: 6,
    };
    adc.init(&params);
    adc.set_ch_list(&[ADC_CH]);
    adc.cmd(true);

    // Give the analog front-end a moment to settle after power-on.
    Timer::after(Duration::from_millis(50)).await;

    let mut all_ok = true;
    for _ in 0..5 {
        adc.clear_fifo();
        adc.sw_trig_cmd(true);
        while !adc.readable() {}
        adc.sw_trig_cmd(false);

        let raw = adc.read().data;
        // EFuse-calibrated raw -> millivolts (per-chip curve, read once by the driver).
        let mv = adc.get_voltage(raw as u32);
        let whole = mv / 1000;
        let frac = ((mv % 1000) / 10).abs(); // two decimal places

        println!("[ADC] raw: {} voltage: {}.{:02}V", raw as u32, whole, frac,);

        if !(VMIN_MV..=VMAX_MV).contains(&mv) {
            all_ok = false;
            println!("[FAIL] ADC: voltage out of range: {}.{:02}V", whole, frac,);
        }

        Timer::after(Duration::from_millis(200)).await;
    }

    if all_ok {
        println!("[PASS] ADC: voltage OK");
    }

    // ── Timer-trigger mode: exercise the de-ROM'd RTIM path (`timer_trig_cmd`). ──
    // The SDK `ADC_TEST_TIMTrigger` enables the basic-timer clock (APBPeriph_TIMx)
    // before arming the trigger; `timer_trig_cmd` (like ROM `RTIM_TimeBaseInit`)
    // does not touch RCC, so enable it here.
    rcc::enable(rcc::Peripheral::LTIM0);

    adc.cmd(false);
    let tt_params = AdcInitParams {
        op_mode: AdcOpMode::TimerTrigger,
        ..params
    };
    adc.init(&tt_params);
    adc.set_ch_list(&[ADC_CH]);
    adc.cmd(true);
    adc.clear_fifo();

    // Basic timer 0 fires a conversion every 100 ms.
    adc.timer_trig_cmd(0, 100, true);

    let mut tt_ok = false;
    // Wait up to ~1 s for the timer to drive a conversion into the FIFO.
    for _ in 0..20 {
        if adc.readable() {
            let raw = adc.read().data;
            let mv = adc.get_voltage(raw as u32);
            let whole = mv / 1000;
            let frac = ((mv % 1000) / 10).abs();
            println!(
                "[ADC] timer-trig raw: {} voltage: {}.{:02}V",
                raw as u32, whole, frac,
            );
            tt_ok = (VMIN_MV..=VMAX_MV).contains(&mv);
            break;
        }
        Timer::after(Duration::from_millis(50)).await;
    }

    adc.timer_trig_cmd(0, 100, false);

    if tt_ok {
        println!("[PASS] ADC: timer-trigger OK");
    } else {
        println!("[FAIL] ADC: timer-trigger no valid sample");
    }

    loop {
        Timer::after(Duration::from_millis(1000)).await;
    }
}
