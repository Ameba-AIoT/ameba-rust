#[doc = "Register `REG_MBIDCAM1_ENTRY01` reader"]
pub type R = crate::R<RegMbidcam1Entry01Spec>;
#[doc = "Register `REG_MBIDCAM1_ENTRY01` writer"]
pub type W = crate::W<RegMbidcam1Entry01Spec>;
#[doc = "Field `MBIDCAM1_ENTRY01_BSSID_MACID1` reader - "]
pub type Mbidcam1Entry01BssidMacid1R = crate::FieldReader<u16>;
#[doc = "Field `MBIDCAM1_ENTRY01_BSSID_MACID1` writer - "]
pub type Mbidcam1Entry01BssidMacid1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBIDCAM1_ENTRY01_DUA_CTS_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry01DuaCtsEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY01_DUA_CTS_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry01DuaCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY01_LSIG_TXOP_EN` reader - only use in mbidcam"]
pub type Mbidcam1Entry01LsigTxopEnR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY01_LSIG_TXOP_EN` writer - only use in mbidcam"]
pub type Mbidcam1Entry01LsigTxopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBIDCAM1_ENTRY01_VALID` reader - "]
pub type Mbidcam1Entry01ValidR = crate::BitReader;
#[doc = "Field `MBIDCAM1_ENTRY01_VALID` writer - "]
pub type Mbidcam1Entry01ValidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry01_bssid_macid1(&self) -> Mbidcam1Entry01BssidMacid1R {
        Mbidcam1Entry01BssidMacid1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry01_dua_cts_en(&self) -> Mbidcam1Entry01DuaCtsEnR {
        Mbidcam1Entry01DuaCtsEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry01_lsig_txop_en(&self) -> Mbidcam1Entry01LsigTxopEnR {
        Mbidcam1Entry01LsigTxopEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry01_valid(&self) -> Mbidcam1Entry01ValidR {
        Mbidcam1Entry01ValidR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn mbidcam1_entry01_bssid_macid1(
        &mut self,
    ) -> Mbidcam1Entry01BssidMacid1W<'_, RegMbidcam1Entry01Spec> {
        Mbidcam1Entry01BssidMacid1W::new(self, 0)
    }
    #[doc = "Bit 16 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry01_dua_cts_en(
        &mut self,
    ) -> Mbidcam1Entry01DuaCtsEnW<'_, RegMbidcam1Entry01Spec> {
        Mbidcam1Entry01DuaCtsEnW::new(self, 16)
    }
    #[doc = "Bit 17 - only use in mbidcam"]
    #[inline(always)]
    pub fn mbidcam1_entry01_lsig_txop_en(
        &mut self,
    ) -> Mbidcam1Entry01LsigTxopEnW<'_, RegMbidcam1Entry01Spec> {
        Mbidcam1Entry01LsigTxopEnW::new(self, 17)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn mbidcam1_entry01_valid(&mut self) -> Mbidcam1Entry01ValidW<'_, RegMbidcam1Entry01Spec> {
        Mbidcam1Entry01ValidW::new(self, 23)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam1_entry01::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcam1_entry01::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcam1Entry01Spec;
impl crate::RegisterSpec for RegMbidcam1Entry01Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam1_entry01::R`](R) reader structure"]
impl crate::Readable for RegMbidcam1Entry01Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcam1_entry01::W`](W) writer structure"]
impl crate::Writable for RegMbidcam1Entry01Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAM1_ENTRY01 to value 0"]
impl crate::Resettable for RegMbidcam1Entry01Spec {}
