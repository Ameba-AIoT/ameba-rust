#[doc = "Register `REG_PSDO_I2C_CTL` reader"]
pub type R = crate::R<RegPsdoI2cCtlSpec>;
#[doc = "Register `REG_PSDO_I2C_CTL` writer"]
pub type W = crate::W<RegPsdoI2cCtlSpec>;
#[doc = "Field `r_i2c_clk_div` reader - clock division factor = r_i2c_clk_div * 2, minimum value is 2"]
pub type RI2cClkDivR = crate::FieldReader;
#[doc = "Field `r_i2c_clk_div` writer - clock division factor = r_i2c_clk_div * 2, minimum value is 2"]
pub type RI2cClkDivW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `r_i2c_tx_status` reader - 1: psdo i2c is idle, 0: psdo i2c is working"]
pub type RI2cTxStatusR = crate::BitReader;
#[doc = "Field `r_psedo_i2c_en` reader - indicate psedo i2c in working, only when this bit=1 can access psedo i2c ram"]
pub type RPsedoI2cEnR = crate::BitReader;
#[doc = "Field `r_psedo_i2c_en` writer - indicate psedo i2c in working, only when this bit=1 can access psedo i2c ram"]
pub type RPsedoI2cEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_i2c_data_num` reader - psdo i2c data num, unit: byte"]
pub type RI2cDataNumR = crate::FieldReader;
#[doc = "Field `r_i2c_data_num` writer - psdo i2c data num, unit: byte"]
pub type RI2cDataNumW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `dummy` reader - dummy"]
pub type DummyR = crate::BitReader;
#[doc = "Field `dummy` writer - dummy"]
pub type DummyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - clock division factor = r_i2c_clk_div * 2, minimum value is 2"]
    #[inline(always)]
    pub fn r_i2c_clk_div(&self) -> RI2cClkDivR {
        RI2cClkDivR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 1: psdo i2c is idle, 0: psdo i2c is working"]
    #[inline(always)]
    pub fn r_i2c_tx_status(&self) -> RI2cTxStatusR {
        RI2cTxStatusR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - indicate psedo i2c in working, only when this bit=1 can access psedo i2c ram"]
    #[inline(always)]
    pub fn r_psedo_i2c_en(&self) -> RPsedoI2cEnR {
        RPsedoI2cEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - psdo i2c data num, unit: byte"]
    #[inline(always)]
    pub fn r_i2c_data_num(&self) -> RI2cDataNumR {
        RI2cDataNumR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - dummy"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - clock division factor = r_i2c_clk_div * 2, minimum value is 2"]
    #[inline(always)]
    pub fn r_i2c_clk_div(&mut self) -> RI2cClkDivW<'_, RegPsdoI2cCtlSpec> {
        RI2cClkDivW::new(self, 0)
    }
    #[doc = "Bit 7 - indicate psedo i2c in working, only when this bit=1 can access psedo i2c ram"]
    #[inline(always)]
    pub fn r_psedo_i2c_en(&mut self) -> RPsedoI2cEnW<'_, RegPsdoI2cCtlSpec> {
        RPsedoI2cEnW::new(self, 7)
    }
    #[doc = "Bits 8:14 - psdo i2c data num, unit: byte"]
    #[inline(always)]
    pub fn r_i2c_data_num(&mut self) -> RI2cDataNumW<'_, RegPsdoI2cCtlSpec> {
        RI2cDataNumW::new(self, 8)
    }
    #[doc = "Bit 15 - dummy"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegPsdoI2cCtlSpec> {
        DummyW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_psdo_i2c_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_psdo_i2c_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPsdoI2cCtlSpec;
impl crate::RegisterSpec for RegPsdoI2cCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_psdo_i2c_ctl::R`](R) reader structure"]
impl crate::Readable for RegPsdoI2cCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_psdo_i2c_ctl::W`](W) writer structure"]
impl crate::Writable for RegPsdoI2cCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PSDO_I2C_CTL to value 0x40"]
impl crate::Resettable for RegPsdoI2cCtlSpec {
    const RESET_VALUE: u32 = 0x40;
}
