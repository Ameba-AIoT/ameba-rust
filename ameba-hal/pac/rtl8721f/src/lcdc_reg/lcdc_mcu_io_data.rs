#[doc = "Register `LCDC_MCU_IO_DATA` reader"]
pub type R = crate::R<LcdcMcuIoDataSpec>;
#[doc = "Field `mcu_rw_data` reader - When reading this register, this lower 24 bits represent the data read from LCM through MCU I/O mode I/F. Note: When TX FIFO not full and I/O timeout interrupt not happen ,this data can be read."]
pub type McuRwDataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:23 - When reading this register, this lower 24 bits represent the data read from LCM through MCU I/O mode I/F. Note: When TX FIFO not full and I/O timeout interrupt not happen ,this data can be read."]
    #[inline(always)]
    pub fn mcu_rw_data(&self) -> McuRwDataR {
        McuRwDataR::new(self.bits & 0x00ff_ffff)
    }
}
#[doc = "MCU IO mode config\n\nYou can [`read`](crate::Reg::read) this register and get [`lcdc_mcu_io_data::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcdcMcuIoDataSpec;
impl crate::RegisterSpec for LcdcMcuIoDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcdc_mcu_io_data::R`](R) reader structure"]
impl crate::Readable for LcdcMcuIoDataSpec {}
#[doc = "`reset()` method sets LCDC_MCU_IO_DATA to value 0"]
impl crate::Resettable for LcdcMcuIoDataSpec {}
