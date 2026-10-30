#[doc = "Register `RAW_CODE_FIFO_READ` reader"]
pub type R = crate::R<RawCodeFifoReadSpec>;
#[doc = "Field `afifo_rd_data` reader - Read data from raw code FIFO \\[11:0\\] : data"]
pub type AfifoRdDataR = crate::FieldReader<u16>;
#[doc = "Field `afifo_rd_ch_num` reader - Read data from raw code FIFO \\[15:12\\] : channel number"]
pub type AfifoRdChNumR = crate::FieldReader;
#[doc = "Field `afifo_rd_data_vld` reader - Read data from raw code FIFO valid"]
pub type AfifoRdDataVldR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - Read data from raw code FIFO \\[11:0\\] : data"]
    #[inline(always)]
    pub fn afifo_rd_data(&self) -> AfifoRdDataR {
        AfifoRdDataR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:15 - Read data from raw code FIFO \\[15:12\\] : channel number"]
    #[inline(always)]
    pub fn afifo_rd_ch_num(&self) -> AfifoRdChNumR {
        AfifoRdChNumR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Read data from raw code FIFO valid"]
    #[inline(always)]
    pub fn afifo_rd_data_vld(&self) -> AfifoRdDataVldR {
        AfifoRdDataVldR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FIFO Read Register\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_code_fifo_read::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawCodeFifoReadSpec;
impl crate::RegisterSpec for RawCodeFifoReadSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_code_fifo_read::R`](R) reader structure"]
impl crate::Readable for RawCodeFifoReadSpec {}
#[doc = "`reset()` method sets RAW_CODE_FIFO_READ to value 0"]
impl crate::Resettable for RawCodeFifoReadSpec {}
