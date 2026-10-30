#[doc = "Register `CAN_RAM_FDDATA_8` reader"]
pub type R = crate::R<CanRamFddata8Spec>;
#[doc = "Register `CAN_RAM_FDDATA_8` writer"]
pub type W = crate::W<CanRamFddata8Spec>;
#[doc = "Field `can_ram_data_8_0` reader - data_byte 4*8+0"]
pub type CanRamData8_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_8_0` writer - data_byte 4*8+0"]
pub type CanRamData8_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_8_1` reader - data_byte 4*8+1"]
pub type CanRamData8_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_8_1` writer - data_byte 4*8+1"]
pub type CanRamData8_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_8_2` reader - data_byte 4*8+2"]
pub type CanRamData8_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_8_2` writer - data_byte 4*8+2"]
pub type CanRamData8_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_8_3` reader - data_byte 4*8+3"]
pub type CanRamData8_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_8_3` writer - data_byte 4*8+3"]
pub type CanRamData8_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*8+0"]
    #[inline(always)]
    pub fn can_ram_data_8_0(&self) -> CanRamData8_0R {
        CanRamData8_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*8+1"]
    #[inline(always)]
    pub fn can_ram_data_8_1(&self) -> CanRamData8_1R {
        CanRamData8_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*8+2"]
    #[inline(always)]
    pub fn can_ram_data_8_2(&self) -> CanRamData8_2R {
        CanRamData8_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*8+3"]
    #[inline(always)]
    pub fn can_ram_data_8_3(&self) -> CanRamData8_3R {
        CanRamData8_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*8+0"]
    #[inline(always)]
    pub fn can_ram_data_8_0(&mut self) -> CanRamData8_0W<'_, CanRamFddata8Spec> {
        CanRamData8_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*8+1"]
    #[inline(always)]
    pub fn can_ram_data_8_1(&mut self) -> CanRamData8_1W<'_, CanRamFddata8Spec> {
        CanRamData8_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*8+2"]
    #[inline(always)]
    pub fn can_ram_data_8_2(&mut self) -> CanRamData8_2W<'_, CanRamFddata8Spec> {
        CanRamData8_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*8+3"]
    #[inline(always)]
    pub fn can_ram_data_8_3(&mut self) -> CanRamData8_3W<'_, CanRamFddata8Spec> {
        CanRamData8_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata8Spec;
impl crate::RegisterSpec for CanRamFddata8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_8::R`](R) reader structure"]
impl crate::Readable for CanRamFddata8Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_8::W`](W) writer structure"]
impl crate::Writable for CanRamFddata8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_8 to value 0"]
impl crate::Resettable for CanRamFddata8Spec {}
