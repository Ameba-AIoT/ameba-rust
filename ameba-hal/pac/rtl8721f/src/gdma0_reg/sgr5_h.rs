#[doc = "Register `SGR5_H` reader"]
pub type R = crate::R<Sgr5HSpec>;
#[doc = "Field `SGR5_H_BLOCK_TS_L` reader - Low 16-bits of Block Transfer Size.The number programmed into CTLX.BLOCK_TS, which indicates the total number of single transactions to perform for every block transfer, is separated into low 16 bits and high part. The low 16 bits is stored into SGRx.BLOCK_TS_L while the high part is stored into DSRx.BLOCK_TS_H Width: The width of the single transaction is determined by CTLx.SRC_TR_WIDTH."]
pub type Sgr5HBlockTsLR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 16:31 - Low 16-bits of Block Transfer Size.The number programmed into CTLX.BLOCK_TS, which indicates the total number of single transactions to perform for every block transfer, is separated into low 16 bits and high part. The low 16 bits is stored into SGRx.BLOCK_TS_L while the high part is stored into DSRx.BLOCK_TS_H Width: The width of the single transaction is determined by CTLx.SRC_TR_WIDTH."]
    #[inline(always)]
    pub fn sgr5_h_block_ts_l(&self) -> Sgr5HBlockTsLR {
        Sgr5HBlockTsLR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Channel 5 high-32 bits Source Gather Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sgr5_h::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sgr5HSpec;
impl crate::RegisterSpec for Sgr5HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sgr5_h::R`](R) reader structure"]
impl crate::Readable for Sgr5HSpec {}
#[doc = "`reset()` method sets SGR5_H to value 0x0002_0000"]
impl crate::Resettable for Sgr5HSpec {
    const RESET_VALUE: u32 = 0x0002_0000;
}
