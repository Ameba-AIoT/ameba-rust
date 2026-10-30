#[doc = "Register `CAN_RAM_FDDATA_6` reader"]
pub type R = crate::R<CanRamFddata6Spec>;
#[doc = "Register `CAN_RAM_FDDATA_6` writer"]
pub type W = crate::W<CanRamFddata6Spec>;
#[doc = "Field `can_ram_data_6_0` reader - data_byte 4*6+0"]
pub type CanRamData6_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_6_0` writer - data_byte 4*6+0"]
pub type CanRamData6_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_6_1` reader - data_byte 4*6+1"]
pub type CanRamData6_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_6_1` writer - data_byte 4*6+1"]
pub type CanRamData6_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_6_2` reader - data_byte 4*6+2"]
pub type CanRamData6_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_6_2` writer - data_byte 4*6+2"]
pub type CanRamData6_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_6_3` reader - data_byte 4*6+3"]
pub type CanRamData6_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_6_3` writer - data_byte 4*6+3"]
pub type CanRamData6_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*6+0"]
    #[inline(always)]
    pub fn can_ram_data_6_0(&self) -> CanRamData6_0R {
        CanRamData6_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*6+1"]
    #[inline(always)]
    pub fn can_ram_data_6_1(&self) -> CanRamData6_1R {
        CanRamData6_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*6+2"]
    #[inline(always)]
    pub fn can_ram_data_6_2(&self) -> CanRamData6_2R {
        CanRamData6_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*6+3"]
    #[inline(always)]
    pub fn can_ram_data_6_3(&self) -> CanRamData6_3R {
        CanRamData6_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*6+0"]
    #[inline(always)]
    pub fn can_ram_data_6_0(&mut self) -> CanRamData6_0W<'_, CanRamFddata6Spec> {
        CanRamData6_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*6+1"]
    #[inline(always)]
    pub fn can_ram_data_6_1(&mut self) -> CanRamData6_1W<'_, CanRamFddata6Spec> {
        CanRamData6_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*6+2"]
    #[inline(always)]
    pub fn can_ram_data_6_2(&mut self) -> CanRamData6_2W<'_, CanRamFddata6Spec> {
        CanRamData6_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*6+3"]
    #[inline(always)]
    pub fn can_ram_data_6_3(&mut self) -> CanRamData6_3W<'_, CanRamFddata6Spec> {
        CanRamData6_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata6Spec;
impl crate::RegisterSpec for CanRamFddata6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_6::R`](R) reader structure"]
impl crate::Readable for CanRamFddata6Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_6::W`](W) writer structure"]
impl crate::Writable for CanRamFddata6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_6 to value 0"]
impl crate::Resettable for CanRamFddata6Spec {}
