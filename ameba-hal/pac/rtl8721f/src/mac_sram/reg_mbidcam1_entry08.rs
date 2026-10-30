#[doc = "Register `REG_MBIDCAM1_ENTRY08` reader"]
pub type R = crate::R<RegMbidcam1Entry08Spec>;
#[doc = "Register `REG_MBIDCAM1_ENTRY08` writer"]
pub type W = crate::W<RegMbidcam1Entry08Spec>;
#[doc = "Field `MBIDCAM1_ENTRY08_BSSID_MACID1` reader - "]
pub type Mbidcam1Entry08BssidMacid1R = crate::FieldReader<u16>;
#[doc = "Field `MBIDCAM1_ENTRY08_BSSID_MACID1` writer - "]
pub type Mbidcam1Entry08BssidMacid1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBIDCAM1_ENTRY08_DUA_CTS_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry08DuaCtsEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY08_DUA_CTS_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry08DuaCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY08_LSIG_TXOP_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry08LsigTxopEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY08_LSIG_TXOP_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry08LsigTxopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY08_VALID` reader - "]
pub type Mbidcam1Entry08ValidR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY08_VALID` writer - "]
pub type Mbidcam1Entry08ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry08_bssid_macid1(&self) -> Mbidcam1Entry08BssidMacid1R {
        Mbidcam1Entry08BssidMacid1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry08_dua_cts_en(&self) -> Mbidcam1Entry08DuaCtsEnR {
        Mbidcam1Entry08DuaCtsEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry08_lsig_txop_en(&self) -> Mbidcam1Entry08LsigTxopEnR {
        Mbidcam1Entry08LsigTxopEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry08_valid(&self) -> Mbidcam1Entry08ValidR {
        Mbidcam1Entry08ValidR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry08_bssid_macid1(
        &mut self,
    ) -> Mbidcam1Entry08BssidMacid1W<'_, RegMbidcam1Entry08Spec> {
        Mbidcam1Entry08BssidMacid1W::new(self, 0)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry08_dua_cts_en(
        &mut self,
    ) -> Mbidcam1Entry08DuaCtsEnW<'_, RegMbidcam1Entry08Spec> {
        Mbidcam1Entry08DuaCtsEnW::new(self, 16)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry08_lsig_txop_en(
        &mut self,
    ) -> Mbidcam1Entry08LsigTxopEnW<'_, RegMbidcam1Entry08Spec> {
        Mbidcam1Entry08LsigTxopEnW::new(self, 17)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry08_valid(&mut self) -> Mbidcam1Entry08ValidW<'_, RegMbidcam1Entry08Spec> {
        Mbidcam1Entry08ValidW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam1_entry08::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcam1_entry08::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcam1Entry08Spec;
impl crate::RegisterSpec for RegMbidcam1Entry08Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam1_entry08::R`](R) reader structure"]
impl crate::Readable for RegMbidcam1Entry08Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcam1_entry08::W`](W) writer structure"]
impl crate::Writable for RegMbidcam1Entry08Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAM1_ENTRY08 to value 0"]
impl crate::Resettable for RegMbidcam1Entry08Spec {}
