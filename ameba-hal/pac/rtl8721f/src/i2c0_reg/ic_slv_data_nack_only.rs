#[doc = "Register `IC_SLV_DATA_NACK_ONLY` reader"]
pub type R = crate::R<IcSlvDataNackOnlySpec>;
#[doc = "Register `IC_SLV_DATA_NACK_ONLY` writer"]
pub type W = crate::W<IcSlvDataNackOnlySpec>;
#[doc = "Field `NACK` reader - Generate NACK. This NACK generation only occurs when I2C is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer. When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. * 1: generate NACK after data byte received * 0: generate NACK/ACK normally"]
pub type NackR = crate::BitReader;
#[doc = "Field `NACK` writer - Generate NACK. This NACK generation only occurs when I2C is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer. When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. * 1: generate NACK after data byte received * 0: generate NACK/ACK normally"]
pub type NackW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Generate NACK. This NACK generation only occurs when I2C is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer. When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. * 1: generate NACK after data byte received * 0: generate NACK/ACK normally"]
    #[inline(always)]
    pub fn nack(&self) -> NackR {
        NackR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Generate NACK. This NACK generation only occurs when I2C is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer. When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. * 1: generate NACK after data byte received * 0: generate NACK/ACK normally"]
    #[inline(always)]
    pub fn nack(&mut self) -> NackW<'_, IcSlvDataNackOnlySpec> {
        NackW::new(self, 0)
    }
}
#[doc = "Generate Slave Data NACK Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_slv_data_nack_only::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ic_slv_data_nack_only::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcSlvDataNackOnlySpec;
impl crate::RegisterSpec for IcSlvDataNackOnlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_slv_data_nack_only::R`](R) reader structure"]
impl crate::Readable for IcSlvDataNackOnlySpec {}
#[doc = "`write(|w| ..)` method takes [`ic_slv_data_nack_only::W`](W) writer structure"]
impl crate::Writable for IcSlvDataNackOnlySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IC_SLV_DATA_NACK_ONLY to value 0"]
impl crate::Resettable for IcSlvDataNackOnlySpec {}
