#[doc = "Register `RX_NDF` reader"]
pub type R = crate::R<RxNdfSpec>;
#[doc = "Register `RX_NDF` writer"]
pub type W = crate::W<RxNdfSpec>;
#[doc = "Field `RX_NDF` reader - Indicates a number of data frames (unit: Byte). When executing receives operation in user mode, SPIC receives data continuously until data frames are equal to RX_NDF."]
pub type RxNdfR = crate::FieldReader<u32>;
#[doc = "Field `RX_NDF` writer - Indicates a number of data frames (unit: Byte). When executing receives operation in user mode, SPIC receives data continuously until data frames are equal to RX_NDF."]
pub type RxNdfW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Indicates a number of data frames (unit: Byte). When executing receives operation in user mode, SPIC receives data continuously until data frames are equal to RX_NDF."]
    #[inline(always)]
    pub fn rx_ndf(&self) -> RxNdfR {
        RxNdfR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Indicates a number of data frames (unit: Byte). When executing receives operation in user mode, SPIC receives data continuously until data frames are equal to RX_NDF."]
    #[inline(always)]
    pub fn rx_ndf(&mut self) -> RxNdfW<'_, RxNdfSpec> {
        RxNdfW::new(self, 0)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_ndf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_ndf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxNdfSpec;
impl crate::RegisterSpec for RxNdfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_ndf::R`](R) reader structure"]
impl crate::Readable for RxNdfSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_ndf::W`](W) writer structure"]
impl crate::Writable for RxNdfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_NDF to value 0"]
impl crate::Resettable for RxNdfSpec {}
