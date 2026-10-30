#[doc = "Register `REG_MACID1` reader"]
pub type R = crate::R<RegMacid1Spec>;
#[doc = "Register `REG_MACID1` writer"]
pub type W = crate::W<RegMacid1Spec>;
#[doc = "Field `BIT_MACID1_0` reader - MAC Identification Number Register for port 1"]
pub type BitMacid1_0R = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID1_0` writer - MAC Identification Number Register for port 1"]
pub type BitMacid1_0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MAC Identification Number Register for port 1"]
    #[inline(always)]
    pub fn bit_macid1_0(&self) -> BitMacid1_0R {
        BitMacid1_0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MAC Identification Number Register for port 1"]
    #[inline(always)]
    pub fn bit_macid1_0(&mut self) -> BitMacid1_0W<'_, RegMacid1Spec> {
        BitMacid1_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacid1Spec;
impl crate::RegisterSpec for RegMacid1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid1::R`](R) reader structure"]
impl crate::Readable for RegMacid1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid1::W`](W) writer structure"]
impl crate::Writable for RegMacid1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID1 to value 0"]
impl crate::Resettable for RegMacid1Spec {}
