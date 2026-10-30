#[doc = "Register `CTL6_H` reader"]
pub type R = crate::R<Ctl6HSpec>;
#[doc = "Register `CTL6_H` writer"]
pub type W = crate::W<Ctl6HSpec>;
#[doc = "Field `CTL6_H_TRANS_DATA_CNT` reader - Data Transfered Count.Once the transfer starts, the read-back value is the total number of data bytes already write to destination memory or peripheral."]
pub type Ctl6HTransDataCntR = crate::FieldReader<u32>;
#[doc = "Field `CTL6_H_BLOCK_TS` writer - Block Transfer Size.When the RTK_DMAC is the flow controller, the user writes this field before the channel is enabled in order to indicate the block size. The number programmed into BLOCK_TS indicates the total number of single transactions to perform for every block transfer; a single transaction is mapped to a single AXI beat. This field is write-only, number of block size programmed into CTLx_H.BLOCK_TS can be read from SGRx_H.BLOCK_TS_L and DSRx_H.BLOCK_TS_H. Width: The width of the single transaction is determined by CTLX_L.SRC_TR_WIDTH.When the source or destination peripheral is assigned as the flow controller, then the maximum block size that can be read back saturates at DMAC_CHx_MAX_BLK_SIZE, but the actual block size can be greater. b = log2(DMAC_CHx_MAX_BLK_SIZE + 1) -1"]
pub type Ctl6HBlockTsW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Transfered Count.Once the transfer starts, the read-back value is the total number of data bytes already write to destination memory or peripheral."]
    #[inline(always)]
    pub fn ctl6_h_trans_data_cnt(&self) -> Ctl6HTransDataCntR {
        Ctl6HTransDataCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:20 - Block Transfer Size.When the RTK_DMAC is the flow controller, the user writes this field before the channel is enabled in order to indicate the block size. The number programmed into BLOCK_TS indicates the total number of single transactions to perform for every block transfer; a single transaction is mapped to a single AXI beat. This field is write-only, number of block size programmed into CTLx_H.BLOCK_TS can be read from SGRx_H.BLOCK_TS_L and DSRx_H.BLOCK_TS_H. Width: The width of the single transaction is determined by CTLX_L.SRC_TR_WIDTH.When the source or destination peripheral is assigned as the flow controller, then the maximum block size that can be read back saturates at DMAC_CHx_MAX_BLK_SIZE, but the actual block size can be greater. b = log2(DMAC_CHx_MAX_BLK_SIZE + 1) -1"]
    #[inline(always)]
    pub fn ctl6_h_block_ts(&mut self) -> Ctl6HBlockTsW<'_, Ctl6HSpec> {
        Ctl6HBlockTsW::new(self, 0)
    }
}
#[doc = "Channel 6 high-32 bits Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl6_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl6_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl6HSpec;
impl crate::RegisterSpec for Ctl6HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl6_h::R`](R) reader structure"]
impl crate::Readable for Ctl6HSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl6_h::W`](W) writer structure"]
impl crate::Writable for Ctl6HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL6_H to value 0x02"]
impl crate::Resettable for Ctl6HSpec {
    const RESET_VALUE: u32 = 0x02;
}
