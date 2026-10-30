#[doc = "Register `CAN_RAM_FDDATA_9` reader"]
pub type R = crate::R<CanRamFddata9Spec>;
#[doc = "Register `CAN_RAM_FDDATA_9` writer"]
pub type W = crate::W<CanRamFddata9Spec>;
#[doc = "Field `can_ram_data_9_0` reader - data_byte 4*9+0"]
pub type CanRamData9_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_9_0` writer - data_byte 4*9+0"]
pub type CanRamData9_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_9_1` reader - data_byte 4*9+1"]
pub type CanRamData9_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_9_1` writer - data_byte 4*9+1"]
pub type CanRamData9_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_9_2` reader - data_byte 4*9+2"]
pub type CanRamData9_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_9_2` writer - data_byte 4*9+2"]
pub type CanRamData9_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_9_3` reader - data_byte 4*9+3"]
pub type CanRamData9_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_9_3` writer - data_byte 4*9+3"]
pub type CanRamData9_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*9+0"]
    #[inline(always)]
    pub fn can_ram_data_9_0(&self) -> CanRamData9_0R {
        CanRamData9_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*9+1"]
    #[inline(always)]
    pub fn can_ram_data_9_1(&self) -> CanRamData9_1R {
        CanRamData9_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*9+2"]
    #[inline(always)]
    pub fn can_ram_data_9_2(&self) -> CanRamData9_2R {
        CanRamData9_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*9+3"]
    #[inline(always)]
    pub fn can_ram_data_9_3(&self) -> CanRamData9_3R {
        CanRamData9_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*9+0"]
    #[inline(always)]
    pub fn can_ram_data_9_0(&mut self) -> CanRamData9_0W<'_, CanRamFddata9Spec> {
        CanRamData9_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*9+1"]
    #[inline(always)]
    pub fn can_ram_data_9_1(&mut self) -> CanRamData9_1W<'_, CanRamFddata9Spec> {
        CanRamData9_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*9+2"]
    #[inline(always)]
    pub fn can_ram_data_9_2(&mut self) -> CanRamData9_2W<'_, CanRamFddata9Spec> {
        CanRamData9_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*9+3"]
    #[inline(always)]
    pub fn can_ram_data_9_3(&mut self) -> CanRamData9_3W<'_, CanRamFddata9Spec> {
        CanRamData9_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata9Spec;
impl crate::RegisterSpec for CanRamFddata9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_9::R`](R) reader structure"]
impl crate::Readable for CanRamFddata9Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_9::W`](W) writer structure"]
impl crate::Writable for CanRamFddata9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_9 to value 0"]
impl crate::Resettable for CanRamFddata9Spec {}
