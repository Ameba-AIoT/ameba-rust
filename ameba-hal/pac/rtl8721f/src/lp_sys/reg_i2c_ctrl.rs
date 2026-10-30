#[doc = "Register `REG_I2C_CTRL` reader"]
pub type R = crate::R<RegI2cCtrlSpec>;
#[doc = "Register `REG_I2C_CTRL` writer"]
pub type W = crate::W<RegI2cCtrlSpec>;
#[doc = "Field `i2c0_tx_swap` reader - I2C0 TX swap control"]
pub type I2c0TxSwapR = crate::BitReader;
#[doc = "Field `i2c0_tx_swap` writer - I2C0 TX swap control"]
pub type I2c0TxSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c0_rx_swap` reader - I2C0 RX swap control"]
pub type I2c0RxSwapR = crate::BitReader;
#[doc = "Field `i2c0_rx_swap` writer - I2C0 RX swap control"]
pub type I2c0RxSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_tx_swap` reader - I2C1 TX swap control"]
pub type I2c1TxSwapR = crate::BitReader;
#[doc = "Field `i2c1_tx_swap` writer - I2C1 TX swap control"]
pub type I2c1TxSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `i2c1_rx_swap` reader - I2C1 RX swap control"]
pub type I2c1RxSwapR = crate::BitReader;
#[doc = "Field `i2c1_rx_swap` writer - I2C1 RX swap control"]
pub type I2c1RxSwapW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C0 TX swap control"]
    #[inline(always)]
    pub fn i2c0_tx_swap(&self) -> I2c0TxSwapR {
        I2c0TxSwapR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C0 RX swap control"]
    #[inline(always)]
    pub fn i2c0_rx_swap(&self) -> I2c0RxSwapR {
        I2c0RxSwapR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C1 TX swap control"]
    #[inline(always)]
    pub fn i2c1_tx_swap(&self) -> I2c1TxSwapR {
        I2c1TxSwapR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C1 RX swap control"]
    #[inline(always)]
    pub fn i2c1_rx_swap(&self) -> I2c1RxSwapR {
        I2c1RxSwapR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C0 TX swap control"]
    #[inline(always)]
    pub fn i2c0_tx_swap(&mut self) -> I2c0TxSwapW<'_, RegI2cCtrlSpec> {
        I2c0TxSwapW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C0 RX swap control"]
    #[inline(always)]
    pub fn i2c0_rx_swap(&mut self) -> I2c0RxSwapW<'_, RegI2cCtrlSpec> {
        I2c0RxSwapW::new(self, 1)
    }
    #[doc = "Bit 2 - I2C1 TX swap control"]
    #[inline(always)]
    pub fn i2c1_tx_swap(&mut self) -> I2c1TxSwapW<'_, RegI2cCtrlSpec> {
        I2c1TxSwapW::new(self, 2)
    }
    #[doc = "Bit 3 - I2C1 RX swap control"]
    #[inline(always)]
    pub fn i2c1_rx_swap(&mut self) -> I2c1RxSwapW<'_, RegI2cCtrlSpec> {
        I2c1RxSwapW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_i2c_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_i2c_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegI2cCtrlSpec;
impl crate::RegisterSpec for RegI2cCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_i2c_ctrl::R`](R) reader structure"]
impl crate::Readable for RegI2cCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_i2c_ctrl::W`](W) writer structure"]
impl crate::Writable for RegI2cCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_I2C_CTRL to value 0"]
impl crate::Resettable for RegI2cCtrlSpec {}
