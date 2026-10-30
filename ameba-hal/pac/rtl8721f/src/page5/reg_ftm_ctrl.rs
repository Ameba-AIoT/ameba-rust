#[doc = "Register `REG_FTM_CTRL` reader"]
pub type R = crate::R<RegFtmCtrlSpec>;
#[doc = "Register `REG_FTM_CTRL` writer"]
pub type W = crate::W<RegFtmCtrlSpec>;
#[doc = "Field `BIT_FTM_PTT` reader - partial tsf timer setting"]
pub type BitFtmPttR = crate::FieldReader<u16>;
#[doc = "Field `BIT_FTM_PTT` writer - partial tsf timer setting"]
pub type BitFtmPttW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_FTM_PTT_PORT` reader - 0: sync tsf of port 0 1: sync tsf of port 1"]
pub type BitFtmPttPortR = crate::BitReader;
#[doc = "Field `BIT_FTM_PTT_PORT` writer - 0: sync tsf of port 0 1: sync tsf of port 1"]
pub type BitFtmPttPortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FTM_TSF_T2R_PORT` reader - 0:T2R latch count sync tsf of port 0 1: T2R latch count sync tsf of port 1"]
pub type BitFtmTsfT2rPortR = crate::BitReader;
#[doc = "Field `BIT_FTM_TSF_T2R_PORT` writer - 0:T2R latch count sync tsf of port 0 1: T2R latch count sync tsf of port 1"]
pub type BitFtmTsfT2rPortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FTM_TSF_R2T_PORT` reader - 0:R2T latch count sync tsf of port 0 1: R2T latch count sync tsf of port 1"]
pub type BitFtmTsfR2tPortR = crate::BitReader;
#[doc = "Field `BIT_FTM_TSF_R2T_PORT` writer - 0:R2T latch count sync tsf of port 0 1: R2T latch count sync tsf of port 1"]
pub type BitFtmTsfR2tPortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - partial tsf timer setting"]
    #[inline(always)]
    pub fn bit_ftm_ptt(&self) -> BitFtmPttR {
        BitFtmPttR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - 0: sync tsf of port 0 1: sync tsf of port 1"]
    #[inline(always)]
    pub fn bit_ftm_ptt_port(&self) -> BitFtmPttPortR {
        BitFtmPttPortR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - 0:T2R latch count sync tsf of port 0 1: T2R latch count sync tsf of port 1"]
    #[inline(always)]
    pub fn bit_ftm_tsf_t2r_port(&self) -> BitFtmTsfT2rPortR {
        BitFtmTsfT2rPortR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - 0:R2T latch count sync tsf of port 0 1: R2T latch count sync tsf of port 1"]
    #[inline(always)]
    pub fn bit_ftm_tsf_r2t_port(&self) -> BitFtmTsfR2tPortR {
        BitFtmTsfR2tPortR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - partial tsf timer setting"]
    #[inline(always)]
    pub fn bit_ftm_ptt(&mut self) -> BitFtmPttW<'_, RegFtmCtrlSpec> {
        BitFtmPttW::new(self, 0)
    }
    #[doc = "Bit 16 - 0: sync tsf of port 0 1: sync tsf of port 1"]
    #[inline(always)]
    pub fn bit_ftm_ptt_port(&mut self) -> BitFtmPttPortW<'_, RegFtmCtrlSpec> {
        BitFtmPttPortW::new(self, 16)
    }
    #[doc = "Bit 20 - 0:T2R latch count sync tsf of port 0 1: T2R latch count sync tsf of port 1"]
    #[inline(always)]
    pub fn bit_ftm_tsf_t2r_port(&mut self) -> BitFtmTsfT2rPortW<'_, RegFtmCtrlSpec> {
        BitFtmTsfT2rPortW::new(self, 20)
    }
    #[doc = "Bit 24 - 0:R2T latch count sync tsf of port 0 1: R2T latch count sync tsf of port 1"]
    #[inline(always)]
    pub fn bit_ftm_tsf_r2t_port(&mut self) -> BitFtmTsfR2tPortW<'_, RegFtmCtrlSpec> {
        BitFtmTsfR2tPortW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ftm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ftm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFtmCtrlSpec;
impl crate::RegisterSpec for RegFtmCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ftm_ctrl::R`](R) reader structure"]
impl crate::Readable for RegFtmCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ftm_ctrl::W`](W) writer structure"]
impl crate::Writable for RegFtmCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FTM_CTRL to value 0"]
impl crate::Resettable for RegFtmCtrlSpec {}
