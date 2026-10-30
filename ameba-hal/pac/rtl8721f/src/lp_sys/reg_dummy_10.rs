#[doc = "Register `REG_DUMMY_10` reader"]
pub type R = crate::R<RegDummy10Spec>;
#[doc = "Register `REG_DUMMY_10` writer"]
pub type W = crate::W<RegDummy10Spec>;
#[doc = "Field `dummy` reader - Reserved for HW ECO"]
pub type DummyR = crate::FieldReader<u16>;
#[doc = "Field `dummy` writer - Reserved for HW ECO"]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `aip_ctrl_rsvd1` reader - reserved for aip info indication"]
pub type AipCtrlRsvd1R = crate::FieldReader;
#[doc = "Field `aip_ctrl_rsvd1` writer - reserved for aip info indication"]
pub type AipCtrlRsvd1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `aip_info_rsvd1` reader - reserved for aip info indication"]
pub type AipInfoRsvd1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Reserved for HW ECO"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - reserved for aip info indication"]
    #[inline(always)]
    pub fn aip_ctrl_rsvd1(&self) -> AipCtrlRsvd1R {
        AipCtrlRsvd1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - reserved for aip info indication"]
    #[inline(always)]
    pub fn aip_info_rsvd1(&self) -> AipInfoRsvd1R {
        AipInfoRsvd1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Reserved for HW ECO"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegDummy10Spec> {
        DummyW::new(self, 0)
    }
    #[doc = "Bits 16:23 - reserved for aip info indication"]
    #[inline(always)]
    pub fn aip_ctrl_rsvd1(&mut self) -> AipCtrlRsvd1W<'_, RegDummy10Spec> {
        AipCtrlRsvd1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dummy_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dummy_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDummy10Spec;
impl crate::RegisterSpec for RegDummy10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dummy_10::R`](R) reader structure"]
impl crate::Readable for RegDummy10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_dummy_10::W`](W) writer structure"]
impl crate::Writable for RegDummy10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DUMMY_10 to value 0x00f0_ff00"]
impl crate::Resettable for RegDummy10Spec {
    const RESET_VALUE: u32 = 0x00f0_ff00;
}
