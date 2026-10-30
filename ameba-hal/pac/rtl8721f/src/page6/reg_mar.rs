#[doc = "Register `REG_MAR` reader"]
pub type R = crate::R<RegMarSpec>;
#[doc = "Register `REG_MAR` writer"]
pub type W = crate::W<RegMarSpec>;
#[doc = "Field `BIT_MAR_V1` reader - Multicast Address Register BIT\\[31:0\\]. The value of this register is written by the driver to specify the type of multicast address supported by RTL8192D. The value is created by a hashed calculation that is also implemented by the hardware for multicast address filtering."]
pub type BitMarV1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_MAR_V1` writer - Multicast Address Register BIT\\[31:0\\]. The value of this register is written by the driver to specify the type of multicast address supported by RTL8192D. The value is created by a hashed calculation that is also implemented by the hardware for multicast address filtering."]
pub type BitMarV1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast Address Register BIT\\[31:0\\]. The value of this register is written by the driver to specify the type of multicast address supported by RTL8192D. The value is created by a hashed calculation that is also implemented by the hardware for multicast address filtering."]
    #[inline(always)]
    pub fn bit_mar_v1(&self) -> BitMarV1R {
        BitMarV1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Multicast Address Register BIT\\[31:0\\]. The value of this register is written by the driver to specify the type of multicast address supported by RTL8192D. The value is created by a hashed calculation that is also implemented by the hardware for multicast address filtering."]
    #[inline(always)]
    pub fn bit_mar_v1(&mut self) -> BitMarV1W<'_, RegMarSpec> {
        BitMarV1W::new(self, 0)
    }
}
#[doc = "Multicast Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMarSpec;
impl crate::RegisterSpec for RegMarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mar::R`](R) reader structure"]
impl crate::Readable for RegMarSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mar::W`](W) writer structure"]
impl crate::Writable for RegMarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MAR to value 0"]
impl crate::Resettable for RegMarSpec {}
