#[doc = "Register `REG_MBIDCAM1_ENTRY31` reader"]
pub type R = crate::R<RegMbidcam1Entry31Spec>;
#[doc = "Register `REG_MBIDCAM1_ENTRY31` writer"]
pub type W = crate::W<RegMbidcam1Entry31Spec>;
#[doc = "Field `MBIDCAM1_ENTRY31_BSSID_MACID1` reader - "]
pub type Mbidcam1Entry31BssidMacid1R = crate::FieldReader<u16>;
#[doc = "Field `MBIDCAM1_ENTRY31_BSSID_MACID1` writer - "]
pub type Mbidcam1Entry31BssidMacid1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBIDCAM1_ENTRY31_DUA_CTS_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry31DuaCtsEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY31_DUA_CTS_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry31DuaCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY31_LSIG_TXOP_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry31LsigTxopEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY31_LSIG_TXOP_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry31LsigTxopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY31_VALID` reader - "]
pub type Mbidcam1Entry31ValidR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY31_VALID` writer - "]
pub type Mbidcam1Entry31ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry31_bssid_macid1(&self) -> Mbidcam1Entry31BssidMacid1R {
        Mbidcam1Entry31BssidMacid1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry31_dua_cts_en(&self) -> Mbidcam1Entry31DuaCtsEnR {
        Mbidcam1Entry31DuaCtsEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry31_lsig_txop_en(&self) -> Mbidcam1Entry31LsigTxopEnR {
        Mbidcam1Entry31LsigTxopEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry31_valid(&self) -> Mbidcam1Entry31ValidR {
        Mbidcam1Entry31ValidR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry31_bssid_macid1(
        &mut self,
    ) -> Mbidcam1Entry31BssidMacid1W<'_, RegMbidcam1Entry31Spec> {
        Mbidcam1Entry31BssidMacid1W::new(self, 0)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry31_dua_cts_en(
        &mut self,
    ) -> Mbidcam1Entry31DuaCtsEnW<'_, RegMbidcam1Entry31Spec> {
        Mbidcam1Entry31DuaCtsEnW::new(self, 16)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry31_lsig_txop_en(
        &mut self,
    ) -> Mbidcam1Entry31LsigTxopEnW<'_, RegMbidcam1Entry31Spec> {
        Mbidcam1Entry31LsigTxopEnW::new(self, 17)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry31_valid(&mut self) -> Mbidcam1Entry31ValidW<'_, RegMbidcam1Entry31Spec> {
        Mbidcam1Entry31ValidW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam1_entry31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcam1_entry31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcam1Entry31Spec;
impl crate::RegisterSpec for RegMbidcam1Entry31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam1_entry31::R`](R) reader structure"]
impl crate::Readable for RegMbidcam1Entry31Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcam1_entry31::W`](W) writer structure"]
impl crate::Writable for RegMbidcam1Entry31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAM1_ENTRY31 to value 0"]
impl crate::Resettable for RegMbidcam1Entry31Spec {}
