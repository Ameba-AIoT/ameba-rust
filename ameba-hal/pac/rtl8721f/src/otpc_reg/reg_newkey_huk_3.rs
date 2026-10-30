#[doc = "Register `REG_NEWKEY_HUK_3` reader"]
pub type R = crate::R<RegNewkeyHuk3Spec>;
#[doc = "Field `HUK_3_0` reader - "]
pub type Huk3_0R = crate::FieldReader;
#[doc = "Field `HUK_3_1` reader - "]
pub type Huk3_1R = crate::FieldReader;
#[doc = "Field `HUK_3_2` reader - "]
pub type Huk3_2R = crate::FieldReader;
#[doc = "Field `HUK_3_3` reader - "]
pub type Huk3_3R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn huk_3_0(&self) -> Huk3_0R {
        Huk3_0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn huk_3_1(&self) -> Huk3_1R {
        Huk3_1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn huk_3_2(&self) -> Huk3_2R {
        Huk3_2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn huk_3_3(&self) -> Huk3_3R {
        Huk3_3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_newkey_huk_3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegNewkeyHuk3Spec;
impl crate::RegisterSpec for RegNewkeyHuk3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_newkey_huk_3::R`](R) reader structure"]
impl crate::Readable for RegNewkeyHuk3Spec {}
#[doc = "`reset()` method sets REG_NEWKEY_HUK_3 to value 0xffff_ffff"]
impl crate::Resettable for RegNewkeyHuk3Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
