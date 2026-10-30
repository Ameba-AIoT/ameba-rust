#[doc = "Register `REG_MBIDCAM1_ENTRY11` reader"]
pub type R = crate::R<RegMbidcam1Entry11Spec>;
#[doc = "Register `REG_MBIDCAM1_ENTRY11` writer"]
pub type W = crate::W<RegMbidcam1Entry11Spec>;
#[doc = "Field `MBIDCAM1_ENTRY11_BSSID_MACID1` reader - "]
pub type Mbidcam1Entry11BssidMacid1R = crate::FieldReader<u16>;
#[doc = "Field `MBIDCAM1_ENTRY11_BSSID_MACID1` writer - "]
pub type Mbidcam1Entry11BssidMacid1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBIDCAM1_ENTRY11_DUA_CTS_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry11DuaCtsEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY11_DUA_CTS_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry11DuaCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY11_LSIG_TXOP_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry11LsigTxopEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY11_LSIG_TXOP_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry11LsigTxopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY11_VALID` reader - "]
pub type Mbidcam1Entry11ValidR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY11_VALID` writer - "]
pub type Mbidcam1Entry11ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry11_bssid_macid1(&self) -> Mbidcam1Entry11BssidMacid1R {
        Mbidcam1Entry11BssidMacid1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry11_dua_cts_en(&self) -> Mbidcam1Entry11DuaCtsEnR {
        Mbidcam1Entry11DuaCtsEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry11_lsig_txop_en(&self) -> Mbidcam1Entry11LsigTxopEnR {
        Mbidcam1Entry11LsigTxopEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry11_valid(&self) -> Mbidcam1Entry11ValidR {
        Mbidcam1Entry11ValidR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry11_bssid_macid1(
        &mut self,
    ) -> Mbidcam1Entry11BssidMacid1W<'_, RegMbidcam1Entry11Spec> {
        Mbidcam1Entry11BssidMacid1W::new(self, 0)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry11_dua_cts_en(
        &mut self,
    ) -> Mbidcam1Entry11DuaCtsEnW<'_, RegMbidcam1Entry11Spec> {
        Mbidcam1Entry11DuaCtsEnW::new(self, 16)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry11_lsig_txop_en(
        &mut self,
    ) -> Mbidcam1Entry11LsigTxopEnW<'_, RegMbidcam1Entry11Spec> {
        Mbidcam1Entry11LsigTxopEnW::new(self, 17)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry11_valid(&mut self) -> Mbidcam1Entry11ValidW<'_, RegMbidcam1Entry11Spec> {
        Mbidcam1Entry11ValidW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam1_entry11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcam1_entry11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcam1Entry11Spec;
impl crate::RegisterSpec for RegMbidcam1Entry11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam1_entry11::R`](R) reader structure"]
impl crate::Readable for RegMbidcam1Entry11Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcam1_entry11::W`](W) writer structure"]
impl crate::Writable for RegMbidcam1Entry11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAM1_ENTRY11 to value 0"]
impl crate::Resettable for RegMbidcam1Entry11Spec {}
