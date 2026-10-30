#[doc = "Register `REG_BCN_SPACE_PORT1_2` reader"]
pub type R = crate::R<RegBcnSpacePort1_2Spec>;
#[doc = "Register `REG_BCN_SPACE_PORT1_2` writer"]
pub type W = crate::W<RegBcnSpacePort1_2Spec>;
#[doc = "Field `BIT_BCN_SPACE3` reader - sync beacon space of port1"]
pub type BitBcnSpace3R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BCN_SPACE3` writer - sync beacon space of port1"]
pub type BitBcnSpace3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_BCN_SPACE4` reader - Unit is ms. the beacon space of port2 disc beacon. default is 100ms"]
pub type BitBcnSpace4R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BCN_SPACE4` writer - Unit is ms. the beacon space of port2 disc beacon. default is 100ms"]
pub type BitBcnSpace4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - sync beacon space of port1"]
    #[inline(always)]
    pub fn bit_bcn_space3(&self) -> BitBcnSpace3R {
        BitBcnSpace3R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Unit is ms. the beacon space of port2 disc beacon. default is 100ms"]
    #[inline(always)]
    pub fn bit_bcn_space4(&self) -> BitBcnSpace4R {
        BitBcnSpace4R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - sync beacon space of port1"]
    #[inline(always)]
    pub fn bit_bcn_space3(&mut self) -> BitBcnSpace3W<'_, RegBcnSpacePort1_2Spec> {
        BitBcnSpace3W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Unit is ms. the beacon space of port2 disc beacon. default is 100ms"]
    #[inline(always)]
    pub fn bit_bcn_space4(&mut self) -> BitBcnSpace4W<'_, RegBcnSpacePort1_2Spec> {
        BitBcnSpace4W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_space_port1_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_space_port1_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnSpacePort1_2Spec;
impl crate::RegisterSpec for RegBcnSpacePort1_2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_space_port1_2::R`](R) reader structure"]
impl crate::Readable for RegBcnSpacePort1_2Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_space_port1_2::W`](W) writer structure"]
impl crate::Writable for RegBcnSpacePort1_2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_SPACE_PORT1_2 to value 0x0064_0064"]
impl crate::Resettable for RegBcnSpacePort1_2Spec {
    const RESET_VALUE: u32 = 0x0064_0064;
}
