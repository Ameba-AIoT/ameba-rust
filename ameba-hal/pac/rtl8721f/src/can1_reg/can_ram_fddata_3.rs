#[doc = "Register `CAN_RAM_FDDATA_3` reader"]
pub type R = crate::R<CanRamFddata3Spec>;
#[doc = "Register `CAN_RAM_FDDATA_3` writer"]
pub type W = crate::W<CanRamFddata3Spec>;
#[doc = "Field `can_ram_data_3_0` reader - data_byte 4*3+0"]
pub type CanRamData3_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_3_0` writer - data_byte 4*3+0"]
pub type CanRamData3_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_3_1` reader - data_byte 4*3+1"]
pub type CanRamData3_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_3_1` writer - data_byte 4*3+1"]
pub type CanRamData3_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_3_2` reader - data_byte 4*3+2"]
pub type CanRamData3_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_3_2` writer - data_byte 4*3+2"]
pub type CanRamData3_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_3_3` reader - data_byte 4*3+3"]
pub type CanRamData3_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_3_3` writer - data_byte 4*3+3"]
pub type CanRamData3_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*3+0"]
    #[inline(always)]
    pub fn can_ram_data_3_0(&self) -> CanRamData3_0R {
        CanRamData3_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*3+1"]
    #[inline(always)]
    pub fn can_ram_data_3_1(&self) -> CanRamData3_1R {
        CanRamData3_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*3+2"]
    #[inline(always)]
    pub fn can_ram_data_3_2(&self) -> CanRamData3_2R {
        CanRamData3_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*3+3"]
    #[inline(always)]
    pub fn can_ram_data_3_3(&self) -> CanRamData3_3R {
        CanRamData3_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*3+0"]
    #[inline(always)]
    pub fn can_ram_data_3_0(&mut self) -> CanRamData3_0W<'_, CanRamFddata3Spec> {
        CanRamData3_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*3+1"]
    #[inline(always)]
    pub fn can_ram_data_3_1(&mut self) -> CanRamData3_1W<'_, CanRamFddata3Spec> {
        CanRamData3_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*3+2"]
    #[inline(always)]
    pub fn can_ram_data_3_2(&mut self) -> CanRamData3_2W<'_, CanRamFddata3Spec> {
        CanRamData3_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*3+3"]
    #[inline(always)]
    pub fn can_ram_data_3_3(&mut self) -> CanRamData3_3W<'_, CanRamFddata3Spec> {
        CanRamData3_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata3Spec;
impl crate::RegisterSpec for CanRamFddata3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_3::R`](R) reader structure"]
impl crate::Readable for CanRamFddata3Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_3::W`](W) writer structure"]
impl crate::Writable for CanRamFddata3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_3 to value 0"]
impl crate::Resettable for CanRamFddata3Spec {}
