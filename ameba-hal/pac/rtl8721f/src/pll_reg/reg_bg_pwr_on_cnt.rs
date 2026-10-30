#[doc = "Register `REG_BG_PWR_ON_CNT` reader"]
pub type R = crate::R<RegBgPwrOnCntSpec>;
#[doc = "Register `REG_BG_PWR_ON_CNT` writer"]
pub type W = crate::W<RegBgPwrOnCntSpec>;
#[doc = "Field `bg_on_cnt` reader - "]
pub type BgOnCntR = crate::FieldReader<u16>;
#[doc = "Field `bg_on_cnt` writer - "]
pub type BgOnCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `bg_i_cnt` reader - "]
pub type BgICntR = crate::FieldReader<u16>;
#[doc = "Field `bg_i_cnt` writer - "]
pub type BgICntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bg_on_cnt(&self) -> BgOnCntR {
        BgOnCntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn bg_i_cnt(&self) -> BgICntR {
        BgICntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn bg_on_cnt(&mut self) -> BgOnCntW<'_, RegBgPwrOnCntSpec> {
        BgOnCntW::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn bg_i_cnt(&mut self) -> BgICntW<'_, RegBgPwrOnCntSpec> {
        BgICntW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bg_pwr_on_cnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bg_pwr_on_cnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBgPwrOnCntSpec;
impl crate::RegisterSpec for RegBgPwrOnCntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bg_pwr_on_cnt::R`](R) reader structure"]
impl crate::Readable for RegBgPwrOnCntSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bg_pwr_on_cnt::W`](W) writer structure"]
impl crate::Writable for RegBgPwrOnCntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BG_PWR_ON_CNT to value 0x0028_0078"]
impl crate::Resettable for RegBgPwrOnCntSpec {
    const RESET_VALUE: u32 = 0x0028_0078;
}
