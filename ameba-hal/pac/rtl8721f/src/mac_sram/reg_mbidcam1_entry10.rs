#[doc = "Register `REG_MBIDCAM1_ENTRY10` reader"]
pub type R = crate::R<RegMbidcam1Entry10Spec>;
#[doc = "Register `REG_MBIDCAM1_ENTRY10` writer"]
pub type W = crate::W<RegMbidcam1Entry10Spec>;
#[doc = "Field `MBIDCAM1_ENTRY10_BSSID_MACID1` reader - "]
pub type Mbidcam1Entry10BssidMacid1R = crate::FieldReader<u16>;
#[doc = "Field `MBIDCAM1_ENTRY10_BSSID_MACID1` writer - "]
pub type Mbidcam1Entry10BssidMacid1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBIDCAM1_ENTRY10_DUA_CTS_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry10DuaCtsEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY10_DUA_CTS_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry10DuaCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY10_LSIG_TXOP_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry10LsigTxopEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY10_LSIG_TXOP_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry10LsigTxopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY10_VALID` reader - "]
pub type Mbidcam1Entry10ValidR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY10_VALID` writer - "]
pub type Mbidcam1Entry10ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry10_bssid_macid1(&self) -> Mbidcam1Entry10BssidMacid1R {
        Mbidcam1Entry10BssidMacid1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry10_dua_cts_en(&self) -> Mbidcam1Entry10DuaCtsEnR {
        Mbidcam1Entry10DuaCtsEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry10_lsig_txop_en(&self) -> Mbidcam1Entry10LsigTxopEnR {
        Mbidcam1Entry10LsigTxopEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry10_valid(&self) -> Mbidcam1Entry10ValidR {
        Mbidcam1Entry10ValidR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry10_bssid_macid1(
        &mut self,
    ) -> Mbidcam1Entry10BssidMacid1W<'_, RegMbidcam1Entry10Spec> {
        Mbidcam1Entry10BssidMacid1W::new(self, 0)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry10_dua_cts_en(
        &mut self,
    ) -> Mbidcam1Entry10DuaCtsEnW<'_, RegMbidcam1Entry10Spec> {
        Mbidcam1Entry10DuaCtsEnW::new(self, 16)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry10_lsig_txop_en(
        &mut self,
    ) -> Mbidcam1Entry10LsigTxopEnW<'_, RegMbidcam1Entry10Spec> {
        Mbidcam1Entry10LsigTxopEnW::new(self, 17)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry10_valid(&mut self) -> Mbidcam1Entry10ValidW<'_, RegMbidcam1Entry10Spec> {
        Mbidcam1Entry10ValidW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam1_entry10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcam1_entry10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcam1Entry10Spec;
impl crate::RegisterSpec for RegMbidcam1Entry10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam1_entry10::R`](R) reader structure"]
impl crate::Readable for RegMbidcam1Entry10Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcam1_entry10::W`](W) writer structure"]
impl crate::Writable for RegMbidcam1Entry10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAM1_ENTRY10 to value 0"]
impl crate::Resettable for RegMbidcam1Entry10Spec {}
