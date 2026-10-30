#[doc = "Register `IC_SDA_SETUP` reader"]
pub type R = crate::R<IcSdaSetupSpec>;
#[doc = "Register `IC_SDA_SETUP` writer"]
pub type W = crate::W<IcSdaSetupSpec>;
#[doc = "Field `IC_SDA_SETUP` reader - Set the required SDA setup time in units of ic_clk period."]
pub type IcSdaSetupR = crate::FieldReader;
#[doc = "Field `IC_SDA_SETUP` writer - Set the required SDA setup time in units of ic_clk period."]
pub type IcSdaSetupW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Set the required SDA setup time in units of ic_clk period."]
    #[inline(always)]
    pub fn ic_sda_setup(&self) -> IcSdaSetupR {
        IcSdaSetupR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set the required SDA setup time in units of ic_clk period."]
    #[inline(always)]
    pub fn ic_sda_setup(&mut self) -> IcSdaSetupW<'_, IcSdaSetupSpec> {
        IcSdaSetupW::new(self, 0)
    }
}
#[doc = "I2C SDA Setup Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_sda_setup::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sda_setup::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcSdaSetupSpec;
impl crate::RegisterSpec for IcSdaSetupSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_sda_setup::R`](R) reader structure"]
impl crate::Readable for IcSdaSetupSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_sda_setup::W`](W) writer structure"]
impl crate::Writable for IcSdaSetupSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_SDA_SETUP to value 0x05"]
impl crate::Resettable for IcSdaSetupSpec {
    const RESET_VALUE: u32 = 0x05;
}
