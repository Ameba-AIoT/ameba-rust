#[doc = "Register `CAN_RAM_FDDATA_10` reader"]
pub type R = crate::R<CanRamFddata10Spec>;
#[doc = "Register `CAN_RAM_FDDATA_10` writer"]
pub type W = crate::W<CanRamFddata10Spec>;
#[doc = "Field `can_ram_data_10_0` reader - data_byte 4*10+0"]
pub type CanRamData10_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_10_0` writer - data_byte 4*10+0"]
pub type CanRamData10_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_10_1` reader - data_byte 4*10+1"]
pub type CanRamData10_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_10_1` writer - data_byte 4*10+1"]
pub type CanRamData10_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_10_2` reader - data_byte 4*10+2"]
pub type CanRamData10_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_10_2` writer - data_byte 4*10+2"]
pub type CanRamData10_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_10_3` reader - data_byte 4*10+3"]
pub type CanRamData10_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_10_3` writer - data_byte 4*10+3"]
pub type CanRamData10_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*10+0"]
    #[inline(always)]
    pub fn can_ram_data_10_0(&self) -> CanRamData10_0R {
        CanRamData10_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*10+1"]
    #[inline(always)]
    pub fn can_ram_data_10_1(&self) -> CanRamData10_1R {
        CanRamData10_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*10+2"]
    #[inline(always)]
    pub fn can_ram_data_10_2(&self) -> CanRamData10_2R {
        CanRamData10_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*10+3"]
    #[inline(always)]
    pub fn can_ram_data_10_3(&self) -> CanRamData10_3R {
        CanRamData10_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*10+0"]
    #[inline(always)]
    pub fn can_ram_data_10_0(&mut self) -> CanRamData10_0W<'_, CanRamFddata10Spec> {
        CanRamData10_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*10+1"]
    #[inline(always)]
    pub fn can_ram_data_10_1(&mut self) -> CanRamData10_1W<'_, CanRamFddata10Spec> {
        CanRamData10_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*10+2"]
    #[inline(always)]
    pub fn can_ram_data_10_2(&mut self) -> CanRamData10_2W<'_, CanRamFddata10Spec> {
        CanRamData10_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*10+3"]
    #[inline(always)]
    pub fn can_ram_data_10_3(&mut self) -> CanRamData10_3W<'_, CanRamFddata10Spec> {
        CanRamData10_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata10Spec;
impl crate::RegisterSpec for CanRamFddata10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_10::R`](R) reader structure"]
impl crate::Readable for CanRamFddata10Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_10::W`](W) writer structure"]
impl crate::Writable for CanRamFddata10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_10 to value 0"]
impl crate::Resettable for CanRamFddata10Spec {}
