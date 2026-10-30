#[doc = "Register `REG_WMAC_CSIDMA_CFG` reader"]
pub type R = crate::R<RegWmacCsidmaCfgSpec>;
#[doc = "Register `REG_WMAC_CSIDMA_CFG` writer"]
pub type W = crate::W<RegWmacCsidmaCfgSpec>;
#[doc = "Field `BIT_CSI_START_PAGE` reader - The starting page number of csi buffer in TXPKTBUF"]
pub type BitCsiStartPageR = crate::FieldReader<u16>;
#[doc = "Field `BIT_CSI_START_PAGE` writer - The starting page number of csi buffer in TXPKTBUF"]
pub type BitCsiStartPageW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_CSI_SEG_SIZE` reader - CSI data segmentation size (bytes) = Max. MPDU length - mac hdr (24) - act type (1) - category (1) - mimo ctrl (3) - FCS (4)(default:3862byte)"]
pub type BitCsiSegSizeR = crate::FieldReader<u16>;
#[doc = "Field `BIT_CSI_SEG_SIZE` writer - CSI data segmentation size (bytes) = Max. MPDU length - mac hdr (24) - act type (1) - category (1) - mimo ctrl (3) - FCS (4)(default:3862byte)"]
pub type BitCsiSegSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - The starting page number of csi buffer in TXPKTBUF"]
    #[inline(always)]
    pub fn bit_csi_start_page(&self) -> BitCsiStartPageR {
        BitCsiStartPageR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - CSI data segmentation size (bytes) = Max. MPDU length - mac hdr (24) - act type (1) - category (1) - mimo ctrl (3) - FCS (4)(default:3862byte)"]
    #[inline(always)]
    pub fn bit_csi_seg_size(&self) -> BitCsiSegSizeR {
        BitCsiSegSizeR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The starting page number of csi buffer in TXPKTBUF"]
    #[inline(always)]
    pub fn bit_csi_start_page(&mut self) -> BitCsiStartPageW<'_, RegWmacCsidmaCfgSpec> {
        BitCsiStartPageW::new(self, 0)
    }
    #[doc = "Bits 16:27 - CSI data segmentation size (bytes) = Max. MPDU length - mac hdr (24) - act type (1) - category (1) - mimo ctrl (3) - FCS (4)(default:3862byte)"]
    #[inline(always)]
    pub fn bit_csi_seg_size(&mut self) -> BitCsiSegSizeW<'_, RegWmacCsidmaCfgSpec> {
        BitCsiSegSizeW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_csidma_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_csidma_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacCsidmaCfgSpec;
impl crate::RegisterSpec for RegWmacCsidmaCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_csidma_cfg::R`](R) reader structure"]
impl crate::Readable for RegWmacCsidmaCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_csidma_cfg::W`](W) writer structure"]
impl crate::Writable for RegWmacCsidmaCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_CSIDMA_CFG to value 0x0f16_00f8"]
impl crate::Resettable for RegWmacCsidmaCfgSpec {
    const RESET_VALUE: u32 = 0x0f16_00f8;
}
