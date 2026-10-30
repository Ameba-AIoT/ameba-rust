#[doc = "Register `REG_MAR_H` reader"]
pub type R = crate::R<RegMarHSpec>;
#[doc = "Register `REG_MAR_H` writer"]
pub type W = crate::W<RegMarHSpec>;
#[doc = "Field `BIT_MAR_H` reader - Multicast Address Register BIT\\[63:32\\].The value of this register is written by the driver to specify the type of multicast address supported by RTL8192D. The value is created by a hashed calculation that is also implemented by the hardware for multicast address filtering."]
pub type BitMarHR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MAR_H` writer - Multicast Address Register BIT\\[63:32\\].The value of this register is written by the driver to specify the type of multicast address supported by RTL8192D. The value is created by a hashed calculation that is also implemented by the hardware for multicast address filtering."]
pub type BitMarHW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Multicast Address Register BIT\\[63:32\\].The value of this register is written by the driver to specify the type of multicast address supported by RTL8192D. The value is created by a hashed calculation that is also implemented by the hardware for multicast address filtering."]
    #[inline(always)]
    pub fn bit_mar_h(&self) -> BitMarHR {
        BitMarHR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Multicast Address Register BIT\\[63:32\\].The value of this register is written by the driver to specify the type of multicast address supported by RTL8192D. The value is created by a hashed calculation that is also implemented by the hardware for multicast address filtering."]
    #[inline(always)]
    pub fn bit_mar_h(&mut self) -> BitMarHW<'_, RegMarHSpec> {
        BitMarHW::new(self, 0)
    }
}
#[doc = "Multicast Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mar_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mar_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMarHSpec;
impl crate::RegisterSpec for RegMarHSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mar_h::R`](R) reader structure"]
impl crate::Readable for RegMarHSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mar_h::W`](W) writer structure"]
impl crate::Writable for RegMarHSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MAR_H to value 0"]
impl crate::Resettable for RegMarHSpec {}
