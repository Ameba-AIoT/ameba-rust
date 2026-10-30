#[doc = "Register `CAN_RAM_FDDATA_14` reader"]
pub type R = crate::R<CanRamFddata14Spec>;
#[doc = "Register `CAN_RAM_FDDATA_14` writer"]
pub type W = crate::W<CanRamFddata14Spec>;
#[doc = "Field `can_ram_data_14_0` reader - data_byte 4*14+0"]
pub type CanRamData14_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_14_0` writer - data_byte 4*14+0"]
pub type CanRamData14_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_14_1` reader - data_byte 4*14+1"]
pub type CanRamData14_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_14_1` writer - data_byte 4*14+1"]
pub type CanRamData14_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_14_2` reader - data_byte 4*14+2"]
pub type CanRamData14_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_14_2` writer - data_byte 4*14+2"]
pub type CanRamData14_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_14_3` reader - data_byte 4*14+3"]
pub type CanRamData14_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_14_3` writer - data_byte 4*14+3"]
pub type CanRamData14_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*14+0"]
    #[inline(always)]
    pub fn can_ram_data_14_0(&self) -> CanRamData14_0R {
        CanRamData14_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*14+1"]
    #[inline(always)]
    pub fn can_ram_data_14_1(&self) -> CanRamData14_1R {
        CanRamData14_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*14+2"]
    #[inline(always)]
    pub fn can_ram_data_14_2(&self) -> CanRamData14_2R {
        CanRamData14_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*14+3"]
    #[inline(always)]
    pub fn can_ram_data_14_3(&self) -> CanRamData14_3R {
        CanRamData14_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*14+0"]
    #[inline(always)]
    pub fn can_ram_data_14_0(&mut self) -> CanRamData14_0W<'_, CanRamFddata14Spec> {
        CanRamData14_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*14+1"]
    #[inline(always)]
    pub fn can_ram_data_14_1(&mut self) -> CanRamData14_1W<'_, CanRamFddata14Spec> {
        CanRamData14_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*14+2"]
    #[inline(always)]
    pub fn can_ram_data_14_2(&mut self) -> CanRamData14_2W<'_, CanRamFddata14Spec> {
        CanRamData14_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*14+3"]
    #[inline(always)]
    pub fn can_ram_data_14_3(&mut self) -> CanRamData14_3W<'_, CanRamFddata14Spec> {
        CanRamData14_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata14Spec;
impl crate::RegisterSpec for CanRamFddata14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_14::R`](R) reader structure"]
impl crate::Readable for CanRamFddata14Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_14::W`](W) writer structure"]
impl crate::Writable for CanRamFddata14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_14 to value 0"]
impl crate::Resettable for CanRamFddata14Spec {}
