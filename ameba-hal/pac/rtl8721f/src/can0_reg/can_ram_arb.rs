#[doc = "Register `CAN_RAM_ARB` reader"]
pub type R = crate::R<CanRamArbSpec>;
#[doc = "Register `CAN_RAM_ARB` writer"]
pub type W = crate::W<CanRamArbSpec>;
#[doc = "Field `can_ram_id` reader - can frame ID, including high 11 bit standard ID and low 18 bit extend ID"]
pub type CanRamIdR = crate::FieldReader<u32>;
#[doc = "Field `can_ram_id` writer - can frame ID, including high 11 bit standard ID and low 18 bit extend ID"]
pub type CanRamIdW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `can_ram_ide` reader - can frame IDE bit, determine standard or extend format"]
pub type CanRamIdeR = crate::BitReader;
#[doc = "Field `can_ram_ide` writer - can frame IDE bit, determine standard or extend format"]
pub type CanRamIdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_rtr` reader - can frame RTR bit, determine DATA or REMOTE frame"]
pub type CanRamRtrR = crate::BitReader;
#[doc = "Field `can_ram_rtr` writer - can frame RTR bit, determine DATA or REMOTE frame"]
pub type CanRamRtrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - can frame ID, including high 11 bit standard ID and low 18 bit extend ID"]
    #[inline(always)]
    pub fn can_ram_id(&self) -> CanRamIdR {
        CanRamIdR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - can frame IDE bit, determine standard or extend format"]
    #[inline(always)]
    pub fn can_ram_ide(&self) -> CanRamIdeR {
        CanRamIdeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - can frame RTR bit, determine DATA or REMOTE frame"]
    #[inline(always)]
    pub fn can_ram_rtr(&self) -> CanRamRtrR {
        CanRamRtrR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - can frame ID, including high 11 bit standard ID and low 18 bit extend ID"]
    #[inline(always)]
    pub fn can_ram_id(&mut self) -> CanRamIdW<'_, CanRamArbSpec> {
        CanRamIdW::new(self, 0)
    }
    #[doc = "Bit 29 - can frame IDE bit, determine standard or extend format"]
    #[inline(always)]
    pub fn can_ram_ide(&mut self) -> CanRamIdeW<'_, CanRamArbSpec> {
        CanRamIdeW::new(self, 29)
    }
    #[doc = "Bit 30 - can frame RTR bit, determine DATA or REMOTE frame"]
    #[inline(always)]
    pub fn can_ram_rtr(&mut self) -> CanRamRtrW<'_, CanRamArbSpec> {
        CanRamRtrW::new(self, 30)
    }
}
#[doc = "CAN message buffer arb field configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_arb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_arb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamArbSpec;
impl crate::RegisterSpec for CanRamArbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_arb::R`](R) reader structure"]
impl crate::Readable for CanRamArbSpec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_arb::W`](W) writer structure"]
impl crate::Writable for CanRamArbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_ARB to value 0"]
impl crate::Resettable for CanRamArbSpec {}
