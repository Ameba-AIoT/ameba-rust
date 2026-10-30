#[doc = "Register `REG_MACID_SLEEP1` reader"]
pub type R = crate::R<RegMacidSleep1Spec>;
#[doc = "Register `REG_MACID_SLEEP1` writer"]
pub type W = crate::W<RegMacidSleep1Spec>;
#[doc = "Field `BIT_MACID63_32_PKTSLEEP` reader - Macid 63~Macid 32 Packet Sleep (per bit controlled)"]
pub type BitMacid63_32PktsleepR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID63_32_PKTSLEEP` writer - Macid 63~Macid 32 Packet Sleep (per bit controlled)"]
pub type BitMacid63_32PktsleepW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid 63~Macid 32 Packet Sleep (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid63_32_pktsleep(&self) -> BitMacid63_32PktsleepR {
        BitMacid63_32PktsleepR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid 63~Macid 32 Packet Sleep (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid63_32_pktsleep(&mut self) -> BitMacid63_32PktsleepW<'_, RegMacidSleep1Spec> {
        BitMacid63_32PktsleepW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_sleep1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_sleep1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidSleep1Spec;
impl crate::RegisterSpec for RegMacidSleep1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_sleep1::R`](R) reader structure"]
impl crate::Readable for RegMacidSleep1Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_sleep1::W`](W) writer structure"]
impl crate::Writable for RegMacidSleep1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_SLEEP1 to value 0"]
impl crate::Resettable for RegMacidSleep1Spec {}
