#[doc = "Register `CTL2_H` reader"]
pub type R = crate::R<Ctl2HSpec>;
#[doc = "Register `CTL2_H` writer"]
pub type W = crate::W<Ctl2HSpec>;
#[doc = "Field `CTL2_H_TRANS_DATA_CNT` reader - Data Transfered Count.Once the transfer starts, the read-back value is the total number of data bytes already write to destination memory or peripheral."]
pub type Ctl2HTransDataCntR = crate::FieldReader<u32>;
#[doc = "Field `CTL2_H_BLOCK_TS` writer - Block Transfer Size.When the RTK_DMAC is the flow controller, the user writes this field before the channel is enabled in order to indicate the block size. The number programmed into BLOCK_TS indicates the total number of single transactions to perform for every block transfer; a single transaction is mapped to a single AXI beat. This field is write-only, number of block size programmed into CTLx_H.BLOCK_TS can be read from SGRx_H.BLOCK_TS_L and DSRx_H.BLOCK_TS_H. Width: The width of the single transaction is determined by CTLX_L.SRC_TR_WIDTH.When the source or destination peripheral is assigned as the flow controller, then the maximum block size that can be read back saturates at DMAC_CHx_MAX_BLK_SIZE, but the actual block size can be greater. b = log2(DMAC_CHx_MAX_BLK_SIZE + 1) -1"]
pub type Ctl2HBlockTsW<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:31 - Data Transfered Count.Once the transfer starts, the read-back value is the total number of data bytes already write to destination memory or peripheral."]
    #[inline(always)]
    pub fn ctl2_h_trans_data_cnt(&self) -> Ctl2HTransDataCntR {
        Ctl2HTransDataCntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:20 - Block Transfer Size.When the RTK_DMAC is the flow controller, the user writes this field before the channel is enabled in order to indicate the block size. The number programmed into BLOCK_TS indicates the total number of single transactions to perform for every block transfer; a single transaction is mapped to a single AXI beat. This field is write-only, number of block size programmed into CTLx_H.BLOCK_TS can be read from SGRx_H.BLOCK_TS_L and DSRx_H.BLOCK_TS_H. Width: The width of the single transaction is determined by CTLX_L.SRC_TR_WIDTH.When the source or destination peripheral is assigned as the flow controller, then the maximum block size that can be read back saturates at DMAC_CHx_MAX_BLK_SIZE, but the actual block size can be greater. b = log2(DMAC_CHx_MAX_BLK_SIZE + 1) -1"]
    #[inline(always)]
    pub fn ctl2_h_block_ts(&mut self) -> Ctl2HBlockTsW<'_, Ctl2HSpec> {
        Ctl2HBlockTsW::new(self, 0)
    }
}
#[doc = "Channel 2 high-32 bits Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctl2_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl2_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctl2HSpec;
impl crate::RegisterSpec for Ctl2HSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctl2_h::R`](R) reader structure"]
impl crate::Readable for Ctl2HSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl2_h::W`](W) writer structure"]
impl crate::Writable for Ctl2HSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTL2_H to value 0x02"]
impl crate::Resettable for Ctl2HSpec {
    const RESET_VALUE: u32 = 0x02;
}
