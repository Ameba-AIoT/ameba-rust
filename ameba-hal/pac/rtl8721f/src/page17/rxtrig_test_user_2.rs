#[doc = "Register `RXTRIG_TEST_USER_2` reader"]
pub type R = crate::R<RxtrigTestUser2Spec>;
#[doc = "Register `RXTRIG_TEST_USER_2` writer"]
pub type W = crate::W<RxtrigTestUser2Spec>;
#[doc = "Field `BIT_RXTRIG_USERINFO_2` reader - msb 8bit of user field in trigger frame of test mode"]
pub type BitRxtrigUserinfo2R = crate::FieldReader;
#[doc = "Field `BIT_RXTRIG_USERINFO_2` writer - msb 8bit of user field in trigger frame of test mode"]
pub type BitRxtrigUserinfo2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_RXTRIG_EN` writer - enable rx trigger test mode"]
pub type BitRxtrigEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXTRIG_PORT_SEL` reader - port sel"]
pub type BitRxtrigPortSelR = crate::BitReader;
#[doc = "Field `BIT_RXTRIG_PORT_SEL` writer - port sel"]
pub type BitRxtrigPortSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXTRIG_FCSCHK_EN` reader - 1:enable fcs check"]
pub type BitRxtrigFcschkEnR = crate::BitReader;
#[doc = "Field `BIT_RXTRIG_FCSCHK_EN` writer - 1:enable fcs check"]
pub type BitRxtrigFcschkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RXTRIG_MACID` reader - macid"]
pub type BitRxtrigMacidR = crate::FieldReader;
#[doc = "Field `BIT_RXTRIG_MACID` writer - macid"]
pub type BitRxtrigMacidW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_RXTRIG_BSS_COLOR` reader - bss color"]
pub type BitRxtrigBssColorR = crate::FieldReader;
#[doc = "Field `BIT_RXTRIG_BSS_COLOR` writer - bss color"]
pub type BitRxtrigBssColorW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - msb 8bit of user field in trigger frame of test mode"]
    #[inline(always)]
    pub fn bit_rxtrig_userinfo_2(&self) -> BitRxtrigUserinfo2R {
        BitRxtrigUserinfo2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 17 - port sel"]
    #[inline(always)]
    pub fn bit_rxtrig_port_sel(&self) -> BitRxtrigPortSelR {
        BitRxtrigPortSelR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - 1:enable fcs check"]
    #[inline(always)]
    pub fn bit_rxtrig_fcschk_en(&self) -> BitRxtrigFcschkEnR {
        BitRxtrigFcschkEnR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:25 - macid"]
    #[inline(always)]
    pub fn bit_rxtrig_macid(&self) -> BitRxtrigMacidR {
        BitRxtrigMacidR::new(((self.bits >> 19) & 0x7f) as u8)
    }
    #[doc = "Bits 26:31 - bss color"]
    #[inline(always)]
    pub fn bit_rxtrig_bss_color(&self) -> BitRxtrigBssColorR {
        BitRxtrigBssColorR::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - msb 8bit of user field in trigger frame of test mode"]
    #[inline(always)]
    pub fn bit_rxtrig_userinfo_2(&mut self) -> BitRxtrigUserinfo2W<'_, RxtrigTestUser2Spec> {
        BitRxtrigUserinfo2W::new(self, 0)
    }
    #[doc = "Bit 16 - enable rx trigger test mode"]
    #[inline(always)]
    pub fn bit_rxtrig_en(&mut self) -> BitRxtrigEnW<'_, RxtrigTestUser2Spec> {
        BitRxtrigEnW::new(self, 16)
    }
    #[doc = "Bit 17 - port sel"]
    #[inline(always)]
    pub fn bit_rxtrig_port_sel(&mut self) -> BitRxtrigPortSelW<'_, RxtrigTestUser2Spec> {
        BitRxtrigPortSelW::new(self, 17)
    }
    #[doc = "Bit 18 - 1:enable fcs check"]
    #[inline(always)]
    pub fn bit_rxtrig_fcschk_en(&mut self) -> BitRxtrigFcschkEnW<'_, RxtrigTestUser2Spec> {
        BitRxtrigFcschkEnW::new(self, 18)
    }
    #[doc = "Bits 19:25 - macid"]
    #[inline(always)]
    pub fn bit_rxtrig_macid(&mut self) -> BitRxtrigMacidW<'_, RxtrigTestUser2Spec> {
        BitRxtrigMacidW::new(self, 19)
    }
    #[doc = "Bits 26:31 - bss color"]
    #[inline(always)]
    pub fn bit_rxtrig_bss_color(&mut self) -> BitRxtrigBssColorW<'_, RxtrigTestUser2Spec> {
        BitRxtrigBssColorW::new(self, 26)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`rxtrig_test_user_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxtrig_test_user_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxtrigTestUser2Spec;
impl crate::RegisterSpec for RxtrigTestUser2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxtrig_test_user_2::R`](R) reader structure"]
impl crate::Readable for RxtrigTestUser2Spec {}
#[doc = "`write(|w| ..)` method takes [`rxtrig_test_user_2::W`](W) writer structure"]
impl crate::Writable for RxtrigTestUser2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXTRIG_TEST_USER_2 to value 0"]
impl crate::Resettable for RxtrigTestUser2Spec {}
