#[doc = "Register `REG_MBIDCAM1_ENTRY03` reader"]
pub type R = crate::R<RegMbidcam1Entry03Spec>;
#[doc = "Register `REG_MBIDCAM1_ENTRY03` writer"]
pub type W = crate::W<RegMbidcam1Entry03Spec>;
#[doc = "Field `MBIDCAM1_ENTRY03_BSSID_MACID1` reader - "]
pub type Mbidcam1Entry03BssidMacid1R = crate::FieldReader<u16>;
#[doc = "Field `MBIDCAM1_ENTRY03_BSSID_MACID1` writer - "]
pub type Mbidcam1Entry03BssidMacid1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBIDCAM1_ENTRY03_DUA_CTS_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry03DuaCtsEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY03_DUA_CTS_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry03DuaCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY03_LSIG_TXOP_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry03LsigTxopEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY03_LSIG_TXOP_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry03LsigTxopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY03_VALID` reader - "]
pub type Mbidcam1Entry03ValidR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY03_VALID` writer - "]
pub type Mbidcam1Entry03ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry03_bssid_macid1(&self) -> Mbidcam1Entry03BssidMacid1R {
        Mbidcam1Entry03BssidMacid1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry03_dua_cts_en(&self) -> Mbidcam1Entry03DuaCtsEnR {
        Mbidcam1Entry03DuaCtsEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry03_lsig_txop_en(&self) -> Mbidcam1Entry03LsigTxopEnR {
        Mbidcam1Entry03LsigTxopEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry03_valid(&self) -> Mbidcam1Entry03ValidR {
        Mbidcam1Entry03ValidR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry03_bssid_macid1(
        &mut self,
    ) -> Mbidcam1Entry03BssidMacid1W<'_, RegMbidcam1Entry03Spec> {
        Mbidcam1Entry03BssidMacid1W::new(self, 0)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry03_dua_cts_en(
        &mut self,
    ) -> Mbidcam1Entry03DuaCtsEnW<'_, RegMbidcam1Entry03Spec> {
        Mbidcam1Entry03DuaCtsEnW::new(self, 16)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry03_lsig_txop_en(
        &mut self,
    ) -> Mbidcam1Entry03LsigTxopEnW<'_, RegMbidcam1Entry03Spec> {
        Mbidcam1Entry03LsigTxopEnW::new(self, 17)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry03_valid(&mut self) -> Mbidcam1Entry03ValidW<'_, RegMbidcam1Entry03Spec> {
        Mbidcam1Entry03ValidW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam1_entry03::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcam1_entry03::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcam1Entry03Spec;
impl crate::RegisterSpec for RegMbidcam1Entry03Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam1_entry03::R`](R) reader structure"]
impl crate::Readable for RegMbidcam1Entry03Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcam1_entry03::W`](W) writer structure"]
impl crate::Writable for RegMbidcam1Entry03Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAM1_ENTRY03 to value 0"]
impl crate::Resettable for RegMbidcam1Entry03Spec {}
