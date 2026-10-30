#[doc = "Register `REG_SEC_PPC_CTRL` reader"]
pub type R = crate::R<RegSecPpcCtrlSpec>;
#[doc = "Register `REG_SEC_PPC_CTRL` writer"]
pub type W = crate::W<RegSecPpcCtrlSpec>;
#[doc = "Field `ppc_aip` reader - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcAipR = crate::FieldReader;
#[doc = "Field `ppc_aip` writer - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcAipW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ppc_sdm` reader - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcSdmR = crate::FieldReader;
#[doc = "Field `ppc_sdm` writer - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcSdmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ppc_rtc` reader - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcRtcR = crate::FieldReader;
#[doc = "Field `ppc_rtc` writer - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcRtcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ppc_pad` reader - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both Note:this control both of pinmux and ledc, for they are in same reset domain."]
pub type PpcPadR = crate::FieldReader;
#[doc = "Field `ppc_pad` writer - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both Note:this control both of pinmux and ledc, for they are in same reset domain."]
pub type PpcPadW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ppc_mem` reader - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcMemR = crate::FieldReader;
#[doc = "Field `ppc_mem` writer - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcMemW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ppc_mmu` reader - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcMmuR = crate::FieldReader;
#[doc = "Field `ppc_mmu` writer - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcMmuW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ppc_wdg` reader - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcWdgR = crate::FieldReader;
#[doc = "Field `ppc_wdg` writer - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcWdgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ppc_aret` reader - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcAretR = crate::FieldReader;
#[doc = "Field `ppc_aret` writer - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
pub type PpcAretW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `pcr_prot_k0` reader - Retention Mem last 256 bit protect (located in last 256bit of retention memory) 0: read and write , 1 : read only"]
pub type PcrProtK0R = crate::BitReader;
#[doc = "Field `pcr_prot_k0` writer - Retention Mem last 256 bit protect (located in last 256bit of retention memory) 0: read and write , 1 : read only"]
pub type PcrProtK0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pcr_prot_k1` reader - Retention Mem last second 256 bit protect 0: read and write , 1 : read only"]
pub type PcrProtK1R = crate::BitReader;
#[doc = "Field `pcr_prot_k1` writer - Retention Mem last second 256 bit protect 0: read and write , 1 : read only"]
pub type PcrProtK1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pcr_prot_k2` reader - Retention Mem last third 256 bit protect 0: read and write , 1 : read only"]
pub type PcrProtK2R = crate::BitReader;
#[doc = "Field `pcr_prot_k2` writer - Retention Mem last third 256 bit protect 0: read and write , 1 : read only"]
pub type PcrProtK2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `pcr_prot_k3` reader - Retention Mem last forth 256 bit protect 0: read and write , 1 : read only"]
pub type PcrProtK3R = crate::BitReader;
#[doc = "Field `pcr_prot_k3` writer - Retention Mem last forth 256 bit protect 0: read and write , 1 : read only"]
pub type PcrProtK3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_pmc` reader - Select IP controller source : pmc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPmcR = crate::BitReader;
#[doc = "Field `bpc_pmc` writer - Select IP controller source : pmc 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_swrst` reader - Select IP controller source : rst 1:control by nonsec zone 0:control by secure zone"]
pub type BpcSwrstR = crate::BitReader;
#[doc = "Field `bpc_swrst` writer - Select IP controller source : rst 1:control by nonsec zone 0:control by secure zone"]
pub type BpcSwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_cpu0` reader - Select IP controller source : cpu0 1:control by nonsec zone 0:control by secure zone"]
pub type BpcCpu0R = crate::BitReader;
#[doc = "Field `bpc_cpu0` writer - Select IP controller source : cpu0 1:control by nonsec zone 0:control by secure zone"]
pub type BpcCpu0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_cpu1` reader - Select IP controller source : cpu1 1:control by nonsec zone 0:control by secure zone"]
pub type BpcCpu1R = crate::BitReader;
#[doc = "Field `bpc_cpu1` writer - Select IP controller source : cpu1 1:control by nonsec zone 0:control by secure zone"]
pub type BpcCpu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_spic` reader - Select IP controller source : spic 1:control by nonsec zone 0:control by secure zone"]
pub type BpcSpicR = crate::BitReader;
#[doc = "Field `bpc_spic` writer - Select IP controller source : spic 1:control by nonsec zone 0:control by secure zone"]
pub type BpcSpicW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_psram` reader - Select IP controller source : psram 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPsramR = crate::BitReader;
#[doc = "Field `bpc_psram` writer - Select IP controller source : psram 1:control by nonsec zone 0:control by secure zone"]
pub type BpcPsramW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_wl` reader - Select IP controller source : wl 1:control by nonsec zone 0:control by secure zone"]
pub type BpcWlR = crate::BitReader;
#[doc = "Field `bpc_wl` writer - Select IP controller source : wl 1:control by nonsec zone 0:control by secure zone"]
pub type BpcWlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `bpc_bt` reader - Select IP controller source : bt 1:control by nonsec zone 0:control by secure zone"]
pub type BpcBtR = crate::BitReader;
#[doc = "Field `bpc_bt` writer - Select IP controller source : bt 1:control by nonsec zone 0:control by secure zone"]
pub type BpcBtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_aip(&self) -> PpcAipR {
        PpcAipR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_sdm(&self) -> PpcSdmR {
        PpcSdmR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_rtc(&self) -> PpcRtcR {
        PpcRtcR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both Note:this control both of pinmux and ledc, for they are in same reset domain."]
    #[inline(always)]
    pub fn ppc_pad(&self) -> PpcPadR {
        PpcPadR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_mem(&self) -> PpcMemR {
        PpcMemR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_mmu(&self) -> PpcMmuR {
        PpcMmuR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_wdg(&self) -> PpcWdgR {
        PpcWdgR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_aret(&self) -> PpcAretR {
        PpcAretR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Retention Mem last 256 bit protect (located in last 256bit of retention memory) 0: read and write , 1 : read only"]
    #[inline(always)]
    pub fn pcr_prot_k0(&self) -> PcrProtK0R {
        PcrProtK0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Retention Mem last second 256 bit protect 0: read and write , 1 : read only"]
    #[inline(always)]
    pub fn pcr_prot_k1(&self) -> PcrProtK1R {
        PcrProtK1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Retention Mem last third 256 bit protect 0: read and write , 1 : read only"]
    #[inline(always)]
    pub fn pcr_prot_k2(&self) -> PcrProtK2R {
        PcrProtK2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Retention Mem last forth 256 bit protect 0: read and write , 1 : read only"]
    #[inline(always)]
    pub fn pcr_prot_k3(&self) -> PcrProtK3R {
        PcrProtK3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - Select IP controller source : pmc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_pmc(&self) -> BpcPmcR {
        BpcPmcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Select IP controller source : rst 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_swrst(&self) -> BpcSwrstR {
        BpcSwrstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Select IP controller source : cpu0 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_cpu0(&self) -> BpcCpu0R {
        BpcCpu0R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Select IP controller source : cpu1 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_cpu1(&self) -> BpcCpu1R {
        BpcCpu1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Select IP controller source : spic 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_spic(&self) -> BpcSpicR {
        BpcSpicR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Select IP controller source : psram 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_psram(&self) -> BpcPsramR {
        BpcPsramR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Select IP controller source : wl 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_wl(&self) -> BpcWlR {
        BpcWlR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Select IP controller source : bt 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_bt(&self) -> BpcBtR {
        BpcBtR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_aip(&mut self) -> PpcAipW<'_, RegSecPpcCtrlSpec> {
        PpcAipW::new(self, 0)
    }
    #[doc = "Bits 2:3 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_sdm(&mut self) -> PpcSdmW<'_, RegSecPpcCtrlSpec> {
        PpcSdmW::new(self, 2)
    }
    #[doc = "Bits 4:5 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_rtc(&mut self) -> PpcRtcW<'_, RegSecPpcCtrlSpec> {
        PpcRtcW::new(self, 4)
    }
    #[doc = "Bits 6:7 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both Note:this control both of pinmux and ledc, for they are in same reset domain."]
    #[inline(always)]
    pub fn ppc_pad(&mut self) -> PpcPadW<'_, RegSecPpcCtrlSpec> {
        PpcPadW::new(self, 6)
    }
    #[doc = "Bits 8:9 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_mem(&mut self) -> PpcMemW<'_, RegSecPpcCtrlSpec> {
        PpcMemW::new(self, 8)
    }
    #[doc = "Bits 10:11 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_mmu(&mut self) -> PpcMmuW<'_, RegSecPpcCtrlSpec> {
        PpcMmuW::new(self, 10)
    }
    #[doc = "Bits 12:13 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_wdg(&mut self) -> PpcWdgW<'_, RegSecPpcCtrlSpec> {
        PpcWdgW::new(self, 12)
    }
    #[doc = "Bits 14:15 - IP slave port R/W secure ctrl \\[0\\]: wr prot ctrl , 0: secure only , 1: both \\[1\\]: rd prot ctrl , 0:secure only , 1:both"]
    #[inline(always)]
    pub fn ppc_aret(&mut self) -> PpcAretW<'_, RegSecPpcCtrlSpec> {
        PpcAretW::new(self, 14)
    }
    #[doc = "Bit 16 - Retention Mem last 256 bit protect (located in last 256bit of retention memory) 0: read and write , 1 : read only"]
    #[inline(always)]
    pub fn pcr_prot_k0(&mut self) -> PcrProtK0W<'_, RegSecPpcCtrlSpec> {
        PcrProtK0W::new(self, 16)
    }
    #[doc = "Bit 17 - Retention Mem last second 256 bit protect 0: read and write , 1 : read only"]
    #[inline(always)]
    pub fn pcr_prot_k1(&mut self) -> PcrProtK1W<'_, RegSecPpcCtrlSpec> {
        PcrProtK1W::new(self, 17)
    }
    #[doc = "Bit 18 - Retention Mem last third 256 bit protect 0: read and write , 1 : read only"]
    #[inline(always)]
    pub fn pcr_prot_k2(&mut self) -> PcrProtK2W<'_, RegSecPpcCtrlSpec> {
        PcrProtK2W::new(self, 18)
    }
    #[doc = "Bit 19 - Retention Mem last forth 256 bit protect 0: read and write , 1 : read only"]
    #[inline(always)]
    pub fn pcr_prot_k3(&mut self) -> PcrProtK3W<'_, RegSecPpcCtrlSpec> {
        PcrProtK3W::new(self, 19)
    }
    #[doc = "Bit 24 - Select IP controller source : pmc 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_pmc(&mut self) -> BpcPmcW<'_, RegSecPpcCtrlSpec> {
        BpcPmcW::new(self, 24)
    }
    #[doc = "Bit 25 - Select IP controller source : rst 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_swrst(&mut self) -> BpcSwrstW<'_, RegSecPpcCtrlSpec> {
        BpcSwrstW::new(self, 25)
    }
    #[doc = "Bit 26 - Select IP controller source : cpu0 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_cpu0(&mut self) -> BpcCpu0W<'_, RegSecPpcCtrlSpec> {
        BpcCpu0W::new(self, 26)
    }
    #[doc = "Bit 27 - Select IP controller source : cpu1 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_cpu1(&mut self) -> BpcCpu1W<'_, RegSecPpcCtrlSpec> {
        BpcCpu1W::new(self, 27)
    }
    #[doc = "Bit 28 - Select IP controller source : spic 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_spic(&mut self) -> BpcSpicW<'_, RegSecPpcCtrlSpec> {
        BpcSpicW::new(self, 28)
    }
    #[doc = "Bit 29 - Select IP controller source : psram 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_psram(&mut self) -> BpcPsramW<'_, RegSecPpcCtrlSpec> {
        BpcPsramW::new(self, 29)
    }
    #[doc = "Bit 30 - Select IP controller source : wl 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_wl(&mut self) -> BpcWlW<'_, RegSecPpcCtrlSpec> {
        BpcWlW::new(self, 30)
    }
    #[doc = "Bit 31 - Select IP controller source : bt 1:control by nonsec zone 0:control by secure zone"]
    #[inline(always)]
    pub fn bpc_bt(&mut self) -> BpcBtW<'_, RegSecPpcCtrlSpec> {
        BpcBtW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sec_ppc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sec_ppc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSecPpcCtrlSpec;
impl crate::RegisterSpec for RegSecPpcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sec_ppc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegSecPpcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sec_ppc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegSecPpcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SEC_PPC_CTRL to value 0xff00_ffff"]
impl crate::Resettable for RegSecPpcCtrlSpec {
    const RESET_VALUE: u32 = 0xff00_ffff;
}
