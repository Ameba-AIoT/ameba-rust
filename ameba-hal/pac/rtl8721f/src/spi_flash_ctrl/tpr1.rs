#[doc = "Register `TPR1` reader"]
pub type R = crate::R<Tpr1Spec>;
#[doc = "Register `TPR1` writer"]
pub type W = crate::W<Tpr1Spec>;
#[doc = "Field `CS_ACTIVE_SETUP` reader - It is used to configure the setup time before the first edge (rising edge for mode0 and falling edge for mode3) of spi_sclk(refer to tCSS or tSLCH ). Note: It can't be configured 0 (if you configure 0 to this 4 bits field, this field will not be updated)."]
pub type CsActiveSetupR = crate::FieldReader;
#[doc = "Field `CS_ACTIVE_SETUP` writer - It is used to configure the setup time before the first edge (rising edge for mode0 and falling edge for mode3) of spi_sclk(refer to tCSS or tSLCH ). Note: It can't be configured 0 (if you configure 0 to this 4 bits field, this field will not be updated)."]
pub type CsActiveSetupW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IDLE_WINDOW` reader - It is used to configure the cycle counts before turn off the spi_pwr when SPIC is IDLE."]
pub type IdleWindowR = crate::FieldReader;
#[doc = "Field `IDLE_WINDOW` writer - It is used to configure the cycle counts before turn off the spi_pwr when SPIC is IDLE."]
pub type IdleWindowW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TPWR` reader - It is used to configure the cycle counts before spi_csn becomes low when spi_pwr turns from low to high."]
pub type TpwrR = crate::FieldReader;
#[doc = "Field `TPWR` writer - It is used to configure the cycle counts before spi_csn becomes low when spi_pwr turns from low to high."]
pub type TpwrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - It is used to configure the setup time before the first edge (rising edge for mode0 and falling edge for mode3) of spi_sclk(refer to tCSS or tSLCH ). Note: It can't be configured 0 (if you configure 0 to this 4 bits field, this field will not be updated)."]
    #[inline(always)]
    pub fn cs_active_setup(&self) -> CsActiveSetupR {
        CsActiveSetupR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:15 - It is used to configure the cycle counts before turn off the spi_pwr when SPIC is IDLE."]
    #[inline(always)]
    pub fn idle_window(&self) -> IdleWindowR {
        IdleWindowR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - It is used to configure the cycle counts before spi_csn becomes low when spi_pwr turns from low to high."]
    #[inline(always)]
    pub fn tpwr(&self) -> TpwrR {
        TpwrR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - It is used to configure the setup time before the first edge (rising edge for mode0 and falling edge for mode3) of spi_sclk(refer to tCSS or tSLCH ). Note: It can't be configured 0 (if you configure 0 to this 4 bits field, this field will not be updated)."]
    #[inline(always)]
    pub fn cs_active_setup(&mut self) -> CsActiveSetupW<'_, Tpr1Spec> {
        CsActiveSetupW::new(self, 0)
    }
    #[doc = "Bits 8:15 - It is used to configure the cycle counts before turn off the spi_pwr when SPIC is IDLE."]
    #[inline(always)]
    pub fn idle_window(&mut self) -> IdleWindowW<'_, Tpr1Spec> {
        IdleWindowW::new(self, 8)
    }
    #[doc = "Bits 16:23 - It is used to configure the cycle counts before spi_csn becomes low when spi_pwr turns from low to high."]
    #[inline(always)]
    pub fn tpwr(&mut self) -> TpwrW<'_, Tpr1Spec> {
        TpwrW::new(self, 16)
    }
}
#[doc = "Timing Parameters Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tpr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tpr1Spec;
impl crate::RegisterSpec for Tpr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpr1::R`](R) reader structure"]
impl crate::Readable for Tpr1Spec {}
#[doc = "`write(|w| ..)` method takes [`tpr1::W`](W) writer structure"]
impl crate::Writable for Tpr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPR1 to value 0x0028_0a01"]
impl crate::Resettable for Tpr1Spec {
    const RESET_VALUE: u32 = 0x0028_0a01;
}
