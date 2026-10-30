#[doc = "Register `IC_SDA_HOLD` reader"]
pub type R = crate::R<IcSdaHoldSpec>;
#[doc = "Register `IC_SDA_HOLD` writer"]
pub type W = crate::W<IcSdaHoldSpec>;
#[doc = "Field `IC_SDA_HOLD` reader - Set the required SDA hold time in units of ic_clk period."]
pub type IcSdaHoldR = crate::FieldReader<u16>;
#[doc = "Field `IC_SDA_HOLD` writer - Set the required SDA hold time in units of ic_clk period."]
pub type IcSdaHoldW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Set the required SDA hold time in units of ic_clk period."]
    #[inline(always)]
    pub fn ic_sda_hold(&self) -> IcSdaHoldR {
        IcSdaHoldR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Set the required SDA hold time in units of ic_clk period."]
    #[inline(always)]
    pub fn ic_sda_hold(&mut self) -> IcSdaHoldW<'_, IcSdaHoldSpec> {
        IcSdaHoldW::new(self, 0)
    }
}
#[doc = "I2C SDA Hold Time Length Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_sda_hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_sda_hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcSdaHoldSpec;
impl crate::RegisterSpec for IcSdaHoldSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_sda_hold::R`](R) reader structure"]
impl crate::Readable for IcSdaHoldSpec {}
#[doc = "`write(|w| ..)` method takes [`ic_sda_hold::W`](W) writer structure"]
impl crate::Writable for IcSdaHoldSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_SDA_HOLD to value 0x01"]
impl crate::Resettable for IcSdaHoldSpec {
    const RESET_VALUE: u32 = 0x01;
}
