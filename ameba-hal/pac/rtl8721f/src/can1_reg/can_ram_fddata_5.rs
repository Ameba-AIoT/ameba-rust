#[doc = "Register `CAN_RAM_FDDATA_5` reader"]
pub type R = crate::R<CanRamFddata5Spec>;
#[doc = "Register `CAN_RAM_FDDATA_5` writer"]
pub type W = crate::W<CanRamFddata5Spec>;
#[doc = "Field `can_ram_data_5_0` reader - data_byte 4*5+0"]
pub type CanRamData5_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_5_0` writer - data_byte 4*5+0"]
pub type CanRamData5_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_5_1` reader - data_byte 4*5+1"]
pub type CanRamData5_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_5_1` writer - data_byte 4*5+1"]
pub type CanRamData5_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_5_2` reader - data_byte 4*5+2"]
pub type CanRamData5_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_5_2` writer - data_byte 4*5+2"]
pub type CanRamData5_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_5_3` reader - data_byte 4*5+3"]
pub type CanRamData5_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_5_3` writer - data_byte 4*5+3"]
pub type CanRamData5_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*5+0"]
    #[inline(always)]
    pub fn can_ram_data_5_0(&self) -> CanRamData5_0R {
        CanRamData5_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*5+1"]
    #[inline(always)]
    pub fn can_ram_data_5_1(&self) -> CanRamData5_1R {
        CanRamData5_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*5+2"]
    #[inline(always)]
    pub fn can_ram_data_5_2(&self) -> CanRamData5_2R {
        CanRamData5_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*5+3"]
    #[inline(always)]
    pub fn can_ram_data_5_3(&self) -> CanRamData5_3R {
        CanRamData5_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*5+0"]
    #[inline(always)]
    pub fn can_ram_data_5_0(&mut self) -> CanRamData5_0W<'_, CanRamFddata5Spec> {
        CanRamData5_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*5+1"]
    #[inline(always)]
    pub fn can_ram_data_5_1(&mut self) -> CanRamData5_1W<'_, CanRamFddata5Spec> {
        CanRamData5_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*5+2"]
    #[inline(always)]
    pub fn can_ram_data_5_2(&mut self) -> CanRamData5_2W<'_, CanRamFddata5Spec> {
        CanRamData5_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*5+3"]
    #[inline(always)]
    pub fn can_ram_data_5_3(&mut self) -> CanRamData5_3W<'_, CanRamFddata5Spec> {
        CanRamData5_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata5Spec;
impl crate::RegisterSpec for CanRamFddata5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_5::R`](R) reader structure"]
impl crate::Readable for CanRamFddata5Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_5::W`](W) writer structure"]
impl crate::Writable for CanRamFddata5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_5 to value 0"]
impl crate::Resettable for CanRamFddata5Spec {}
