#[doc = "Register `CAN_RAM_FDDATA_15` reader"]
pub type R = crate::R<CanRamFddata15Spec>;
#[doc = "Register `CAN_RAM_FDDATA_15` writer"]
pub type W = crate::W<CanRamFddata15Spec>;
#[doc = "Field `can_ram_data_15_0` reader - data_byte 4*15+0"]
pub type CanRamData15_0R = crate::FieldReader;
#[doc = "Field `can_ram_data_15_0` writer - data_byte 4*15+0"]
pub type CanRamData15_0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_15_1` reader - data_byte 4*15+1"]
pub type CanRamData15_1R = crate::FieldReader;
#[doc = "Field `can_ram_data_15_1` writer - data_byte 4*15+1"]
pub type CanRamData15_1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_15_2` reader - data_byte 4*15+2"]
pub type CanRamData15_2R = crate::FieldReader;
#[doc = "Field `can_ram_data_15_2` writer - data_byte 4*15+2"]
pub type CanRamData15_2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `can_ram_data_15_3` reader - data_byte 4*15+3"]
pub type CanRamData15_3R = crate::FieldReader;
#[doc = "Field `can_ram_data_15_3` writer - data_byte 4*15+3"]
pub type CanRamData15_3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - data_byte 4*15+0"]
    #[inline(always)]
    pub fn can_ram_data_15_0(&self) -> CanRamData15_0R {
        CanRamData15_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - data_byte 4*15+1"]
    #[inline(always)]
    pub fn can_ram_data_15_1(&self) -> CanRamData15_1R {
        CanRamData15_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - data_byte 4*15+2"]
    #[inline(always)]
    pub fn can_ram_data_15_2(&self) -> CanRamData15_2R {
        CanRamData15_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - data_byte 4*15+3"]
    #[inline(always)]
    pub fn can_ram_data_15_3(&self) -> CanRamData15_3R {
        CanRamData15_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - data_byte 4*15+0"]
    #[inline(always)]
    pub fn can_ram_data_15_0(&mut self) -> CanRamData15_0W<'_, CanRamFddata15Spec> {
        CanRamData15_0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - data_byte 4*15+1"]
    #[inline(always)]
    pub fn can_ram_data_15_1(&mut self) -> CanRamData15_1W<'_, CanRamFddata15Spec> {
        CanRamData15_1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - data_byte 4*15+2"]
    #[inline(always)]
    pub fn can_ram_data_15_2(&mut self) -> CanRamData15_2W<'_, CanRamFddata15Spec> {
        CanRamData15_2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - data_byte 4*15+3"]
    #[inline(always)]
    pub fn can_ram_data_15_3(&mut self) -> CanRamData15_3W<'_, CanRamFddata15Spec> {
        CanRamData15_3W::new(self, 24)
    }
}
#[doc = "CAN data\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ram_fddata_15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ram_fddata_15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanRamFddata15Spec;
impl crate::RegisterSpec for CanRamFddata15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ram_fddata_15::R`](R) reader structure"]
impl crate::Readable for CanRamFddata15Spec {}
#[doc = "`write(|w| ..)` method takes [`can_ram_fddata_15::W`](W) writer structure"]
impl crate::Writable for CanRamFddata15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_RAM_FDDATA_15 to value 0"]
impl crate::Resettable for CanRamFddata15Spec {}
