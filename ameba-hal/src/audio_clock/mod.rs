// ── Clock source constants ────────────────────────────────────────────────────

pub const PLL_CLOCK_98P304M: u32 = 98_304_000;
pub const PLL_CLOCK_45P1584M: u32 = 45_158_400;
pub const I2S_CLOCK_XTAL40M: u32 = 40_000_000;

/// Indexed by `clock_index` (0, 1, 2).
pub const I2S_CLOCK_SOURCE: [u32; 3] = [PLL_CLOCK_98P304M, PLL_CLOCK_45P1584M, I2S_CLOCK_XTAL40M];

const PLL_CLOCK_MAX_NUM: usize = 2; // only PLL sources, not XTAL

// ── Public types ──────────────────────────────────────────────────────────────

/// Source selection for `Audio_Clock_Choose`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClockSel {
    Pll,
    Xtal,
}

/// Parameters describing the audio path (sample rate, channel geometry, codec).
#[derive(Clone, Copy, Debug)]
pub struct AudioInitParams {
    /// Sample rate in Hz (e.g., 48_000).
    pub sr: u32,
    /// Channel length in bits (e.g., 32).
    pub chn_len: u32,
    /// Channel count (e.g., 2 for stereo).
    pub chn_cnt: u32,
    /// External codec MCLK multiplier (0 if not used).
    pub codec_multiplier_with_rate: u32,
    /// Maximum MCLK frequency in Hz (0 if not used).
    pub sport_mclk_fixed_max: u32,
}

/// Output parameters: chosen clock source and MCLK NI/MI divider pair.
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioClockParams {
    pub clock: u32,
    pub mclk_ni: u32,
    pub mclk_mi: u32,
}

/// Error returned when no valid clock configuration can be found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ClockNotFound;

// ── Pure functions ────────────────────────────────────────────────────────────

/// Check whether a valid NI/MI pair exists for the given clock and audio params.
///
/// Mirrors `is_sport_ni_mi_supported`.
pub fn is_sport_ni_mi_supported(clock: u32, sr: u32, chn_len: u32, chn_cnt: u32) -> bool {
    let denom = chn_cnt * chn_len * sr;
    if denom == 0 {
        return false;
    }
    for ni in 1i32..=32767 {
        let num = clock as i64 * ni as i64;
        if num % denom as i64 != 0 {
            continue;
        }
        let mi = (num / denom as i64) as i32;
        if mi < 2 * ni {
            break;
        }
        if mi <= 65535 {
            return true;
        }
    }
    false
}

/// Choose the appropriate PLL/XTAL clock and MCLK divider for the audio path.
///
/// Returns `Ok(AudioClockParams)` on success, `Err(ClockNotFound)` when no
/// valid configuration exists for the requested parameters.
///
/// Mirrors `Audio_Clock_Choose` (RTK_LOGE/LOGI calls replaced by `Result`).
pub fn audio_clock_choose(
    clock_sel: ClockSel,
    params: &AudioInitParams,
) -> Result<AudioClockParams, ClockNotFound> {
    let mut clock_index = 0usize;
    let mut ni: u32 = 1;
    let mut mi: u32 = 1;

    if clock_sel == ClockSel::Pll {
        let mut choose_done = false;

        if params.codec_multiplier_with_rate != 0 {
            while clock_index < PLL_CLOCK_MAX_NUM {
                let src = I2S_CLOCK_SOURCE[clock_index];
                if !src.is_multiple_of(params.codec_multiplier_with_rate * params.sr) {
                    clock_index += 1;
                    continue;
                }
                mi = src / params.codec_multiplier_with_rate / params.sr;
                if is_sport_ni_mi_supported(src, params.sr, params.chn_len, params.chn_cnt)
                    && mi >= 2
                {
                    choose_done = true;
                }
                break;
            }
        }

        if params.sport_mclk_fixed_max != 0 {
            let clock = if params.sr.is_multiple_of(4000) {
                PLL_CLOCK_98P304M
            } else if params.sr.is_multiple_of(11025) {
                clock_index = 1;
                PLL_CLOCK_45P1584M
            } else {
                return Err(ClockNotFound);
            };

            mi = 1;
            while mi < 65536 {
                if (clock / mi <= params.sport_mclk_fixed_max) && (mi & mi.wrapping_sub(1)) == 0 {
                    if is_sport_ni_mi_supported(clock, params.sr, params.chn_len, params.chn_cnt) {
                        choose_done = true;
                    }
                    break;
                }
                mi += 1;
            }
        }

        if !choose_done {
            return Err(ClockNotFound);
        }
    } else {
        // XTAL clock path
        clock_index = 2;
        match params.codec_multiplier_with_rate {
            512 => {
                if params.sr.is_multiple_of(4000) {
                    if params.sr > 24_000 {
                        return Err(ClockNotFound);
                    }
                    mi = 1250;
                    ni = 2 * 8 * params.sr / 1000;
                } else {
                    if params.sr > 22_050 {
                        return Err(ClockNotFound);
                    }
                    mi = 50_000;
                    ni = 2 * (441 * 8 * (params.sr / 11025));
                }
            }
            256 => {
                if params.sr.is_multiple_of(4000) {
                    if params.sr > 48_000 {
                        return Err(ClockNotFound);
                    }
                    mi = 1250;
                    ni = 8 * params.sr / 1000;
                } else {
                    if params.sr > 44_100 {
                        return Err(ClockNotFound);
                    }
                    mi = 50_000;
                    ni = 441 * 8 * (params.sr / 11025);
                }
            }
            128 => {
                if params.sr.is_multiple_of(4000) {
                    if params.sr > 96_000 {
                        return Err(ClockNotFound);
                    }
                    mi = 1250;
                    ni = 8 * params.sr / 1000 / 2;
                } else {
                    if params.sr > 88_200 {
                        return Err(ClockNotFound);
                    }
                    mi = 50_000;
                    ni = (441 * 8 * (params.sr / 11025)) / 2;
                }
            }
            64 => {
                if params.sr.is_multiple_of(4000) {
                    if params.sr > 192_000 {
                        return Err(ClockNotFound);
                    }
                    mi = 1250;
                    ni = 8 * params.sr / 1000 / 4;
                } else {
                    mi = 50_000;
                    ni = (441 * 8 * (params.sr / 11025)) / 4;
                }
            }
            32 => {
                if params.sr != 384_000 {
                    return Err(ClockNotFound);
                }
                mi = 1250;
                ni = params.sr / 1000;
            }
            _ => return Err(ClockNotFound),
        }
    }

    Ok(AudioClockParams {
        clock: I2S_CLOCK_SOURCE[clock_index],
        mclk_ni: ni,
        mclk_mi: mi,
    })
}
