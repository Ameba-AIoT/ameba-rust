#[doc = "Register `REG_SEARCH_MACID` reader"]
pub type R = crate::R<RegSearchMacidSpec>;
#[doc = "Register `REG_SEARCH_MACID` writer"]
pub type W = crate::W<RegSearchMacidSpec>;
#[doc = "Field `BIT_INFO_ADDR_OFFSET` reader - per-macid information address offset (Unit:8bytes)."]
pub type BitInfoAddrOffsetR = crate::FieldReader<u16>;
#[doc = "Field `BIT_INFO_ADDR_OFFSET` writer - per-macid information address offset (Unit:8bytes)."]
pub type BitInfoAddrOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `BIT_DISABLE_B0` reader - when set to 1,disable to check a2\\[0\\]"]
pub type BitDisableB0R = crate::BitReader;
#[doc = "Field `BIT_DISABLE_B0` writer - when set to 1,disable to check a2\\[0\\]"]
pub type BitDisableB0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DIS_INFOSRCH` reader - when set to 1,disable to search information"]
pub type BitDisInfosrchR = crate::BitReader;
#[doc = "Field `BIT_DIS_INFOSRCH` writer - when set to 1,disable to search information"]
pub type BitDisInfosrchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_SRCH_FIFOFULL` reader - when set to 1, when fifofull, stop macid search;"]
pub type BitWmacSrchFifofullR = crate::BitReader;
#[doc = "Field `BIT_WMAC_SRCH_FIFOFULL` writer - when set to 1, when fifofull, stop macid search;"]
pub type BitWmacSrchFifofullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_INFO_INDEX_OFFSET` reader - Pre-macid information index address offset."]
pub type BitInfoIndexOffsetR = crate::FieldReader<u16>;
#[doc = "Field `BIT_INFO_INDEX_OFFSET` writer - Pre-macid information index address offset."]
pub type BitInfoIndexOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `BIT_EN_TXRPTBUF_CLK` reader - when set to 1,keep txrptbuf sram always have clock"]
pub type BitEnTxrptbufClkR = crate::BitReader;
#[doc = "Field `BIT_EN_TXRPTBUF_CLK` writer - when set to 1,keep txrptbuf sram always have clock"]
pub type BitEnTxrptbufClkW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:12 - per-macid information address offset (Unit:8bytes)."]
    #[inline(always)]
    pub fn bit_info_addr_offset(&self) -> BitInfoAddrOffsetR {
        BitInfoAddrOffsetR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - when set to 1,disable to check a2\\[0\\]"]
    #[inline(always)]
    pub fn bit_disable_b0(&self) -> BitDisableB0R {
        BitDisableB0R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - when set to 1,disable to search information"]
    #[inline(always)]
    pub fn bit_dis_infosrch(&self) -> BitDisInfosrchR {
        BitDisInfosrchR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - when set to 1, when fifofull, stop macid search;"]
    #[inline(always)]
    pub fn bit_wmac_srch_fifofull(&self) -> BitWmacSrchFifofullR {
        BitWmacSrchFifofullR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:28 - Pre-macid information index address offset."]
    #[inline(always)]
    pub fn bit_info_index_offset(&self) -> BitInfoIndexOffsetR {
        BitInfoIndexOffsetR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bit 31 - when set to 1,keep txrptbuf sram always have clock"]
    #[inline(always)]
    pub fn bit_en_txrptbuf_clk(&self) -> BitEnTxrptbufClkR {
        BitEnTxrptbufClkR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:12 - per-macid information address offset (Unit:8bytes)."]
    #[inline(always)]
    pub fn bit_info_addr_offset(&mut self) -> BitInfoAddrOffsetW<'_, RegSearchMacidSpec> {
        BitInfoAddrOffsetW::new(self, 0)
    }
    #[doc = "Bit 13 - when set to 1,disable to check a2\\[0\\]"]
    #[inline(always)]
    pub fn bit_disable_b0(&mut self) -> BitDisableB0W<'_, RegSearchMacidSpec> {
        BitDisableB0W::new(self, 13)
    }
    #[doc = "Bit 14 - when set to 1,disable to search information"]
    #[inline(always)]
    pub fn bit_dis_infosrch(&mut self) -> BitDisInfosrchW<'_, RegSearchMacidSpec> {
        BitDisInfosrchW::new(self, 14)
    }
    #[doc = "Bit 15 - when set to 1, when fifofull, stop macid search;"]
    #[inline(always)]
    pub fn bit_wmac_srch_fifofull(&mut self) -> BitWmacSrchFifofullW<'_, RegSearchMacidSpec> {
        BitWmacSrchFifofullW::new(self, 15)
    }
    #[doc = "Bits 16:28 - Pre-macid information index address offset."]
    #[inline(always)]
    pub fn bit_info_index_offset(&mut self) -> BitInfoIndexOffsetW<'_, RegSearchMacidSpec> {
        BitInfoIndexOffsetW::new(self, 16)
    }
    #[doc = "Bit 31 - when set to 1,keep txrptbuf sram always have clock"]
    #[inline(always)]
    pub fn bit_en_txrptbuf_clk(&mut self) -> BitEnTxrptbufClkW<'_, RegSearchMacidSpec> {
        BitEnTxrptbufClkW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_search_macid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_search_macid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSearchMacidSpec;
impl crate::RegisterSpec for RegSearchMacidSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_search_macid::R`](R) reader structure"]
impl crate::Readable for RegSearchMacidSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_search_macid::W`](W) writer structure"]
impl crate::Writable for RegSearchMacidSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SEARCH_MACID to value 0x0150_0000"]
impl crate::Resettable for RegSearchMacidSpec {
    const RESET_VALUE: u32 = 0x0150_0000;
}
