#[doc = "Register `REG_NEWKEY_HUK_2` reader"]
pub type R = crate::R<RegNewkeyHuk2Spec>;
#[doc = "Field `HUK_2_0` reader - "]
pub type Huk2_0R = crate::FieldReader;
#[doc = "Field `HUK_2_1` reader - "]
pub type Huk2_1R = crate::FieldReader;
#[doc = "Field `HUK_2_2` reader - "]
pub type Huk2_2R = crate::FieldReader;
#[doc = "Field `HUK_2_3` reader - "]
pub type Huk2_3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn huk_2_0(&self) -> Huk2_0R {
        Huk2_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn huk_2_1(&self) -> Huk2_1R {
        Huk2_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn huk_2_2(&self) -> Huk2_2R {
        Huk2_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn huk_2_3(&self) -> Huk2_3R {
        Huk2_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_newkey_huk_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNewkeyHuk2Spec;
impl crate::RegisterSpec for RegNewkeyHuk2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_newkey_huk_2::R`](R) reader structure"]
impl crate::Readable for RegNewkeyHuk2Spec {}
#[doc = "`reset()` method sets REG_NEWKEY_HUK_2 to value 0xffff_ffff"]
impl crate::Resettable for RegNewkeyHuk2Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
