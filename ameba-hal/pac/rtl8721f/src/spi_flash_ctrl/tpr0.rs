#[doc = "Register `TPR0` reader"]
pub type R = crate::R<Tpr0Spec>;
#[doc = "Register `TPR0` writer"]
pub type W = crate::W<Tpr0Spec>;
#[doc = "Field `CS_H_RD_DUM_LEN` reader - Dummy cycle between sending read command to SPI Flash. Using the dummy cycles to avoid the timing violation of CS high time."]
pub type CsHRdDumLenR = crate::FieldReader;
#[doc = "Field `CS_H_RD_DUM_LEN` writer - Dummy cycle between sending read command to SPI Flash. Using the dummy cycles to avoid the timing violation of CS high time."]
pub type CsHRdDumLenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CS_H_WR_DUM_LEN` reader - Dummy cycle between sending write command to SPI Flash. Using the dummy cycles to avoid the timing violation of CS high time."]
pub type CsHWrDumLenR = crate::FieldReader;
#[doc = "Field `CS_H_WR_DUM_LEN` writer - Dummy cycle between sending write command to SPI Flash. Using the dummy cycles to avoid the timing violation of CS high time."]
pub type CsHWrDumLenW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CS_ACTIVE_HOLD` reader - For Flash chip select active hold time after SCLK rising edge (refer to tSLCH/CHSH, tCSS/tCSH) Note: Wait most 16 bus_clk cycles before CS inactive. (Implement CS_ACTIVE_Hold timing only, CS_ACTIVE_Setup timing using D-PHY to shift)"]
pub type CsActiveHoldR = crate::FieldReader;
#[doc = "Field `CS_ACTIVE_HOLD` writer - For Flash chip select active hold time after SCLK rising edge (refer to tSLCH/CHSH, tCSS/tCSH) Note: Wait most 16 bus_clk cycles before CS inactive. (Implement CS_ACTIVE_Hold timing only, CS_ACTIVE_Setup timing using D-PHY to shift)"]
pub type CsActiveHoldW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CS_SEQ_TIMEOUT` reader - The timeout setting of auto command after sequential read command. If set CS_SEQ_TIMEOUT = 00 will disable this function. If set CS_SEQ_TIMEOUT > 00, SPIC will exit sequential transaction read and inactive CS when there is no auto read/write command for (CS_SEQ_TIMEOUT *4) bus_clocks."]
pub type CsSeqTimeoutR = crate::FieldReader;
#[doc = "Field `CS_SEQ_TIMEOUT` writer - The timeout setting of auto command after sequential read command. If set CS_SEQ_TIMEOUT = 00 will disable this function. If set CS_SEQ_TIMEOUT > 00, SPIC will exit sequential transaction read and inactive CS when there is no auto read/write command for (CS_SEQ_TIMEOUT *4) bus_clocks."]
pub type CsSeqTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CS_TCEM` reader - Set to chop auto cmd when CSN low pulse width = (CS_TCEM*32) bus_clk. If set CS_TCEM = 0 will disable this function. ex. PSRAM tCEM = 4us, SPIC frequency = 100Mhz. CS_TCEM = ((4000 ns/10 ns)/32) - BYTE_DELAY = 12 (c) - BYTE_DELAY If ((16*baud_rate)/data_channel) <=32. BYTE_DELAY = 0. If ((16*baud_rate)/data_channel) > 32. BYTE_DELAY = (((16*baud_rate)/data_channel) - 32)/32. ex. If baud_rate = 3, data_channel = 1. BYTE_DELAY = (((16*3)/1) - 32)/32 = 16/32 = 0.5 Therefore, BYTE_DELAY should be 1 in this example. Note: Available only if configuration of PSRAM_EN is defined."]
pub type CsTcemR = crate::FieldReader;
#[doc = "Field `CS_TCEM` writer - Set to chop auto cmd when CSN low pulse width = (CS_TCEM*32) bus_clk. If set CS_TCEM = 0 will disable this function. ex. PSRAM tCEM = 4us, SPIC frequency = 100Mhz. CS_TCEM = ((4000 ns/10 ns)/32) - BYTE_DELAY = 12 (c) - BYTE_DELAY If ((16*baud_rate)/data_channel) <=32. BYTE_DELAY = 0. If ((16*baud_rate)/data_channel) > 32. BYTE_DELAY = (((16*baud_rate)/data_channel) - 32)/32. ex. If baud_rate = 3, data_channel = 1. BYTE_DELAY = (((16*3)/1) - 32)/32 = 16/32 = 0.5 Therefore, BYTE_DELAY should be 1 in this example. Note: Available only if configuration of PSRAM_EN is defined."]
pub type CsTcemW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - Dummy cycle between sending read command to SPI Flash. Using the dummy cycles to avoid the timing violation of CS high time."]
    #[inline(always)]
    pub fn cs_h_rd_dum_len(&self) -> CsHRdDumLenR {
        CsHRdDumLenR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - Dummy cycle between sending write command to SPI Flash. Using the dummy cycles to avoid the timing violation of CS high time."]
    #[inline(always)]
    pub fn cs_h_wr_dum_len(&self) -> CsHWrDumLenR {
        CsHWrDumLenR::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:15 - For Flash chip select active hold time after SCLK rising edge (refer to tSLCH/CHSH, tCSS/tCSH) Note: Wait most 16 bus_clk cycles before CS inactive. (Implement CS_ACTIVE_Hold timing only, CS_ACTIVE_Setup timing using D-PHY to shift)"]
    #[inline(always)]
    pub fn cs_active_hold(&self) -> CsActiveHoldR {
        CsActiveHoldR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - The timeout setting of auto command after sequential read command. If set CS_SEQ_TIMEOUT = 00 will disable this function. If set CS_SEQ_TIMEOUT > 00, SPIC will exit sequential transaction read and inactive CS when there is no auto read/write command for (CS_SEQ_TIMEOUT *4) bus_clocks."]
    #[inline(always)]
    pub fn cs_seq_timeout(&self) -> CsSeqTimeoutR {
        CsSeqTimeoutR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Set to chop auto cmd when CSN low pulse width = (CS_TCEM*32) bus_clk. If set CS_TCEM = 0 will disable this function. ex. PSRAM tCEM = 4us, SPIC frequency = 100Mhz. CS_TCEM = ((4000 ns/10 ns)/32) - BYTE_DELAY = 12 (c) - BYTE_DELAY If ((16*baud_rate)/data_channel) <=32. BYTE_DELAY = 0. If ((16*baud_rate)/data_channel) > 32. BYTE_DELAY = (((16*baud_rate)/data_channel) - 32)/32. ex. If baud_rate = 3, data_channel = 1. BYTE_DELAY = (((16*3)/1) - 32)/32 = 16/32 = 0.5 Therefore, BYTE_DELAY should be 1 in this example. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn cs_tcem(&self) -> CsTcemR {
        CsTcemR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dummy cycle between sending read command to SPI Flash. Using the dummy cycles to avoid the timing violation of CS high time."]
    #[inline(always)]
    pub fn cs_h_rd_dum_len(&mut self) -> CsHRdDumLenW<'_, Tpr0Spec> {
        CsHRdDumLenW::new(self, 0)
    }
    #[doc = "Bits 6:11 - Dummy cycle between sending write command to SPI Flash. Using the dummy cycles to avoid the timing violation of CS high time."]
    #[inline(always)]
    pub fn cs_h_wr_dum_len(&mut self) -> CsHWrDumLenW<'_, Tpr0Spec> {
        CsHWrDumLenW::new(self, 6)
    }
    #[doc = "Bits 12:15 - For Flash chip select active hold time after SCLK rising edge (refer to tSLCH/CHSH, tCSS/tCSH) Note: Wait most 16 bus_clk cycles before CS inactive. (Implement CS_ACTIVE_Hold timing only, CS_ACTIVE_Setup timing using D-PHY to shift)"]
    #[inline(always)]
    pub fn cs_active_hold(&mut self) -> CsActiveHoldW<'_, Tpr0Spec> {
        CsActiveHoldW::new(self, 12)
    }
    #[doc = "Bits 16:23 - The timeout setting of auto command after sequential read command. If set CS_SEQ_TIMEOUT = 00 will disable this function. If set CS_SEQ_TIMEOUT > 00, SPIC will exit sequential transaction read and inactive CS when there is no auto read/write command for (CS_SEQ_TIMEOUT *4) bus_clocks."]
    #[inline(always)]
    pub fn cs_seq_timeout(&mut self) -> CsSeqTimeoutW<'_, Tpr0Spec> {
        CsSeqTimeoutW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Set to chop auto cmd when CSN low pulse width = (CS_TCEM*32) bus_clk. If set CS_TCEM = 0 will disable this function. ex. PSRAM tCEM = 4us, SPIC frequency = 100Mhz. CS_TCEM = ((4000 ns/10 ns)/32) - BYTE_DELAY = 12 (c) - BYTE_DELAY If ((16*baud_rate)/data_channel) <=32. BYTE_DELAY = 0. If ((16*baud_rate)/data_channel) > 32. BYTE_DELAY = (((16*baud_rate)/data_channel) - 32)/32. ex. If baud_rate = 3, data_channel = 1. BYTE_DELAY = (((16*3)/1) - 32)/32 = 16/32 = 0.5 Therefore, BYTE_DELAY should be 1 in this example. Note: Available only if configuration of PSRAM_EN is defined."]
    #[inline(always)]
    pub fn cs_tcem(&mut self) -> CsTcemW<'_, Tpr0Spec> {
        CsTcemW::new(self, 24)
    }
}
#[doc = "Timing parameters\n\nYou can [`read`](crate::Reg::read) this register and get [`tpr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tpr0Spec;
impl crate::RegisterSpec for Tpr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tpr0::R`](R) reader structure"]
impl crate::Readable for Tpr0Spec {}
#[doc = "`write(|w| ..)` method takes [`tpr0::W`](W) writer structure"]
impl crate::Writable for Tpr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TPR0 to value 0x000a_0c30"]
impl crate::Resettable for Tpr0Spec {
    const RESET_VALUE: u32 = 0x000a_0c30;
}
