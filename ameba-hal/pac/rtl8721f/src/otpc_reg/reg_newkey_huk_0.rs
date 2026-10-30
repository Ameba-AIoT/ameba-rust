#[doc = "Register `REG_NEWKEY_HUK_0` reader"]
pub type R = crate::R<RegNewkeyHuk0Spec>;
#[doc = "Field `HUK_0_0` reader - "]
pub type Huk0_0R = crate::FieldReader;
#[doc = "Field `HUK_0_1` reader - "]
pub type Huk0_1R = crate::FieldReader;
#[doc = "Field `HUK_0_2` reader - "]
pub type Huk0_2R = crate::FieldReader;
#[doc = "Field `HUK_0_3` reader - "]
pub type Huk0_3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn huk_0_0(&self) -> Huk0_0R {
        Huk0_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn huk_0_1(&self) -> Huk0_1R {
        Huk0_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn huk_0_2(&self) -> Huk0_2R {
        Huk0_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn huk_0_3(&self) -> Huk0_3R {
        Huk0_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_newkey_huk_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNewkeyHuk0Spec;
impl crate::RegisterSpec for RegNewkeyHuk0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_newkey_huk_0::R`](R) reader structure"]
impl crate::Readable for RegNewkeyHuk0Spec {}
#[doc = "`reset()` method sets REG_NEWKEY_HUK_0 to value 0xffff_ffff"]
impl crate::Resettable for RegNewkeyHuk0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
