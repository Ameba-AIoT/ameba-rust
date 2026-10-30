#[doc = "Register `REG_MACID_SLEEP3` reader"]
pub type R = crate::R<RegMacidSleep3Spec>;
#[doc = "Register `REG_MACID_SLEEP3` writer"]
pub type W = crate::W<RegMacidSleep3Spec>;
#[doc = "Field `BIT_MACID127_96_PKTSLEEP` reader - Macid 127~Macid 96 Packet Sleep (per bit controlled)"]
pub type BitMacid127_96PktsleepR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID127_96_PKTSLEEP` writer - Macid 127~Macid 96 Packet Sleep (per bit controlled)"]
pub type BitMacid127_96PktsleepW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid 127~Macid 96 Packet Sleep (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid127_96_pktsleep(&self) -> BitMacid127_96PktsleepR {
        BitMacid127_96PktsleepR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid 127~Macid 96 Packet Sleep (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid127_96_pktsleep(&mut self) -> BitMacid127_96PktsleepW<'_, RegMacidSleep3Spec> {
        BitMacid127_96PktsleepW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_sleep3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_sleep3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidSleep3Spec;
impl crate::RegisterSpec for RegMacidSleep3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_sleep3::R`](R) reader structure"]
impl crate::Readable for RegMacidSleep3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_sleep3::W`](W) writer structure"]
impl crate::Writable for RegMacidSleep3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_SLEEP3 to value 0"]
impl crate::Resettable for RegMacidSleep3Spec {}
