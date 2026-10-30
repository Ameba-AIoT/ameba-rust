#[doc = "Register `REG_PPDU_STATUS_CFG` reader"]
pub type R = crate::R<RegPpduStatusCfgSpec>;
#[doc = "Register `REG_PPDU_STATUS_CFG` writer"]
pub type W = crate::W<RegPpduStatusCfgSpec>;
#[doc = "Field `r_stsbuf_threshod` reader - ppdu status buffer almost full threshold"]
pub type RStsbufThreshodR = crate::FieldReader;
#[doc = "Field `r_stsbuf_threshod` writer - ppdu status buffer almost full threshold"]
pub type RStsbufThreshodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_stsbuf_endaddr` reader - ppdu status buffer end address"]
pub type RStsbufEndaddrR = crate::FieldReader;
#[doc = "Field `r_stsbuf_endaddr` writer - ppdu status buffer end address"]
pub type RStsbufEndaddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - ppdu status buffer almost full threshold"]
    #[inline(always)]
    pub fn r_stsbuf_threshod(&self) -> RStsbufThreshodR {
        RStsbufThreshodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - ppdu status buffer end address"]
    #[inline(always)]
    pub fn r_stsbuf_endaddr(&self) -> RStsbufEndaddrR {
        RStsbufEndaddrR::new(((self.bits >> 4) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - ppdu status buffer almost full threshold"]
    #[inline(always)]
    pub fn r_stsbuf_threshod(&mut self) -> RStsbufThreshodW<'_, RegPpduStatusCfgSpec> {
        RStsbufThreshodW::new(self, 0)
    }
    #[doc = "Bits 4:11 - ppdu status buffer end address"]
    #[inline(always)]
    pub fn r_stsbuf_endaddr(&mut self) -> RStsbufEndaddrW<'_, RegPpduStatusCfgSpec> {
        RStsbufEndaddrW::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_ppdu_status_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_ppdu_status_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPpduStatusCfgSpec;
impl crate::RegisterSpec for RegPpduStatusCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_ppdu_status_cfg::R`](R) reader structure"]
impl crate::Readable for RegPpduStatusCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_ppdu_status_cfg::W`](W) writer structure"]
impl crate::Writable for RegPpduStatusCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PPDU_STATUS_CFG to value 0x0ff4"]
impl crate::Resettable for RegPpduStatusCfgSpec {
    const RESET_VALUE: u32 = 0x0ff4;
}
