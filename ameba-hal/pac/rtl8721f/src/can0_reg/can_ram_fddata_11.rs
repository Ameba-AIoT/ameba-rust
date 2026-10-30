#[doc = "Register `CAN_RAM_FDDATA_11` reader"]
pub type R = crate::R<CanRamFddata11Spec>;
#[doc = "Register `CAN_RAM_FDDATA_11` writer"]
pub type W = crate::W<CanRamFddata11Spec>;
#[doc = "Field `can_ram_data_11_0` reader - data_byte 4*11+0"]
pub type CanRamData11_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_11_0` writer - data_byte 4*11+0"]
pub type CanRamData11_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_11_1` reader - data_byte 4*11+1"]
pub type CanRamData11_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_11_1` writer - data_byte 4*11+1"]
pub type CanRamData11_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_11_2` reader - data_byte 4*11+2"]
pub type CanRamData11_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_11_2` writer - data_byte 4*11+2"]
pub type CanRamData11_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_11_3` reader - data_byte 4*11+3"]
pub type CanRamData11_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_11_3` writer - data_byte 4*11+3"]
pub type CanRamData11_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*11+0"]
    #[inline(always)]
    pub fn can_ram_data_11_0(&self) -> CanRamData11_0R {
        CanRamData11_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*11+1"]
    #[inline(always)]
    pub fn can_ram_data_11_1(&self) -> CanRamData11_1R {
        CanRamData11_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*11+2"]
    #[inline(always)]
    pub fn can_ram_data_11_2(&self) -> CanRamData11_2R {
        CanRamData11_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*11+3"]
    #[inline(always)]
    pub fn can_ram_data_11_3(&self) -> CanRamData11_3R {
        CanRamData11_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*11+0"]
    #[inline(always)]
    pub fn can_ram_data_11_0(&mut self) -> CanRamData11_0W<'_, CanRamFddata11Spec> {
        CanRamData11_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*11+1"]
    #[inline(always)]
    pub fn can_ram_data_11_1(&mut self) -> CanRamData11_1W<'_, CanRamFddata11Spec> {
        CanRamData11_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*11+2"]
    #[inline(always)]
    pub fn can_ram_data_11_2(&mut self) -> CanRamData11_2W<'_, CanRamFddata11Spec> {
        CanRamData11_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*11+3"]
    #[inline(always)]
    pub fn can_ram_data_11_3(&mut self) -> CanRamData11_3W<'_, CanRamFddata11Spec> {
        CanRamData11_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata11Spec;
impl crate::RegisterSpec for CanRamFddata11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_11::R`](R) reader structure"]
impl crate::Readable for CanRamFddata11Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_11::W`](W) writer structure"]
impl crate::Writable for CanRamFddata11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_11 to value 0"]
impl crate::Resettable for CanRamFddata11Spec {}
