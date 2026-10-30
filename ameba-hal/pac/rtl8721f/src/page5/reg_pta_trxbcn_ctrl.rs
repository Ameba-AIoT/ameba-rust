#[doc = "Register `REG_PTA_TRXBCN_CTRL` reader"]
pub type R = crate::R<RegPtaTrxbcnCtrlSpec>;
#[doc = "Register `REG_PTA_TRXBCN_CTRL` writer"]
pub type W = crate::W<RegPtaTrxbcnCtrlSpec>;
#[doc = "Field `BIT_PTA_PRETX_TIME` reader - WIFI TX shall wait settle time when antenna is switched from BT to WIFI, if the BT use the same RFAFE as WIFI. Unit: 1us"]
pub type BitPtaPretxTimeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_PTA_PRETX_TIME` writer - WIFI TX shall wait settle time when antenna is switched from BT to WIFI, if the BT use the same RFAFE as WIFI. Unit: 1us"]
pub type BitPtaPretxTimeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_PTA_PRETX_EN` reader - "]
pub type BitPtaPretxEnR = crate::BitReader;
#[doc = "Field `BIT_PTA_PRETX_EN` writer - "]
pub type BitPtaPretxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PTA_RXBCN_TIME` reader - "]
pub type BitPtaRxbcnTimeR = crate::FieldReader;
#[doc = "Field `BIT_PTA_RXBCN_TIME` writer - "]
pub type BitPtaRxbcnTimeW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_PTA_RXBCN_DUR` reader - "]
pub type BitPtaRxbcnDurR = crate::FieldReader<u16>;
#[doc = "Field `BIT_PTA_RXBCN_DUR` writer - "]
pub type BitPtaRxbcnDurW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `BIT_PTA_RXBCN_DIR` reader - "]
pub type BitPtaRxbcnDirR = crate::BitReader;
#[doc = "Field `BIT_PTA_RXBCN_DIR` writer - "]
pub type BitPtaRxbcnDirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PTA_RXBCN_EN` reader - "]
pub type BitPtaRxbcnEnR = crate::BitReader;
#[doc = "Field `BIT_PTA_RXBCN_EN` writer - "]
pub type BitPtaRxbcnEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_PTA_RXBCN_PRI_MASK` reader - "]
pub type BitPtaRxbcnPriMaskR = crate::BitReader;
#[doc = "Field `BIT_PTA_RXBCN_PRI_MASK` writer - "]
pub type BitPtaRxbcnPriMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - WIFI TX shall wait settle time when antenna is switched from BT to WIFI, if the BT use the same RFAFE as WIFI. Unit: 1us"]
    #[inline(always)]
    pub fn bit_pta_pretx_time(&self) -> BitPtaPretxTimeR {
        BitPtaPretxTimeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bit_pta_pretx_en(&self) -> BitPtaPretxEnR {
        BitPtaPretxEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:19"]
    #[inline(always)]
    pub fn bit_pta_rxbcn_time(&self) -> BitPtaRxbcnTimeR {
        BitPtaRxbcnTimeR::new(((self.bits >> 13) & 0x7f) as u8)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn bit_pta_rxbcn_dur(&self) -> BitPtaRxbcnDurR {
        BitPtaRxbcnDurR::new(((self.bits >> 20) & 0x01ff) as u16)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn bit_pta_rxbcn_dir(&self) -> BitPtaRxbcnDirR {
        BitPtaRxbcnDirR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn bit_pta_rxbcn_en(&self) -> BitPtaRxbcnEnR {
        BitPtaRxbcnEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn bit_pta_rxbcn_pri_mask(&self) -> BitPtaRxbcnPriMaskR {
        BitPtaRxbcnPriMaskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - WIFI TX shall wait settle time when antenna is switched from BT to WIFI, if the BT use the same RFAFE as WIFI. Unit: 1us"]
    #[inline(always)]
    pub fn bit_pta_pretx_time(&mut self) -> BitPtaPretxTimeW<'_, RegPtaTrxbcnCtrlSpec> {
        BitPtaPretxTimeW::new(self, 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn bit_pta_pretx_en(&mut self) -> BitPtaPretxEnW<'_, RegPtaTrxbcnCtrlSpec> {
        BitPtaPretxEnW::new(self, 12)
    }
    #[doc = "Bits 13:19"]
    #[inline(always)]
    pub fn bit_pta_rxbcn_time(&mut self) -> BitPtaRxbcnTimeW<'_, RegPtaTrxbcnCtrlSpec> {
        BitPtaRxbcnTimeW::new(self, 13)
    }
    #[doc = "Bits 20:28"]
    #[inline(always)]
    pub fn bit_pta_rxbcn_dur(&mut self) -> BitPtaRxbcnDurW<'_, RegPtaTrxbcnCtrlSpec> {
        BitPtaRxbcnDurW::new(self, 20)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn bit_pta_rxbcn_dir(&mut self) -> BitPtaRxbcnDirW<'_, RegPtaTrxbcnCtrlSpec> {
        BitPtaRxbcnDirW::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn bit_pta_rxbcn_en(&mut self) -> BitPtaRxbcnEnW<'_, RegPtaTrxbcnCtrlSpec> {
        BitPtaRxbcnEnW::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn bit_pta_rxbcn_pri_mask(&mut self) -> BitPtaRxbcnPriMaskW<'_, RegPtaTrxbcnCtrlSpec> {
        BitPtaRxbcnPriMaskW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pta_trxbcn_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pta_trxbcn_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPtaTrxbcnCtrlSpec;
impl crate::RegisterSpec for RegPtaTrxbcnCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pta_trxbcn_ctrl::R`](R) reader structure"]
impl crate::Readable for RegPtaTrxbcnCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pta_trxbcn_ctrl::W`](W) writer structure"]
impl crate::Writable for RegPtaTrxbcnCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PTA_TRXBCN_CTRL to value 0xe600_d064"]
impl crate::Resettable for RegPtaTrxbcnCtrlSpec {
    const RESET_VALUE: u32 = 0xe600_d064;
}
