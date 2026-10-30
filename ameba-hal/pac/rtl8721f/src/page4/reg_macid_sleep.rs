#[doc = "Register `REG_MACID_SLEEP` reader"]
pub type R = crate::R<RegMacidSleepSpec>;
#[doc = "Register `REG_MACID_SLEEP` writer"]
pub type W = crate::W<RegMacidSleepSpec>;
#[doc = "Field `BIT_MACID31_0_PKTSLEEP` reader - Macid31 ~ Macid 0Packet Sleep. (per bit controlled)"]
pub type BitMacid31_0PktsleepR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MACID31_0_PKTSLEEP` writer - Macid31 ~ Macid 0Packet Sleep. (per bit controlled)"]
pub type BitMacid31_0PktsleepW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Macid31 ~ Macid 0Packet Sleep. (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid31_0_pktsleep(&self) -> BitMacid31_0PktsleepR {
        BitMacid31_0PktsleepR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Macid31 ~ Macid 0Packet Sleep. (per bit controlled)"]
    #[inline(always)]
    pub fn bit_macid31_0_pktsleep(&mut self) -> BitMacid31_0PktsleepW<'_, RegMacidSleepSpec> {
        BitMacid31_0PktsleepW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_macid_sleep::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_macid_sleep::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMacidSleepSpec;
impl crate::RegisterSpec for RegMacidSleepSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_macid_sleep::R`](R) reader structure"]
impl crate::Readable for RegMacidSleepSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_macid_sleep::W`](W) writer structure"]
impl crate::Writable for RegMacidSleepSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MACID_SLEEP to value 0"]
impl crate::Resettable for RegMacidSleepSpec {}
