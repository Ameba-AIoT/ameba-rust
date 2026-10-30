#[doc = "Register `DSR7_H` reader"]
pub type R = crate::R<Dsr7HSpec>;
#[doc = "Field `DSR7_H_BLOCK_TS_H` reader - High part of Block Transfer SizeThe number programmed into CTLx_H.BLOCK_TS, which indicates the total number of single transactions to perform for every block transfer, is separated into low 16 bits and high 16 bits. The low 16-bits is stored into SGRx_H.BLOCK_TS_LOW while the high 16-bits is stored into DSRx_H.BLOCK_TS_HIGH Width: The width of the single transaction is determined by CTLx_L.SRC_TR_WIDTH.b = log2 (DMAC_CHx_MAX_BLK_SIZE). Dependencies: The maximum value of DMAC_CHx_MAX_BLK_SIZE is 2 ^ (31-log2 (DMAC_DATA_WIDTH/8)) - 1."]
pub type Dsr7HBlockTsHR = crate::FieldReader;
impl R {
    #[doc = "Bits 16:20 - High part of Block Transfer SizeThe number programmed into CTLx_H.BLOCK_TS, which indicates the total number of single transactions to perform for every block transfer, is separated into low 16 bits and high 16 bits. The low 16-bits is stored into SGRx_H.BLOCK_TS_LOW while the high 16-bits is stored into DSRx_H.BLOCK_TS_HIGH Width: The width of the single transaction is determined by CTLx_L.SRC_TR_WIDTH.b = log2 (DMAC_CHx_MAX_BLK_SIZE). Dependencies: The maximum value of DMAC_CHx_MAX_BLK_SIZE is 2 ^ (31-log2 (DMAC_DATA_WIDTH/8)) - 1."]
    #[inline(always)]
    pub fn dsr7_h_block_ts_h(&self) -> Dsr7HBlockTsHR {
        Dsr7HBlockTsHR::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
#[doc = "Channel 7 high-32 bits Destination Scatter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr7_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsr7HSpec;
impl crate::RegisterSpec for Dsr7HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr7_h::R`](R) reader structure"]
impl crate::Readable for Dsr7HSpec {}
#[doc = "`reset()` method sets DSR7_H to value 0"]
impl crate::Resettable for Dsr7HSpec {}
