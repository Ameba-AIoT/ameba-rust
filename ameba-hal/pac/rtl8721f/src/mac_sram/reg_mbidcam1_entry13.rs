#[doc = "Register `REG_MBIDCAM1_ENTRY13` reader"]
pub type R = crate::R<RegMbidcam1Entry13Spec>;
#[doc = "Register `REG_MBIDCAM1_ENTRY13` writer"]
pub type W = crate::W<RegMbidcam1Entry13Spec>;
#[doc = "Field `MBIDCAM1_ENTRY13_BSSID_MACID1` reader - "]
pub type Mbidcam1Entry13BssidMacid1R = crate::FieldReader<u16>;
#[doc = "Field `MBIDCAM1_ENTRY13_BSSID_MACID1` writer - "]
pub type Mbidcam1Entry13BssidMacid1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBIDCAM1_ENTRY13_DUA_CTS_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry13DuaCtsEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY13_DUA_CTS_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry13DuaCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY13_LSIG_TXOP_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry13LsigTxopEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY13_LSIG_TXOP_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry13LsigTxopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY13_VALID` reader - "]
pub type Mbidcam1Entry13ValidR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY13_VALID` writer - "]
pub type Mbidcam1Entry13ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry13_bssid_macid1(&self) -> Mbidcam1Entry13BssidMacid1R {
        Mbidcam1Entry13BssidMacid1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry13_dua_cts_en(&self) -> Mbidcam1Entry13DuaCtsEnR {
        Mbidcam1Entry13DuaCtsEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry13_lsig_txop_en(&self) -> Mbidcam1Entry13LsigTxopEnR {
        Mbidcam1Entry13LsigTxopEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry13_valid(&self) -> Mbidcam1Entry13ValidR {
        Mbidcam1Entry13ValidR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry13_bssid_macid1(
        &mut self,
    ) -> Mbidcam1Entry13BssidMacid1W<'_, RegMbidcam1Entry13Spec> {
        Mbidcam1Entry13BssidMacid1W::new(self, 0)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry13_dua_cts_en(
        &mut self,
    ) -> Mbidcam1Entry13DuaCtsEnW<'_, RegMbidcam1Entry13Spec> {
        Mbidcam1Entry13DuaCtsEnW::new(self, 16)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry13_lsig_txop_en(
        &mut self,
    ) -> Mbidcam1Entry13LsigTxopEnW<'_, RegMbidcam1Entry13Spec> {
        Mbidcam1Entry13LsigTxopEnW::new(self, 17)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry13_valid(&mut self) -> Mbidcam1Entry13ValidW<'_, RegMbidcam1Entry13Spec> {
        Mbidcam1Entry13ValidW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam1_entry13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcam1_entry13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcam1Entry13Spec;
impl crate::RegisterSpec for RegMbidcam1Entry13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam1_entry13::R`](R) reader structure"]
impl crate::Readable for RegMbidcam1Entry13Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcam1_entry13::W`](W) writer structure"]
impl crate::Writable for RegMbidcam1Entry13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAM1_ENTRY13 to value 0"]
impl crate::Resettable for RegMbidcam1Entry13Spec {}
