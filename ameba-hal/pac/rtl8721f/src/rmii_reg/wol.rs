#[doc = "Register `WOL` reader"]
pub type R = crate::R<WolSpec>;
#[doc = "Register `WOL` writer"]
pub type W = crate::W<WolSpec>;
#[doc = "Field `Wol_cmd` reader - Issue wol command by SW"]
pub type WolCmdR = crate::BitReader;
#[doc = "Field `Wol_cmd` writer - Issue wol command by SW"]
pub type WolCmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Wol_sts` reader - HW enters wol idle state when a. SW issues wol_cmd state and b. HW tx/rx function enter idle state. Wol_sts indicates: 0: HW is not in wol idle state. 1: HW is in wol idle state."]
pub type WolStsR = crate::BitReader;
#[doc = "Field `Wol_pme` reader - HW asserts pme when a.HW entered wol idle state and b.received magic ingress pkt. HW asserts pme to wake up system. Wol_pme indicates: 0: No magic pkt receives. 1: HW had received one magic pkt and system should wake up."]
pub type WolPmeR = crate::BitReader;
#[doc = "Field `wol_imr` reader - Interrupt mask of wol_pme."]
pub type WolImrR = crate::BitReader;
#[doc = "Field `wol_imr` writer - Interrupt mask of wol_pme."]
pub type WolImrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Issue wol command by SW"]
    #[inline(always)]
    pub fn wol_cmd(&self) -> WolCmdR {
        WolCmdR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HW enters wol idle state when a. SW issues wol_cmd state and b. HW tx/rx function enter idle state. Wol_sts indicates: 0: HW is not in wol idle state. 1: HW is in wol idle state."]
    #[inline(always)]
    pub fn wol_sts(&self) -> WolStsR {
        WolStsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HW asserts pme when a.HW entered wol idle state and b.received magic ingress pkt. HW asserts pme to wake up system. Wol_pme indicates: 0: No magic pkt receives. 1: HW had received one magic pkt and system should wake up."]
    #[inline(always)]
    pub fn wol_pme(&self) -> WolPmeR {
        WolPmeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt mask of wol_pme."]
    #[inline(always)]
    pub fn wol_imr(&self) -> WolImrR {
        WolImrR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Issue wol command by SW"]
    #[inline(always)]
    pub fn wol_cmd(&mut self) -> WolCmdW<'_, WolSpec> {
        WolCmdW::new(self, 0)
    }
    #[doc = "Bit 3 - Interrupt mask of wol_pme."]
    #[inline(always)]
    pub fn wol_imr(&mut self) -> WolImrW<'_, WolSpec> {
        WolImrW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`wol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WolSpec;
impl crate::RegisterSpec for WolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wol::R`](R) reader structure"]
impl crate::Readable for WolSpec {}
#[doc = "`write(|w| ..)` method takes [`wol::W`](W) writer structure"]
impl crate::Writable for WolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WOL to value 0"]
impl crate::Resettable for WolSpec {}
