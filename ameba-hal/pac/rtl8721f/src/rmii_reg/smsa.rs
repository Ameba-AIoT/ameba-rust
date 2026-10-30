#[doc = "Register `SMSA` reader"]
pub type R = crate::R<SmsaSpec>;
#[doc = "Register `SMSA` writer"]
pub type W = crate::W<SmsaSpec>;
#[doc = "Field `SMSA` reader - SRAM mapping start address for header mapping to sram"]
pub type SmsaR = crate::FieldReader<u32>;
#[doc = "Field `SMSA` writer - SRAM mapping start address for header mapping to sram"]
pub type SmsaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SRAM mapping start address for header mapping to sram"]
    #[inline(always)]
    pub fn smsa(&self) -> SmsaR {
        SmsaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SRAM mapping start address for header mapping to sram"]
    #[inline(always)]
    pub fn smsa(&mut self) -> SmsaW<'_, SmsaSpec> {
        SmsaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`smsa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smsa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmsaSpec;
impl crate::RegisterSpec for SmsaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smsa::R`](R) reader structure"]
impl crate::Readable for SmsaSpec {}
#[doc = "`write(|w| ..)` method takes [`smsa::W`](W) writer structure"]
impl crate::Writable for SmsaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMSA to value 0"]
impl crate::Resettable for SmsaSpec {}
