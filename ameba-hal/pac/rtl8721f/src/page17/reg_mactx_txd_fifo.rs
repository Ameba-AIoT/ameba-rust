#[doc = "Register `REG_MACTX_TXD_FIFO` reader"]
pub type R = crate::R<RegMactxTxdFifoSpec>;
#[doc = "Field `sw_txdfifo_dout` reader - sw read rxd_out"]
pub type SwTxdfifoDoutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - sw read rxd_out"]
    #[inline(always)]
    pub fn sw_txdfifo_dout(&self) -> SwTxdfifoDoutR {
        SwTxdfifoDoutR::new(self.bits)
    }
}
#[doc = "fifo do\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mactx_txd_fifo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMactxTxdFifoSpec;
impl crate::RegisterSpec for RegMactxTxdFifoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mactx_txd_fifo::R`](R) reader structure"]
impl crate::Readable for RegMactxTxdFifoSpec {}
#[doc = "`reset()` method sets REG_MACTX_TXD_FIFO to value 0"]
impl crate::Resettable for RegMactxTxdFifoSpec {}
