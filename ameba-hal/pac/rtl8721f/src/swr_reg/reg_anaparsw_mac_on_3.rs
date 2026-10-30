#[doc = "Register `REG_ANAPARSW_MAC_ON_3` reader"]
pub type R = crate::R<RegAnaparswMacOn3Spec>;
#[doc = "Register `REG_ANAPARSW_MAC_ON_3` writer"]
pub type W = crate::W<RegAnaparswMacOn3Spec>;
#[doc = "Field `EN_BOD` reader - dummy"]
pub type EnBodR = crate::FieldReader;
#[doc = "Field `EN_BOD` writer - dummy"]
pub type EnBodW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BOD0_VH_SEL` reader - dummy"]
pub type Bod0VhSelR = crate::FieldReader;
#[doc = "Field `BOD0_VH_SEL` writer - dummy"]
pub type Bod0VhSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BOD0_VL_SEL` reader - dummy"]
pub type Bod0VlSelR = crate::FieldReader;
#[doc = "Field `BOD0_VL_SEL` writer - dummy"]
pub type Bod0VlSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BOD1_VH_SEL` reader - NO use in RL6973"]
pub type Bod1VhSelR = crate::FieldReader;
#[doc = "Field `BOD1_VH_SEL` writer - NO use in RL6973"]
pub type Bod1VhSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BOD1_VL_SEL` reader - NO use in RL6973"]
pub type Bod1VlSelR = crate::FieldReader;
#[doc = "Field `BOD1_VL_SEL` writer - NO use in RL6973"]
pub type Bod1VlSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `null0` reader - NO use in RL6973"]
pub type Null0R = crate::FieldReader;
#[doc = "Field `null0` writer - NO use in RL6973"]
pub type Null0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - dummy"]
    #[inline(always)]
    pub fn en_bod(&self) -> EnBodR {
        EnBodR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - dummy"]
    #[inline(always)]
    pub fn bod0_vh_sel(&self) -> Bod0VhSelR {
        Bod0VhSelR::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:13 - dummy"]
    #[inline(always)]
    pub fn bod0_vl_sel(&self) -> Bod0VlSelR {
        Bod0VlSelR::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - NO use in RL6973"]
    #[inline(always)]
    pub fn bod1_vh_sel(&self) -> Bod1VhSelR {
        Bod1VhSelR::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - NO use in RL6973"]
    #[inline(always)]
    pub fn bod1_vl_sel(&self) -> Bod1VlSelR {
        Bod1VlSelR::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:31 - NO use in RL6973"]
    #[inline(always)]
    pub fn null0(&self) -> Null0R {
        Null0R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - dummy"]
    #[inline(always)]
    pub fn en_bod(&mut self) -> EnBodW<'_, RegAnaparswMacOn3Spec> {
        EnBodW::new(self, 0)
    }
    #[doc = "Bits 4:8 - dummy"]
    #[inline(always)]
    pub fn bod0_vh_sel(&mut self) -> Bod0VhSelW<'_, RegAnaparswMacOn3Spec> {
        Bod0VhSelW::new(self, 4)
    }
    #[doc = "Bits 9:13 - dummy"]
    #[inline(always)]
    pub fn bod0_vl_sel(&mut self) -> Bod0VlSelW<'_, RegAnaparswMacOn3Spec> {
        Bod0VlSelW::new(self, 9)
    }
    #[doc = "Bits 14:18 - NO use in RL6973"]
    #[inline(always)]
    pub fn bod1_vh_sel(&mut self) -> Bod1VhSelW<'_, RegAnaparswMacOn3Spec> {
        Bod1VhSelW::new(self, 14)
    }
    #[doc = "Bits 19:23 - NO use in RL6973"]
    #[inline(always)]
    pub fn bod1_vl_sel(&mut self) -> Bod1VlSelW<'_, RegAnaparswMacOn3Spec> {
        Bod1VlSelW::new(self, 19)
    }
    #[doc = "Bits 24:31 - NO use in RL6973"]
    #[inline(always)]
    pub fn null0(&mut self) -> Null0W<'_, RegAnaparswMacOn3Spec> {
        Null0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anaparsw_mac_on_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anaparsw_mac_on_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparswMacOn3Spec;
impl crate::RegisterSpec for RegAnaparswMacOn3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anaparsw_mac_on_3::R`](R) reader structure"]
impl crate::Readable for RegAnaparswMacOn3Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_anaparsw_mac_on_3::W`](W) writer structure"]
impl crate::Writable for RegAnaparswMacOn3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPARSW_MAC_ON_3 to value 0xc01f_d8f0"]
impl crate::Resettable for RegAnaparswMacOn3Spec {
    const RESET_VALUE: u32 = 0xc01f_d8f0;
}
