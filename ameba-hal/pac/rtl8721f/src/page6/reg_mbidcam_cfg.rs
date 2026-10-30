#[doc = "Register `REG_MBIDCAM_CFG` reader"]
pub type R = crate::R<RegMbidcamCfgSpec>;
#[doc = "Register `REG_MBIDCAM_CFG` writer"]
pub type W = crate::W<RegMbidcamCfgSpec>;
#[doc = "Field `BIT_MBIDCAM_RWDATA_H` reader - BSSID\\[47:32\\]"]
pub type BitMbidcamRwdataHR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MBIDCAM_RWDATA_H` writer - BSSID\\[47:32\\]"]
pub type BitMbidcamRwdataHW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_CTS_EN` reader - Dual CTS/CFE enable"]
pub type BitCtsEnR = crate::BitReader;
#[doc = "Field `BIT_CTS_EN` writer - Dual CTS/CFE enable"]
pub type BitCtsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_LSIC_TXOP_EN` reader - LSIG TXOP enable"]
pub type BitLsicTxopEnR = crate::BitReader;
#[doc = "Field `BIT_LSIC_TXOP_EN` writer - LSIG TXOP enable"]
pub type BitLsicTxopEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBIDCAM_VALID` reader - CAM Valid bit."]
pub type BitMbidcamValidR = crate::BitReader;
#[doc = "Field `BIT_MBIDCAM_VALID` writer - CAM Valid bit."]
pub type BitMbidcamValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBIDCAM_ADDR_V1` reader - CAM Address"]
pub type BitMbidcamAddrV1R = crate::FieldReader;
#[doc = "Field `BIT_MBIDCAM_ADDR_V1` writer - CAM Address"]
pub type BitMbidcamAddrV1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BIT_MBIDCAM_DA_EN` reader - CAM Direct Acess Enable"]
pub type BitMbidcamDaEnR = crate::BitReader;
#[doc = "Field `BIT_MBIDCAM_DA_EN` writer - CAM Direct Acess Enable"]
pub type BitMbidcamDaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBIDCAM_WT_EN` reader - Write Enable."]
pub type BitMbidcamWtEnR = crate::BitReader;
#[doc = "Field `BIT_MBIDCAM_WT_EN` writer - Write Enable."]
pub type BitMbidcamWtEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_MBIDCAM_POLL` writer - Pooling bit."]
pub type BitMbidcamPollW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - BSSID\\[47:32\\]"]
    #[inline(always)]
    pub fn bit_mbidcam_rwdata_h(&self) -> BitMbidcamRwdataHR {
        BitMbidcamRwdataHR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Dual CTS/CFE enable"]
    #[inline(always)]
    pub fn bit_cts_en(&self) -> BitCtsEnR {
        BitCtsEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LSIG TXOP enable"]
    #[inline(always)]
    pub fn bit_lsic_txop_en(&self) -> BitLsicTxopEnR {
        BitLsicTxopEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 23 - CAM Valid bit."]
    #[inline(always)]
    pub fn bit_mbidcam_valid(&self) -> BitMbidcamValidR {
        BitMbidcamValidR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:28 - CAM Address"]
    #[inline(always)]
    pub fn bit_mbidcam_addr_v1(&self) -> BitMbidcamAddrV1R {
        BitMbidcamAddrV1R::new(((self.bits >> 24) & 0x1f) as u8)
    }
    #[doc = "Bit 29 - CAM Direct Acess Enable"]
    #[inline(always)]
    pub fn bit_mbidcam_da_en(&self) -> BitMbidcamDaEnR {
        BitMbidcamDaEnR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Write Enable."]
    #[inline(always)]
    pub fn bit_mbidcam_wt_en(&self) -> BitMbidcamWtEnR {
        BitMbidcamWtEnR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - BSSID\\[47:32\\]"]
    #[inline(always)]
    pub fn bit_mbidcam_rwdata_h(&mut self) -> BitMbidcamRwdataHW<'_, RegMbidcamCfgSpec> {
        BitMbidcamRwdataHW::new(self, 0)
    }
    #[doc = "Bit 16 - Dual CTS/CFE enable"]
    #[inline(always)]
    pub fn bit_cts_en(&mut self) -> BitCtsEnW<'_, RegMbidcamCfgSpec> {
        BitCtsEnW::new(self, 16)
    }
    #[doc = "Bit 17 - LSIG TXOP enable"]
    #[inline(always)]
    pub fn bit_lsic_txop_en(&mut self) -> BitLsicTxopEnW<'_, RegMbidcamCfgSpec> {
        BitLsicTxopEnW::new(self, 17)
    }
    #[doc = "Bit 23 - CAM Valid bit."]
    #[inline(always)]
    pub fn bit_mbidcam_valid(&mut self) -> BitMbidcamValidW<'_, RegMbidcamCfgSpec> {
        BitMbidcamValidW::new(self, 23)
    }
    #[doc = "Bits 24:28 - CAM Address"]
    #[inline(always)]
    pub fn bit_mbidcam_addr_v1(&mut self) -> BitMbidcamAddrV1W<'_, RegMbidcamCfgSpec> {
        BitMbidcamAddrV1W::new(self, 24)
    }
    #[doc = "Bit 29 - CAM Direct Acess Enable"]
    #[inline(always)]
    pub fn bit_mbidcam_da_en(&mut self) -> BitMbidcamDaEnW<'_, RegMbidcamCfgSpec> {
        BitMbidcamDaEnW::new(self, 29)
    }
    #[doc = "Bit 30 - Write Enable."]
    #[inline(always)]
    pub fn bit_mbidcam_wt_en(&mut self) -> BitMbidcamWtEnW<'_, RegMbidcamCfgSpec> {
        BitMbidcamWtEnW::new(self, 30)
    }
    #[doc = "Bit 31 - Pooling bit."]
    #[inline(always)]
    pub fn bit_mbidcam_poll(&mut self) -> BitMbidcamPollW<'_, RegMbidcamCfgSpec> {
        BitMbidcamPollW::new(self, 31)
    }
}
#[doc = "MBSSID CAM Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mbidcam_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mbidcam_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMbidcamCfgSpec;
impl crate::RegisterSpec for RegMbidcamCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mbidcam_cfg::R`](R) reader structure"]
impl crate::Readable for RegMbidcamCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mbidcam_cfg::W`](W) writer structure"]
impl crate::Writable for RegMbidcamCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MBIDCAM_CFG to value 0x2000_0000"]
impl crate::Resettable for RegMbidcamCfgSpec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
