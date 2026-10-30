#[doc = "Register `CAN_RAM_MASK` reader"]
pub type R = crate::R<CanRamMaskSpec>;
#[doc = "Register `CAN_RAM_MASK` writer"]
pub type W = crate::W<CanRamMaskSpec>;
#[doc = "Field `can_ram_id_mask` reader - can frame ID mask, 0 means the ID bit in CAN_RAM_ARB don't care, 1 means the bit should match."]
pub type CanRamIdMaskR = crate::FieldReader<u32>;
#[doc = "Field `can_ram_id_mask` writer - can frame ID mask, 0 means the ID bit in CAN_RAM_ARB don't care, 1 means the bit should match."]
pub type CanRamIdMaskW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
#[doc = "Field `can_ram_ide_mask` reader - can frame IDE mask, 0 means don't care, 1 means the bit should match"]
pub type CanRamIdeMaskR = crate::BitReader;
#[doc = "Field `can_ram_ide_mask` writer - can frame IDE mask, 0 means don't care, 1 means the bit should match"]
pub type CanRamIdeMaskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `can_ram_rtr_mask` reader - can frame RTR mask, 0 means don't care, 1 means the bit should match"]
pub type CanRamRtrMaskR = crate::BitReader;
#[doc = "Field `can_ram_rtr_mask` writer - can frame RTR mask, 0 means don't care, 1 means the bit should match"]
pub type CanRamRtrMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:28 - can frame ID mask, 0 means the ID bit in CAN_RAM_ARB don't care, 1 means the bit should match."]
    #[inline(always)]
    pub fn can_ram_id_mask(&self) -> CanRamIdMaskR {
        CanRamIdMaskR::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 29 - can frame IDE mask, 0 means don't care, 1 means the bit should match"]
    #[inline(always)]
    pub fn can_ram_ide_mask(&self) -> CanRamIdeMaskR {
        CanRamIdeMaskR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - can frame RTR mask, 0 means don't care, 1 means the bit should match"]
    #[inline(always)]
    pub fn can_ram_rtr_mask(&self) -> CanRamRtrMaskR {
        CanRamRtrMaskR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - can frame ID mask, 0 means the ID bit in CAN_RAM_ARB don't care, 1 means the bit should match."]
    #[inline(always)]
    pub fn can_ram_id_mask(&mut self) -> CanRamIdMaskW<'_, CanRamMaskSpec> {
        CanRamIdMaskW::new(self, 0)
    }
    #[doc = "Bit 29 - can frame IDE mask, 0 means don't care, 1 means the bit should match"]
    #[inline(always)]
    pub fn can_ram_ide_mask(&mut self) -> CanRamIdeMaskW<'_, CanRamMaskSpec> {
        CanRamIdeMaskW::new(self, 29)
    }
    #[doc = "Bit 30 - can frame RTR mask, 0 means don't care, 1 means the bit should match"]
    #[inline(always)]
    pub fn can_ram_rtr_mask(&mut self) -> CanRamRtrMaskW<'_, CanRamMaskSpec> {
        CanRamRtrMaskW::new(self, 30)
    }
}
#[doc = "CAN message buffer arb field mask configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamMaskSpec;
impl crate::RegisterSpec for CanRamMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_mask::R`](R) reader structure"]
impl crate::Readable for CanRamMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_mask::W`](W) writer structure"]
impl crate::Writable for CanRamMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_MASK to value 0"]
impl crate::Resettable for CanRamMaskSpec {}
