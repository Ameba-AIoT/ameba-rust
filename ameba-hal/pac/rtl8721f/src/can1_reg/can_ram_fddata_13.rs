#[doc = "Register `CAN_RAM_FDDATA_13` reader"]
pub type R = crate::R<CanRamFddata13Spec>;
#[doc = "Register `CAN_RAM_FDDATA_13` writer"]
pub type W = crate::W<CanRamFddata13Spec>;
#[doc = "Field `can_ram_data_13_0` reader - data_byte 4*13+0"]
pub type CanRamData13_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_13_0` writer - data_byte 4*13+0"]
pub type CanRamData13_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_13_1` reader - data_byte 4*13+1"]
pub type CanRamData13_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_13_1` writer - data_byte 4*13+1"]
pub type CanRamData13_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_13_2` reader - data_byte 4*13+2"]
pub type CanRamData13_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_13_2` writer - data_byte 4*13+2"]
pub type CanRamData13_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_13_3` reader - data_byte 4*13+3"]
pub type CanRamData13_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_13_3` writer - data_byte 4*13+3"]
pub type CanRamData13_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*13+0"]
    #[inline(always)]
    pub fn can_ram_data_13_0(&self) -> CanRamData13_0R {
        CanRamData13_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*13+1"]
    #[inline(always)]
    pub fn can_ram_data_13_1(&self) -> CanRamData13_1R {
        CanRamData13_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*13+2"]
    #[inline(always)]
    pub fn can_ram_data_13_2(&self) -> CanRamData13_2R {
        CanRamData13_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*13+3"]
    #[inline(always)]
    pub fn can_ram_data_13_3(&self) -> CanRamData13_3R {
        CanRamData13_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*13+0"]
    #[inline(always)]
    pub fn can_ram_data_13_0(&mut self) -> CanRamData13_0W<'_, CanRamFddata13Spec> {
        CanRamData13_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*13+1"]
    #[inline(always)]
    pub fn can_ram_data_13_1(&mut self) -> CanRamData13_1W<'_, CanRamFddata13Spec> {
        CanRamData13_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*13+2"]
    #[inline(always)]
    pub fn can_ram_data_13_2(&mut self) -> CanRamData13_2W<'_, CanRamFddata13Spec> {
        CanRamData13_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*13+3"]
    #[inline(always)]
    pub fn can_ram_data_13_3(&mut self) -> CanRamData13_3W<'_, CanRamFddata13Spec> {
        CanRamData13_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata13Spec;
impl crate::RegisterSpec for CanRamFddata13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_13::R`](R) reader structure"]
impl crate::Readable for CanRamFddata13Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_13::W`](W) writer structure"]
impl crate::Writable for CanRamFddata13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_13 to value 0"]
impl crate::Resettable for CanRamFddata13Spec {}
