#[doc = "Register `REG_ANAPAR_RFC` reader"]
pub type R = crate::R<RegAnaparRfcSpec>;
#[doc = "Register `REG_ANAPAR_RFC` writer"]
pub type W = crate::W<RegAnaparRfcSpec>;
#[doc = "Field `ANAPOW_RFC_NM` reader - wlan power rf control"]
pub type AnapowRfcNmR = crate::FieldReader;
#[doc = "Field `ANAPOW_RFC_NM` writer - wlan power rf control"]
pub type AnapowRfcNmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `r_rfafe_VIO1833_sel` reader - IND_VIO1833 output decided by: 1: 1.8V 0: 3.3V"]
pub type RRfafeVio1833SelR = crate::BitReader;
#[doc = "Field `r_rfafe_VIO1833_sel` writer - IND_VIO1833 output decided by: 1: 1.8V 0: 3.3V"]
pub type RRfafeVio1833SelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANAPOW_RFC_LPS` reader - wlan power rf control"]
pub type AnapowRfcLpsR = crate::FieldReader;
#[doc = "Field `ANAPOW_RFC_LPS` writer - wlan power rf control"]
pub type AnapowRfcLpsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WL_FUNEN_WLRF_SEL` reader - 1: select ANAPOW_RFC_NM as ANAPAR_RFC\\[7:0\\],used when RFAFE is active 0: select ANAPOW_RFC_LPS as ANAPAR_RFC\\[7:0\\],,used when RFAFE is power off"]
pub type WlFunenWlrfSelR = crate::BitReader;
#[doc = "Field `WL_FUNEN_WLRF_SEL` writer - 1: select ANAPOW_RFC_NM as ANAPAR_RFC\\[7:0\\],used when RFAFE is active 0: select ANAPOW_RFC_LPS as ANAPAR_RFC\\[7:0\\],,used when RFAFE is power off"]
pub type WlFunenWlrfSelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - wlan power rf control"]
    #[inline(always)]
    pub fn anapow_rfc_nm(&self) -> AnapowRfcNmR {
        AnapowRfcNmR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 10 - IND_VIO1833 output decided by: 1: 1.8V 0: 3.3V"]
    #[inline(always)]
    pub fn r_rfafe_vio1833_sel(&self) -> RRfafeVio1833SelR {
        RRfafeVio1833SelR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:18 - wlan power rf control"]
    #[inline(always)]
    pub fn anapow_rfc_lps(&self) -> AnapowRfcLpsR {
        AnapowRfcLpsR::new(((self.bits >> 11) & 0xff) as u8)
    }
    #[doc = "Bit 19 - 1: select ANAPOW_RFC_NM as ANAPAR_RFC\\[7:0\\],used when RFAFE is active 0: select ANAPOW_RFC_LPS as ANAPAR_RFC\\[7:0\\],,used when RFAFE is power off"]
    #[inline(always)]
    pub fn wl_funen_wlrf_sel(&self) -> WlFunenWlrfSelR {
        WlFunenWlrfSelR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - wlan power rf control"]
    #[inline(always)]
    pub fn anapow_rfc_nm(&mut self) -> AnapowRfcNmW<'_, RegAnaparRfcSpec> {
        AnapowRfcNmW::new(self, 0)
    }
    #[doc = "Bit 10 - IND_VIO1833 output decided by: 1: 1.8V 0: 3.3V"]
    #[inline(always)]
    pub fn r_rfafe_vio1833_sel(&mut self) -> RRfafeVio1833SelW<'_, RegAnaparRfcSpec> {
        RRfafeVio1833SelW::new(self, 10)
    }
    #[doc = "Bits 11:18 - wlan power rf control"]
    #[inline(always)]
    pub fn anapow_rfc_lps(&mut self) -> AnapowRfcLpsW<'_, RegAnaparRfcSpec> {
        AnapowRfcLpsW::new(self, 11)
    }
    #[doc = "Bit 19 - 1: select ANAPOW_RFC_NM as ANAPAR_RFC\\[7:0\\],used when RFAFE is active 0: select ANAPOW_RFC_LPS as ANAPAR_RFC\\[7:0\\],,used when RFAFE is power off"]
    #[inline(always)]
    pub fn wl_funen_wlrf_sel(&mut self) -> WlFunenWlrfSelW<'_, RegAnaparRfcSpec> {
        WlFunenWlrfSelW::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_anapar_rfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_anapar_rfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegAnaparRfcSpec;
impl crate::RegisterSpec for RegAnaparRfcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_anapar_rfc::R`](R) reader structure"]
impl crate::Readable for RegAnaparRfcSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_anapar_rfc::W`](W) writer structure"]
impl crate::Writable for RegAnaparRfcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_ANAPAR_RFC to value 0x0002_08c7"]
impl crate::Resettable for RegAnaparRfcSpec {
    const RESET_VALUE: u32 = 0x0002_08c7;
}
