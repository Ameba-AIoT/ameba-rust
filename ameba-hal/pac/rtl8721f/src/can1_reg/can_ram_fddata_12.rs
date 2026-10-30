#[doc = "Register `CAN_RAM_FDDATA_12` reader"]
pub type R = crate::R<CanRamFddata12Spec>;
#[doc = "Register `CAN_RAM_FDDATA_12` writer"]
pub type W = crate::W<CanRamFddata12Spec>;
#[doc = "Field `can_ram_data_12_0` reader - data_byte 4*12+0"]
pub type CanRamData12_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_12_0` writer - data_byte 4*12+0"]
pub type CanRamData12_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_12_1` reader - data_byte 4*12+1"]
pub type CanRamData12_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_12_1` writer - data_byte 4*12+1"]
pub type CanRamData12_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_12_2` reader - data_byte 4*12+2"]
pub type CanRamData12_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_12_2` writer - data_byte 4*12+2"]
pub type CanRamData12_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_12_3` reader - data_byte 4*12+3"]
pub type CanRamData12_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_12_3` writer - data_byte 4*12+3"]
pub type CanRamData12_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*12+0"]
    #[inline(always)]
    pub fn can_ram_data_12_0(&self) -> CanRamData12_0R {
        CanRamData12_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*12+1"]
    #[inline(always)]
    pub fn can_ram_data_12_1(&self) -> CanRamData12_1R {
        CanRamData12_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*12+2"]
    #[inline(always)]
    pub fn can_ram_data_12_2(&self) -> CanRamData12_2R {
        CanRamData12_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*12+3"]
    #[inline(always)]
    pub fn can_ram_data_12_3(&self) -> CanRamData12_3R {
        CanRamData12_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*12+0"]
    #[inline(always)]
    pub fn can_ram_data_12_0(&mut self) -> CanRamData12_0W<'_, CanRamFddata12Spec> {
        CanRamData12_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*12+1"]
    #[inline(always)]
    pub fn can_ram_data_12_1(&mut self) -> CanRamData12_1W<'_, CanRamFddata12Spec> {
        CanRamData12_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*12+2"]
    #[inline(always)]
    pub fn can_ram_data_12_2(&mut self) -> CanRamData12_2W<'_, CanRamFddata12Spec> {
        CanRamData12_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*12+3"]
    #[inline(always)]
    pub fn can_ram_data_12_3(&mut self) -> CanRamData12_3W<'_, CanRamFddata12Spec> {
        CanRamData12_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata12Spec;
impl crate::RegisterSpec for CanRamFddata12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_12::R`](R) reader structure"]
impl crate::Readable for CanRamFddata12Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_12::W`](W) writer structure"]
impl crate::Writable for CanRamFddata12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_12 to value 0"]
impl crate::Resettable for CanRamFddata12Spec {}
