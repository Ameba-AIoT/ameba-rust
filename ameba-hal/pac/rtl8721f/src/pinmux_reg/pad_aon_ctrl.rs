#[doc = "Register `PAD_AON_CTRL` reader"]
pub type R = crate::R<PadAonCtrlSpec>;
#[doc = "Register `PAD_AON_CTRL` writer"]
pub type W = crate::W<PadAonCtrlSpec>;
#[doc = "Field `pad_pupd_slp_mode` reader - pad pupd sleep mode seletion . Note:this bit used in pmc , don't change this addr"]
pub type PadPupdSlpModeR = crate::BitReader;
#[doc = "Field `pad_pupd_slp_mode` writer - pad pupd sleep mode seletion . Note:this bit used in pmc , don't change this addr"]
pub type PadPupdSlpModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vprob_syson_oe` reader - vprob source select, msut one-hot \\[0\\]: AON_OTP power \\[1\\]: SOC power \\[2\\]: BT power \\[3\\]: RSVD"]
pub type VprobSysonOeR = crate::FieldReader;
#[doc = "Field `vprob_syson_oe` writer - vprob source select, msut one-hot \\[0\\]: AON_OTP power \\[1\\]: SOC power \\[2\\]: BT power \\[3\\]: RSVD"]
pub type VprobSysonOeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vprob_en0` reader - pad vprob function enable"]
pub type VprobEn0R = crate::FieldReader;
#[doc = "Field `vprob_en0` writer - pad vprob function enable"]
pub type VprobEn0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - pad pupd sleep mode seletion . Note:this bit used in pmc , don't change this addr"]
    #[inline(always)]
    pub fn pad_pupd_slp_mode(&self) -> PadPupdSlpModeR {
        PadPupdSlpModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - vprob source select, msut one-hot \\[0\\]: AON_OTP power \\[1\\]: SOC power \\[2\\]: BT power \\[3\\]: RSVD"]
    #[inline(always)]
    pub fn vprob_syson_oe(&self) -> VprobSysonOeR {
        VprobSysonOeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - pad vprob function enable"]
    #[inline(always)]
    pub fn vprob_en0(&self) -> VprobEn0R {
        VprobEn0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - pad pupd sleep mode seletion . Note:this bit used in pmc , don't change this addr"]
    #[inline(always)]
    pub fn pad_pupd_slp_mode(&mut self) -> PadPupdSlpModeW<'_, PadAonCtrlSpec> {
        PadPupdSlpModeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - vprob source select, msut one-hot \\[0\\]: AON_OTP power \\[1\\]: SOC power \\[2\\]: BT power \\[3\\]: RSVD"]
    #[inline(always)]
    pub fn vprob_syson_oe(&mut self) -> VprobSysonOeW<'_, PadAonCtrlSpec> {
        VprobSysonOeW::new(self, 4)
    }
    #[doc = "Bits 8:11 - pad vprob function enable"]
    #[inline(always)]
    pub fn vprob_en0(&mut self) -> VprobEn0W<'_, PadAonCtrlSpec> {
        VprobEn0W::new(self, 8)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, PadAonCtrlSpec> {
        DummyW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_aon_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_aon_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PadAonCtrlSpec;
impl crate::RegisterSpec for PadAonCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_aon_ctrl::R`](R) reader structure"]
impl crate::Readable for PadAonCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pad_aon_ctrl::W`](W) writer structure"]
impl crate::Writable for PadAonCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PAD_AON_CTRL to value 0x00ff_0000"]
impl crate::Resettable for PadAonCtrlSpec {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
