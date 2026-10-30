#[doc = "Register `REG_PWRBIT_SETTING` reader"]
pub type R = crate::R<RegPwrbitSettingSpec>;
#[doc = "Register `REG_PWRBIT_SETTING` writer"]
pub type W = crate::W<RegPwrbitSettingSpec>;
#[doc = "Field `BIT_CLI0_PWR_ST` reader - For client0 (port1). MAC will overwrite power bit of frame control according to CLI0_PWR_ST.(only for data frame)"]
pub type BitCli0PwrStR = crate::BitReader;
#[doc = "Field `BIT_CLI0_PWR_ST` writer - For client0 (port1). MAC will overwrite power bit of frame control according to CLI0_PWR_ST.(only for data frame)"]
pub type BitCli0PwrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLI0_PWRBIT_OW_EN` reader - For client0 (port1). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI0_PWR_ST. (only for data frame)"]
pub type BitCli0PwrbitOwEnR = crate::BitReader;
#[doc = "Field `BIT_CLI0_PWRBIT_OW_EN` writer - For client0 (port1). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI0_PWR_ST. (only for data frame)"]
pub type BitCli0PwrbitOwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLI1_PWR_ST` reader - For client1 (port2). MAC will overwrite power bit of frame control according to CLI1_PWR_ST.(only for data frame)"]
pub type BitCli1PwrStR = crate::BitReader;
#[doc = "Field `BIT_CLI1_PWR_ST` writer - For client1 (port2). MAC will overwrite power bit of frame control according to CLI1_PWR_ST.(only for data frame)"]
pub type BitCli1PwrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLI1_PWRBIT_OW_EN` reader - For client1 (port2). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI1_PWR_ST. (only for data frame)"]
pub type BitCli1PwrbitOwEnR = crate::BitReader;
#[doc = "Field `BIT_CLI1_PWRBIT_OW_EN` writer - For client1 (port2). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI1_PWR_ST. (only for data frame)"]
pub type BitCli1PwrbitOwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLI2_PWR_ST` reader - For client2 (port3). MAC will overwrite power bit of frame control according to CLI2_PWR_ST.(only for data frame)"]
pub type BitCli2PwrStR = crate::BitReader;
#[doc = "Field `BIT_CLI2_PWR_ST` writer - For client2 (port3). MAC will overwrite power bit of frame control according to CLI2_PWR_ST.(only for data frame)"]
pub type BitCli2PwrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLI2_PWRBIT_OW_EN` reader - For client2 (port3). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI2_PWR_ST. (only for data frame)"]
pub type BitCli2PwrbitOwEnR = crate::BitReader;
#[doc = "Field `BIT_CLI2_PWRBIT_OW_EN` writer - For client2 (port3). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI2_PWR_ST. (only for data frame)"]
pub type BitCli2PwrbitOwEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLI3_PWR_ST` reader - For client3 (port4). MAC will overwrite power bit of frame control according to CLI3_PWR_ST.(only for data frame)"]
pub type BitCli3PwrStR = crate::BitReader;
#[doc = "Field `BIT_CLI3_PWR_ST` writer - For client3 (port4). MAC will overwrite power bit of frame control according to CLI3_PWR_ST.(only for data frame)"]
pub type BitCli3PwrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CLI3_PWRBIT_OW_EN` reader - For client3 (port4). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI3_PWR_ST. (only for data frame)"]
pub type BitCli3PwrbitOwEnR = crate::BitReader;
#[doc = "Field `BIT_CLI3_PWRBIT_OW_EN` writer - For client3 (port4). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI3_PWR_ST. (only for data frame)"]
pub type BitCli3PwrbitOwEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - For client0 (port1). MAC will overwrite power bit of frame control according to CLI0_PWR_ST.(only for data frame)"]
    #[inline(always)]
    pub fn bit_cli0_pwr_st(&self) -> BitCli0PwrStR {
        BitCli0PwrStR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For client0 (port1). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI0_PWR_ST. (only for data frame)"]
    #[inline(always)]
    pub fn bit_cli0_pwrbit_ow_en(&self) -> BitCli0PwrbitOwEnR {
        BitCli0PwrbitOwEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For client1 (port2). MAC will overwrite power bit of frame control according to CLI1_PWR_ST.(only for data frame)"]
    #[inline(always)]
    pub fn bit_cli1_pwr_st(&self) -> BitCli1PwrStR {
        BitCli1PwrStR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - For client1 (port2). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI1_PWR_ST. (only for data frame)"]
    #[inline(always)]
    pub fn bit_cli1_pwrbit_ow_en(&self) -> BitCli1PwrbitOwEnR {
        BitCli1PwrbitOwEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For client2 (port3). MAC will overwrite power bit of frame control according to CLI2_PWR_ST.(only for data frame)"]
    #[inline(always)]
    pub fn bit_cli2_pwr_st(&self) -> BitCli2PwrStR {
        BitCli2PwrStR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For client2 (port3). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI2_PWR_ST. (only for data frame)"]
    #[inline(always)]
    pub fn bit_cli2_pwrbit_ow_en(&self) -> BitCli2PwrbitOwEnR {
        BitCli2PwrbitOwEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For client3 (port4). MAC will overwrite power bit of frame control according to CLI3_PWR_ST.(only for data frame)"]
    #[inline(always)]
    pub fn bit_cli3_pwr_st(&self) -> BitCli3PwrStR {
        BitCli3PwrStR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - For client3 (port4). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI3_PWR_ST. (only for data frame)"]
    #[inline(always)]
    pub fn bit_cli3_pwrbit_ow_en(&self) -> BitCli3PwrbitOwEnR {
        BitCli3PwrbitOwEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - For client0 (port1). MAC will overwrite power bit of frame control according to CLI0_PWR_ST.(only for data frame)"]
    #[inline(always)]
    pub fn bit_cli0_pwr_st(&mut self) -> BitCli0PwrStW<'_, RegPwrbitSettingSpec> {
        BitCli0PwrStW::new(self, 0)
    }
    #[doc = "Bit 1 - For client0 (port1). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI0_PWR_ST. (only for data frame)"]
    #[inline(always)]
    pub fn bit_cli0_pwrbit_ow_en(&mut self) -> BitCli0PwrbitOwEnW<'_, RegPwrbitSettingSpec> {
        BitCli0PwrbitOwEnW::new(self, 1)
    }
    #[doc = "Bit 2 - For client1 (port2). MAC will overwrite power bit of frame control according to CLI1_PWR_ST.(only for data frame)"]
    #[inline(always)]
    pub fn bit_cli1_pwr_st(&mut self) -> BitCli1PwrStW<'_, RegPwrbitSettingSpec> {
        BitCli1PwrStW::new(self, 2)
    }
    #[doc = "Bit 3 - For client1 (port2). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI1_PWR_ST. (only for data frame)"]
    #[inline(always)]
    pub fn bit_cli1_pwrbit_ow_en(&mut self) -> BitCli1PwrbitOwEnW<'_, RegPwrbitSettingSpec> {
        BitCli1PwrbitOwEnW::new(self, 3)
    }
    #[doc = "Bit 4 - For client2 (port3). MAC will overwrite power bit of frame control according to CLI2_PWR_ST.(only for data frame)"]
    #[inline(always)]
    pub fn bit_cli2_pwr_st(&mut self) -> BitCli2PwrStW<'_, RegPwrbitSettingSpec> {
        BitCli2PwrStW::new(self, 4)
    }
    #[doc = "Bit 5 - For client2 (port3). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI2_PWR_ST. (only for data frame)"]
    #[inline(always)]
    pub fn bit_cli2_pwrbit_ow_en(&mut self) -> BitCli2PwrbitOwEnW<'_, RegPwrbitSettingSpec> {
        BitCli2PwrbitOwEnW::new(self, 5)
    }
    #[doc = "Bit 6 - For client3 (port4). MAC will overwrite power bit of frame control according to CLI3_PWR_ST.(only for data frame)"]
    #[inline(always)]
    pub fn bit_cli3_pwr_st(&mut self) -> BitCli3PwrStW<'_, RegPwrbitSettingSpec> {
        BitCli3PwrStW::new(self, 6)
    }
    #[doc = "Bit 7 - For client3 (port4). If this bit is set to 1, MAC will overwrite power bit of frame control according to CLI3_PWR_ST. (only for data frame)"]
    #[inline(always)]
    pub fn bit_cli3_pwrbit_ow_en(&mut self) -> BitCli3PwrbitOwEnW<'_, RegPwrbitSettingSpec> {
        BitCli3PwrbitOwEnW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_pwrbit_setting::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_pwrbit_setting::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPwrbitSettingSpec;
impl crate::RegisterSpec for RegPwrbitSettingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_pwrbit_setting::R`](R) reader structure"]
impl crate::Readable for RegPwrbitSettingSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_pwrbit_setting::W`](W) writer structure"]
impl crate::Writable for RegPwrbitSettingSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PWRBIT_SETTING to value 0"]
impl crate::Resettable for RegPwrbitSettingSpec {}
