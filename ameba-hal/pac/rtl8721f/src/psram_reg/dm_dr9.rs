#[doc = "Register `DM_DR9` writer"]
pub type W = crate::W<DmDr9Spec>;
#[doc = "Field `DATA_MASK_DR9` writer - It is a data buffer for spi_dm signal in user mode. The output value of spi_dm is the data that has been pushed to DM_FIFO \\[0\\]. If DM_FIFO is empty, the output value of spi_data_en will be (~DM_ACT). Available only if configuration of DM_EN are defined."]
pub type DataMaskDr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_MASK_DR_EXTEND9` writer - It is a extended data buffer for spi_dm\\[1\\] signal in user mode when the width of spi_dm is 2-bit (DQ16). The output value of spi_dm\\[1\\] is the data that has been pushed to DM_FIFO \\[1\\]. If DM_FIFO is empty, the output value of spi_data_en will be (~DM_ACT). Available only if configuration of DM_EN and DQ16 are defined."]
pub type DataMaskDrExtend9W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - It is a data buffer for spi_dm signal in user mode. The output value of spi_dm is the data that has been pushed to DM_FIFO \\[0\\]. If DM_FIFO is empty, the output value of spi_data_en will be (~DM_ACT). Available only if configuration of DM_EN are defined."]
    #[inline(always)]
    pub fn data_mask_dr9(&mut self) -> DataMaskDr9W<'_, DmDr9Spec> {
        DataMaskDr9W::new(self, 0)
    }
    #[doc = "Bit 1 - It is a extended data buffer for spi_dm\\[1\\] signal in user mode when the width of spi_dm is 2-bit (DQ16). The output value of spi_dm\\[1\\] is the data that has been pushed to DM_FIFO \\[1\\]. If DM_FIFO is empty, the output value of spi_data_en will be (~DM_ACT). Available only if configuration of DM_EN and DQ16 are defined."]
    #[inline(always)]
    pub fn data_mask_dr_extend9(&mut self) -> DataMaskDrExtend9W<'_, DmDr9Spec> {
        DataMaskDrExtend9W::new(self, 1)
    }
}
#[doc = "Data Mask Data Register 9\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dm_dr9::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmDr9Spec;
impl crate::RegisterSpec for DmDr9Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dm_dr9::W`](W) writer structure"]
impl crate::Writable for DmDr9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DM_DR9 to value 0"]
impl crate::Resettable for DmDr9Spec {}
