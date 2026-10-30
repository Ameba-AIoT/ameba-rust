#[doc = "Register `CAN_RAM_FDDATA_7` reader"]
pub type R = crate::R<CanRamFddata7Spec>;
#[doc = "Register `CAN_RAM_FDDATA_7` writer"]
pub type W = crate::W<CanRamFddata7Spec>;
#[doc = "Field `can_ram_data_7_0` reader - data_byte 4*7+0"]
pub type CanRamData7_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_7_0` writer - data_byte 4*7+0"]
pub type CanRamData7_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_7_1` reader - data_byte 4*7+1"]
pub type CanRamData7_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_7_1` writer - data_byte 4*7+1"]
pub type CanRamData7_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_7_2` reader - data_byte 4*7+2"]
pub type CanRamData7_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_7_2` writer - data_byte 4*7+2"]
pub type CanRamData7_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_7_3` reader - data_byte 4*7+3"]
pub type CanRamData7_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_7_3` writer - data_byte 4*7+3"]
pub type CanRamData7_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*7+0"]
    #[inline(always)]
    pub fn can_ram_data_7_0(&self) -> CanRamData7_0R {
        CanRamData7_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*7+1"]
    #[inline(always)]
    pub fn can_ram_data_7_1(&self) -> CanRamData7_1R {
        CanRamData7_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*7+2"]
    #[inline(always)]
    pub fn can_ram_data_7_2(&self) -> CanRamData7_2R {
        CanRamData7_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*7+3"]
    #[inline(always)]
    pub fn can_ram_data_7_3(&self) -> CanRamData7_3R {
        CanRamData7_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*7+0"]
    #[inline(always)]
    pub fn can_ram_data_7_0(&mut self) -> CanRamData7_0W<'_, CanRamFddata7Spec> {
        CanRamData7_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*7+1"]
    #[inline(always)]
    pub fn can_ram_data_7_1(&mut self) -> CanRamData7_1W<'_, CanRamFddata7Spec> {
        CanRamData7_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*7+2"]
    #[inline(always)]
    pub fn can_ram_data_7_2(&mut self) -> CanRamData7_2W<'_, CanRamFddata7Spec> {
        CanRamData7_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*7+3"]
    #[inline(always)]
    pub fn can_ram_data_7_3(&mut self) -> CanRamData7_3W<'_, CanRamFddata7Spec> {
        CanRamData7_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata7Spec;
impl crate::RegisterSpec for CanRamFddata7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_7::R`](R) reader structure"]
impl crate::Readable for CanRamFddata7Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_7::W`](W) writer structure"]
impl crate::Writable for CanRamFddata7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_7 to value 0"]
impl crate::Resettable for CanRamFddata7Spec {}
