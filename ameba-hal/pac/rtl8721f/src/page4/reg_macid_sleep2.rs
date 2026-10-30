#[doc = "Register `REG_MACID_SLEEP2` reader"]
pub type R = crate::R<RegMacidSleep2Spec>;
#[doc = "Register `REG_MACID_SLEEP2` writer"]
pub type W = crate::W<RegMacidSleep2Spec>;
#[doc = "Field `BIT_MACID95_64PKTSLEEP` reader - Macid 95~Macid 64 Packet Sleep (per bit controlled)"]
pub type BitMacid95_64pktsleepR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID95_64PKTSLEEP` writer - Macid 95~Macid 64 Packet Sleep (per bit controlled)"]
pub type BitMacid95_64pktsleepW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid 95~Macid 64 Packet Sleep (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid95_64pktsleep(&self) -> BitMacid95_64pktsleepR {
        BitMacid95_64pktsleepR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid 95~Macid 64 Packet Sleep (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid95_64pktsleep(&mut self) -> BitMacid95_64pktsleepW<'_, RegMacidSleep2Spec> {
        BitMacid95_64pktsleepW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_sleep2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_sleep2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidSleep2Spec;
impl crate::RegisterSpec for RegMacidSleep2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_sleep2::R`](R) reader structure"]
impl crate::Readable for RegMacidSleep2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_sleep2::W`](W) writer structure"]
impl crate::Writable for RegMacidSleep2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_SLEEP2 to value 0"]
impl crate::Resettable for RegMacidSleep2Spec {}
