#[doc = "Register `IC_HS_MAR` reader"]
pub type R = crate::R<IcHsMarSpec>;
#[doc = "Register `IC_HS_MAR` writer"]
pub type W = crate::W<IcHsMarSpec>;
#[doc = "Field `IC_HS_MAR` reader - This bit field holds the value of the I2C HS mode master code. HS-mode master codes are reserved 8-bit codes (00001xxx) that are not used for slave addressing or other purposes. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Write at other times have no effect."]
pub type IcHsMarR = crate::FieldReader;
#[doc = "Field `IC_HS_MAR` writer - This bit field holds the value of the I2C HS mode master code. HS-mode master codes are reserved 8-bit codes (00001xxx) that are not used for slave addressing or other purposes. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Write at other times have no effect."]
pub type IcHsMarW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - This bit field holds the value of the I2C HS mode master code. HS-mode master codes are reserved 8-bit codes (00001xxx) that are not used for slave addressing or other purposes. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Write at other times have no effect."]
    #[inline(always)]
    pub fn ic_hs_mar(&self) -> IcHsMarR {
        IcHsMarR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - This bit field holds the value of the I2C HS mode master code. HS-mode master codes are reserved 8-bit codes (00001xxx) that are not used for slave addressing or other purposes. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE register being set to 0. Write at other times have no effect."]
    #[inline(always)]
    pub fn ic_hs_mar(&mut self) -> IcHsMarW<'_, IcHsMarSpec> {
        IcHsMarW::new(self, 0)
    }
}
#[doc = "I2C High Speed Master Mode Code Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_hs_mar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_hs_mar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcHsMarSpec;
impl crate::RegisterSpec for IcHsMarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_hs_mar::R`](R) reader structure"]
impl crate::Readable for IcHsMarSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_hs_mar::W`](W) writer structure"]
impl crate::Writable for IcHsMarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_HS_MAR to value 0"]
impl crate::Resettable for IcHsMarSpec {}
