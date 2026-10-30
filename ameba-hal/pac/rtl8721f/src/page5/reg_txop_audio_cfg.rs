#[doc = "Register `REG_TXOP_AUDIO_CFG` reader"]
pub type R = crate::R<RegTxopAudioCfgSpec>;
#[doc = "Register `REG_TXOP_AUDIO_CFG` writer"]
pub type W = crate::W<RegTxopAudioCfgSpec>;
#[doc = "Field `BIT_TXOP_BRK_MARGIN` reader - extra transtime during TXOP, unit: 2us if data_transtime + BIT_TXOP_BRK_MARGIN*2 exceed the remaining TXOP count, will abort this TX"]
pub type BitTxopBrkMarginR = crate::FieldReader;
#[doc = "Field `BIT_TXOP_BRK_MARGIN` writer - extra transtime during TXOP, unit: 2us if data_transtime + BIT_TXOP_BRK_MARGIN*2 exceed the remaining TXOP count, will abort this TX"]
pub type BitTxopBrkMarginW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_EN_TXOP_BREAK` reader - 1: enable TXOP break gen_cmd and TXOP abort TX"]
pub type BitEnTxopBreakR = crate::BitReader;
#[doc = "Field `BIT_EN_TXOP_BREAK` writer - 1: enable TXOP break gen_cmd and TXOP abort TX"]
pub type BitEnTxopBreakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_audio_str_play_sel` reader - select ps_timer interrput for audio start play function 1: select ps_timer_a int 2: select ps_timer_b int 3: select ps_timer_c int 4: select ps_timer_d int 5: select ps_timer_e int else: TIE0"]
pub type RAudioStrPlaySelR = crate::FieldReader;
#[doc = "Field `r_audio_str_play_sel` writer - select ps_timer interrput for audio start play function 1: select ps_timer_a int 2: select ps_timer_b int 3: select ps_timer_c int 4: select ps_timer_d int 5: select ps_timer_e int else: TIE0"]
pub type RAudioStrPlaySelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_audio_trig_unit` reader - 0: trigger audio latch period is 1ms 1: period is 2ms 2: period is 4ms 3: period is 8ms ...... 6: period is 64ms 7: period is 128ms"]
pub type RAudioTrigUnitR = crate::FieldReader;
#[doc = "Field `r_audio_trig_unit` writer - 0: trigger audio latch period is 1ms 1: period is 2ms 2: period is 4ms 3: period is 8ms ...... 6: period is 64ms 7: period is 128ms"]
pub type RAudioTrigUnitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `r_audio_sel_p0` reader - 1: select P0's TSFT to trigger audio latch count"]
pub type RAudioSelP0R = crate::BitReader;
#[doc = "Field `r_audio_sel_p0` writer - 1: select P0's TSFT to trigger audio latch count"]
pub type RAudioSelP0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - extra transtime during TXOP, unit: 2us if data_transtime + BIT_TXOP_BRK_MARGIN*2 exceed the remaining TXOP count, will abort this TX"]
    #[inline(always)]
    pub fn bit_txop_brk_margin(&self) -> BitTxopBrkMarginR {
        BitTxopBrkMarginR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: enable TXOP break gen_cmd and TXOP abort TX"]
    #[inline(always)]
    pub fn bit_en_txop_break(&self) -> BitEnTxopBreakR {
        BitEnTxopBreakR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 25:27 - select ps_timer interrput for audio start play function 1: select ps_timer_a int 2: select ps_timer_b int 3: select ps_timer_c int 4: select ps_timer_d int 5: select ps_timer_e int else: TIE0"]
    #[inline(always)]
    pub fn r_audio_str_play_sel(&self) -> RAudioStrPlaySelR {
        RAudioStrPlaySelR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bits 28:30 - 0: trigger audio latch period is 1ms 1: period is 2ms 2: period is 4ms 3: period is 8ms ...... 6: period is 64ms 7: period is 128ms"]
    #[inline(always)]
    pub fn r_audio_trig_unit(&self) -> RAudioTrigUnitR {
        RAudioTrigUnitR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 1: select P0's TSFT to trigger audio latch count"]
    #[inline(always)]
    pub fn r_audio_sel_p0(&self) -> RAudioSelP0R {
        RAudioSelP0R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - extra transtime during TXOP, unit: 2us if data_transtime + BIT_TXOP_BRK_MARGIN*2 exceed the remaining TXOP count, will abort this TX"]
    #[inline(always)]
    pub fn bit_txop_brk_margin(&mut self) -> BitTxopBrkMarginW<'_, RegTxopAudioCfgSpec> {
        BitTxopBrkMarginW::new(self, 0)
    }
    #[doc = "Bit 8 - 1: enable TXOP break gen_cmd and TXOP abort TX"]
    #[inline(always)]
    pub fn bit_en_txop_break(&mut self) -> BitEnTxopBreakW<'_, RegTxopAudioCfgSpec> {
        BitEnTxopBreakW::new(self, 8)
    }
    #[doc = "Bits 25:27 - select ps_timer interrput for audio start play function 1: select ps_timer_a int 2: select ps_timer_b int 3: select ps_timer_c int 4: select ps_timer_d int 5: select ps_timer_e int else: TIE0"]
    #[inline(always)]
    pub fn r_audio_str_play_sel(&mut self) -> RAudioStrPlaySelW<'_, RegTxopAudioCfgSpec> {
        RAudioStrPlaySelW::new(self, 25)
    }
    #[doc = "Bits 28:30 - 0: trigger audio latch period is 1ms 1: period is 2ms 2: period is 4ms 3: period is 8ms ...... 6: period is 64ms 7: period is 128ms"]
    #[inline(always)]
    pub fn r_audio_trig_unit(&mut self) -> RAudioTrigUnitW<'_, RegTxopAudioCfgSpec> {
        RAudioTrigUnitW::new(self, 28)
    }
    #[doc = "Bit 31 - 1: select P0's TSFT to trigger audio latch count"]
    #[inline(always)]
    pub fn r_audio_sel_p0(&mut self) -> RAudioSelP0W<'_, RegTxopAudioCfgSpec> {
        RAudioSelP0W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txop_audio_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_txop_audio_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxopAudioCfgSpec;
impl crate::RegisterSpec for RegTxopAudioCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txop_audio_cfg::R`](R) reader structure"]
impl crate::Readable for RegTxopAudioCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_txop_audio_cfg::W`](W) writer structure"]
impl crate::Writable for RegTxopAudioCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TXOP_AUDIO_CFG to value 0xa000_0000"]
impl crate::Resettable for RegTxopAudioCfgSpec {
    const RESET_VALUE: u32 = 0xa000_0000;
}
