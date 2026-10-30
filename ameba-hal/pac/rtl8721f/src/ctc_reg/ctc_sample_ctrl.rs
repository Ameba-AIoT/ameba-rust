#[doc = "Register `CTC_SAMPLE_CTRL` reader"]
pub type R = crate::R<CtcSampleCtrlSpec>;
#[doc = "Register `CTC_SAMPLE_CTRL` writer"]
pub type W = crate::W<CtcSampleCtrlSpec>;
#[doc = "Field `settle_cnt` reader - settle count for sample data from ADC to captouch, the unit is sample clock. 0 means no need to settle"]
pub type SettleCntR = crate::FieldReader;
#[doc = "Field `settle_cnt` writer - settle count for sample data from ADC to captouch, the unit is sample clock. 0 means no need to settle"]
pub type SettleCntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `settle_cnt_chn_chg` reader - settle count for sample data from chn switch, the unit is sample clock. 0 means no need to settle"]
pub type SettleCntChnChgR = crate::FieldReader;
#[doc = "Field `settle_cnt_chn_chg` writer - settle count for sample data from chn switch, the unit is sample clock. 0 means no need to settle"]
pub type SettleCntChnChgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - settle count for sample data from ADC to captouch, the unit is sample clock. 0 means no need to settle"]
    #[inline(always)]
    pub fn settle_cnt(&self) -> SettleCntR {
        SettleCntR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - settle count for sample data from chn switch, the unit is sample clock. 0 means no need to settle"]
    #[inline(always)]
    pub fn settle_cnt_chn_chg(&self) -> SettleCntChnChgR {
        SettleCntChnChgR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - settle count for sample data from ADC to captouch, the unit is sample clock. 0 means no need to settle"]
    #[inline(always)]
    pub fn settle_cnt(&mut self) -> SettleCntW<'_, CtcSampleCtrlSpec> {
        SettleCntW::new(self, 0)
    }
    #[doc = "Bits 4:7 - settle count for sample data from chn switch, the unit is sample clock. 0 means no need to settle"]
    #[inline(always)]
    pub fn settle_cnt_chn_chg(&mut self) -> SettleCntChnChgW<'_, CtcSampleCtrlSpec> {
        SettleCntChnChgW::new(self, 4)
    }
}
#[doc = "Captouch Sample Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctc_sample_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctc_sample_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcSampleCtrlSpec;
impl crate::RegisterSpec for CtcSampleCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctc_sample_ctrl::R`](R) reader structure"]
impl crate::Readable for CtcSampleCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctc_sample_ctrl::W`](W) writer structure"]
impl crate::Writable for CtcSampleCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTC_SAMPLE_CTRL to value 0x10"]
impl crate::Resettable for CtcSampleCtrlSpec {
    const RESET_VALUE: u32 = 0x10;
}
