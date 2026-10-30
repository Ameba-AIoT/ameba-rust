#[doc = "Register `IC_ENABLE_STATUS` reader"]
pub type R = crate::R<IcEnableStatusSpec>;
#[doc = "Field `IC_EN` reader - ic_en Status. This bit always reflects the value driven on the output port ic_en. * 1: I2C is deemed to be in an enabled state. * 0: I2C is deemed completely inactive."]
pub type IcEnR = crate::BitReader;
#[doc = "Field `SLV_DISABLE_WHILE_BUSY` reader - Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting of the IC_SLAVE_DISBALE register from 1 to 0."]
pub type SlvDisableWhileBusyR = crate::BitReader;
#[doc = "Field `busy_in_mode` reader - * 00: I2C is disable while busy in legacy mode * 01: I2C is disable while busy in DMA mode * 10: I2C is disable while busy in Descriptor mode"]
pub type BusyInModeR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - ic_en Status. This bit always reflects the value driven on the output port ic_en. * 1: I2C is deemed to be in an enabled state. * 0: I2C is deemed completely inactive."]
    #[inline(always)]
    pub fn ic_en(&self) -> IcEnR {
        IcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting of the IC_SLAVE_DISBALE register from 1 to 0."]
    #[inline(always)]
    pub fn slv_disable_while_busy(&self) -> SlvDisableWhileBusyR {
        SlvDisableWhileBusyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - * 00: I2C is disable while busy in legacy mode * 01: I2C is disable while busy in DMA mode * 10: I2C is disable while busy in Descriptor mode"]
    #[inline(always)]
    pub fn busy_in_mode(&self) -> BusyInModeR {
        BusyInModeR::new(((self.bits >> 3) & 3) as u8)
    }
}
#[doc = "I2C Enable Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ic_enable_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcEnableStatusSpec;
impl crate::RegisterSpec for IcEnableStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ic_enable_status::R`](R) reader structure"]
impl crate::Readable for IcEnableStatusSpec {}
#[doc = "`reset()` method sets IC_ENABLE_STATUS to value 0"]
impl crate::Resettable for IcEnableStatusSpec {}
