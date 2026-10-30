#[doc = "Register `DM_DR13` writer"]
pub type W = crate::W<DmDr13Spec>;
#[doc = "Field `DATA_MASK_DR13` writer - It is a data buffer for spi_dm signal in user mode. The output value of spi_dm is the data that has been pushed to DM_FIFO \\[0\\]. If DM_FIFO is empty, the output value of spi_data_en will be (~DM_ACT). Available only if configuration of DM_EN are defined."]
pub type DataMaskDr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_MASK_DR_EXTEND13` writer - It is a extended data buffer for spi_dm\\[1\\] signal in user mode when the width of spi_dm is 2-bit (DQ16). The output value of spi_dm\\[1\\] is the data that has been pushed to DM_FIFO \\[1\\]. If DM_FIFO is empty, the output value of spi_data_en will be (~DM_ACT). Available only if configuration of DM_EN and DQ16 are defined."]
pub type DataMaskDrExtend13W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - It is a data buffer for spi_dm signal in user mode. The output value of spi_dm is the data that has been pushed to DM_FIFO \\[0\\]. If DM_FIFO is empty, the output value of spi_data_en will be (~DM_ACT). Available only if configuration of DM_EN are defined."]
    #[inline(always)]
    pub fn data_mask_dr13(&mut self) -> DataMaskDr13W<'_, DmDr13Spec> {
        DataMaskDr13W::new(self, 0)
    }
    #[doc = "Bit 1 - It is a extended data buffer for spi_dm\\[1\\] signal in user mode when the width of spi_dm is 2-bit (DQ16). The output value of spi_dm\\[1\\] is the data that has been pushed to DM_FIFO \\[1\\]. If DM_FIFO is empty, the output value of spi_data_en will be (~DM_ACT). Available only if configuration of DM_EN and DQ16 are defined."]
    #[inline(always)]
    pub fn data_mask_dr_extend13(&mut self) -> DataMaskDrExtend13W<'_, DmDr13Spec> {
        DataMaskDrExtend13W::new(self, 1)
    }
}
#[doc = "Data Mask Data Register 13\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dm_dr13::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmDr13Spec;
impl crate::RegisterSpec for DmDr13Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dm_dr13::W`](W) writer structure"]
impl crate::Writable for DmDr13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DM_DR13 to value 0"]
impl crate::Resettable for DmDr13Spec {}
