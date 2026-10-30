#[doc = "Register `REG_MACID4` reader"]
pub type R = crate::R<RegMacid4Spec>;
#[doc = "Register `REG_MACID4` writer"]
pub type W = crate::W<RegMacid4Spec>;
#[doc = "Field `BIT_MACID4_V1` reader - MAC Identification Number 4 Register"]
pub type BitMacid4V1R = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID4_V1` writer - MAC Identification Number 4 Register"]
pub type BitMacid4V1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Identification Number 4 Register"]
    #[inline(always)]
    pub fn bit_macid4_v1(&self) -> BitMacid4V1R {
        BitMacid4V1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Identification Number 4 Register"]
    #[inline(always)]
    pub fn bit_macid4_v1(&mut self) -> BitMacid4V1W<'_, RegMacid4Spec> {
        BitMacid4V1W::new(self, 0)
    }
}
#[doc = "MAC ID4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacid4Spec;
impl crate::RegisterSpec for RegMacid4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid4::R`](R) reader structure"]
impl crate::Readable for RegMacid4Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid4::W`](W) writer structure"]
impl crate::Writable for RegMacid4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID4 to value 0"]
impl crate::Resettable for RegMacid4Spec {}
