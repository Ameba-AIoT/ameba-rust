#[doc = "Register `CAN_RAM_FDDATA_0` reader"]
pub type R = crate::R<CanRamFddata0Spec>;
#[doc = "Register `CAN_RAM_FDDATA_0` writer"]
pub type W = crate::W<CanRamFddata0Spec>;
#[doc = "Field `can_ram_data_0_0` reader - data_byte 4*0+0"]
pub type CanRamData0_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_0_0` writer - data_byte 4*0+0"]
pub type CanRamData0_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_0_1` reader - data_byte 4*0+1"]
pub type CanRamData0_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_0_1` writer - data_byte 4*0+1"]
pub type CanRamData0_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_0_2` reader - data_byte 4*0+2"]
pub type CanRamData0_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_0_2` writer - data_byte 4*0+2"]
pub type CanRamData0_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_0_3` reader - data_byte 4*0+3"]
pub type CanRamData0_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_0_3` writer - data_byte 4*0+3"]
pub type CanRamData0_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*0+0"]
    #[inline(always)]
    pub fn can_ram_data_0_0(&self) -> CanRamData0_0R {
        CanRamData0_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*0+1"]
    #[inline(always)]
    pub fn can_ram_data_0_1(&self) -> CanRamData0_1R {
        CanRamData0_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*0+2"]
    #[inline(always)]
    pub fn can_ram_data_0_2(&self) -> CanRamData0_2R {
        CanRamData0_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*0+3"]
    #[inline(always)]
    pub fn can_ram_data_0_3(&self) -> CanRamData0_3R {
        CanRamData0_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*0+0"]
    #[inline(always)]
    pub fn can_ram_data_0_0(&mut self) -> CanRamData0_0W<'_, CanRamFddata0Spec> {
        CanRamData0_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*0+1"]
    #[inline(always)]
    pub fn can_ram_data_0_1(&mut self) -> CanRamData0_1W<'_, CanRamFddata0Spec> {
        CanRamData0_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*0+2"]
    #[inline(always)]
    pub fn can_ram_data_0_2(&mut self) -> CanRamData0_2W<'_, CanRamFddata0Spec> {
        CanRamData0_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*0+3"]
    #[inline(always)]
    pub fn can_ram_data_0_3(&mut self) -> CanRamData0_3W<'_, CanRamFddata0Spec> {
        CanRamData0_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata0Spec;
impl crate::RegisterSpec for CanRamFddata0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_0::R`](R) reader structure"]
impl crate::Readable for CanRamFddata0Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_0::W`](W) writer structure"]
impl crate::Writable for CanRamFddata0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_0 to value 0"]
impl crate::Resettable for CanRamFddata0Spec {}
