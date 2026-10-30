#[doc = "Register `REG_GENTST` reader"]
pub type R = crate::R<RegGentstSpec>;
#[doc = "Register `REG_GENTST` writer"]
pub type W = crate::W<RegGentstSpec>;
#[doc = "Field `BIT_GENTST` reader - Dummy register for MAC I/O REG Read/Write Access TEST"]
pub type BitGentstR = crate::FieldReader<u32>;
#[doc = "Field `BIT_GENTST` writer - Dummy register for MAC I/O REG Read/Write Access TEST"]
pub type BitGentstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Dummy register for MAC I/O REG Read/Write Access TEST"]
    #[inline(always)]
    pub fn bit_gentst(&self) -> BitGentstR {
        BitGentstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Dummy register for MAC I/O REG Read/Write Access TEST"]
    #[inline(always)]
    pub fn bit_gentst(&mut self) -> BitGentstW<'_, RegGentstSpec> {
        BitGentstW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_gentst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_gentst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegGentstSpec;
impl crate::RegisterSpec for RegGentstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_gentst::R`](R) reader structure"]
impl crate::Readable for RegGentstSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_gentst::W`](W) writer structure"]
impl crate::Writable for RegGentstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_GENTST to value 0"]
impl crate::Resettable for RegGentstSpec {}
