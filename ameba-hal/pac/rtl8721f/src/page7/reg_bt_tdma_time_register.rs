#[doc = "Register `REG_BT_TDMA_Time_Register` reader"]
pub type R = crate::R<RegBtTdmaTimeRegisterSpec>;
#[doc = "Register `REG_BT_TDMA_Time_Register` writer"]
pub type W = crate::W<RegBtTdmaTimeRegisterSpec>;
#[doc = "Field `BIT_BT_RPT_SAMPLE_RATE` reader - When wifi detected the raising edge of BT report packet, wifi need to counter BT_RPT_SAMPLE_RATE. Then hw could detect which is high or low. Uint: 20 ns"]
pub type BitBtRptSampleRateR = crate::FieldReader;
#[doc = "Field `BIT_BT_RPT_SAMPLE_RATE` writer - When wifi detected the raising edge of BT report packet, wifi need to counter BT_RPT_SAMPLE_RATE. Then hw could detect which is high or low. Uint: 20 ns"]
pub type BitBtRptSampleRateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_BT_TIME` reader - The time is that WLAN_ACT is low. Unit: us"]
pub type BitBtTimeR = crate::FieldReader<u32>;
#[doc = "Field `BIT_BT_TIME` writer - The time is that WLAN_ACT is low. Unit: us"]
pub type BitBtTimeW<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:5 - When wifi detected the raising edge of BT report packet, wifi need to counter BT_RPT_SAMPLE_RATE. Then hw could detect which is high or low. Uint: 20 ns"]
    #[inline(always)]
    pub fn bit_bt_rpt_sample_rate(&self) -> BitBtRptSampleRateR {
        BitBtRptSampleRateR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:31 - The time is that WLAN_ACT is low. Unit: us"]
    #[inline(always)]
    pub fn bit_bt_time(&self) -> BitBtTimeR {
        BitBtTimeR::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - When wifi detected the raising edge of BT report packet, wifi need to counter BT_RPT_SAMPLE_RATE. Then hw could detect which is high or low. Uint: 20 ns"]
    #[inline(always)]
    pub fn bit_bt_rpt_sample_rate(&mut self) -> BitBtRptSampleRateW<'_, RegBtTdmaTimeRegisterSpec> {
        BitBtRptSampleRateW::new(self, 0)
    }
    #[doc = "Bits 6:31 - The time is that WLAN_ACT is low. Unit: us"]
    #[inline(always)]
    pub fn bit_bt_time(&mut self) -> BitBtTimeW<'_, RegBtTdmaTimeRegisterSpec> {
        BitBtTimeW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_tdma_time_register::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_tdma_time_register::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtTdmaTimeRegisterSpec;
impl crate::RegisterSpec for RegBtTdmaTimeRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_tdma_time_register::R`](R) reader structure"]
impl crate::Readable for RegBtTdmaTimeRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_tdma_time_register::W`](W) writer structure"]
impl crate::Writable for RegBtTdmaTimeRegisterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_TDMA_Time_Register to value 0"]
impl crate::Resettable for RegBtTdmaTimeRegisterSpec {}
