#[doc = "Register `REG_NEWKEY_HUK_1` reader"]
pub type R = crate::R<RegNewkeyHuk1Spec>;
#[doc = "Field `HUK_1_0` reader - "]
pub type Huk1_0R = crate::FieldReader;
#[doc = "Field `HUK_1_1` reader - "]
pub type Huk1_1R = crate::FieldReader;
#[doc = "Field `HUK_1_2` reader - "]
pub type Huk1_2R = crate::FieldReader;
#[doc = "Field `HUK_1_3` reader - "]
pub type Huk1_3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn huk_1_0(&self) -> Huk1_0R {
        Huk1_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn huk_1_1(&self) -> Huk1_1R {
        Huk1_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn huk_1_2(&self) -> Huk1_2R {
        Huk1_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn huk_1_3(&self) -> Huk1_3R {
        Huk1_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_newkey_huk_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNewkeyHuk1Spec;
impl crate::RegisterSpec for RegNewkeyHuk1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_newkey_huk_1::R`](R) reader structure"]
impl crate::Readable for RegNewkeyHuk1Spec {}
#[doc = "`reset()` method sets REG_NEWKEY_HUK_1 to value 0xffff_ffff"]
impl crate::Resettable for RegNewkeyHuk1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
