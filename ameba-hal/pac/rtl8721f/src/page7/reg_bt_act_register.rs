#[doc = "Register `REG_BT_ACT_Register` reader"]
pub type R = crate::R<RegBtActRegisterSpec>;
#[doc = "Register `REG_BT_ACT_Register` writer"]
pub type W = crate::W<RegBtActRegisterSpec>;
#[doc = "Field `BIT_BT_CH` reader - When hardware receive cm_id \"1\", it will recorder to this."]
pub type BitBtChR = crate::FieldReader;
#[doc = "Field `BIT_TDMA_TO_ISR` reader - When BT_TIME expired, it will trigger an interrupt."]
pub type BitTdmaToIsrR = crate::BitReader;
#[doc = "Field `BIT_BT_ACT_RISING_ISR` reader - When BT_ACT signal is raising edge, it will trigger an interrupt"]
pub type BitBtActRisingIsrR = crate::BitReader;
#[doc = "Field `BIT_BT_ACT_FALLING_ISR` reader - When BT_ACT signal is falling edge, it will trigger an interrupt."]
pub type BitBtActFallingIsrR = crate::BitReader;
#[doc = "Field `BIT_BT_EISR_EN` reader - BT eisr enable"]
pub type BitBtEisrEnR = crate::FieldReader;
#[doc = "Field `BIT_BT_EISR_EN` writer - BT eisr enable"]
pub type BitBtEisrEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_B_AX_BT_BLE_EN` reader - Decide the GNT_BT_TX / RX 0: GNT_BT_TX = GNT_BT & BT_TX ; GNT_BT_RX = GNT_BT & BT_RX 1: GNT_BT_TX = GNT_BT & (BT_TX | BT_BLE) ; GNT_BT_RX = GNT_BT & (BT_RX | BT_BLE)"]
pub type BitBAxBtBleEnR = crate::BitReader;
#[doc = "Field `BIT_B_AX_BT_BLE_EN` writer - Decide the GNT_BT_TX / RX 0: GNT_BT_TX = GNT_BT & BT_TX ; GNT_BT_RX = GNT_BT & BT_RX 1: GNT_BT_TX = GNT_BT & (BT_TX | BT_BLE) ; GNT_BT_RX = GNT_BT & (BT_RX | BT_BLE)"]
pub type BitBAxBtBleEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - When hardware receive cm_id \"1\", it will recorder to this."]
    #[inline(always)]
    pub fn bit_bt_ch(&self) -> BitBtChR {
        BitBtChR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - When BT_TIME expired, it will trigger an interrupt."]
    #[inline(always)]
    pub fn bit_tdma_to_isr(&self) -> BitTdmaToIsrR {
        BitTdmaToIsrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When BT_ACT signal is raising edge, it will trigger an interrupt"]
    #[inline(always)]
    pub fn bit_bt_act_rising_isr(&self) -> BitBtActRisingIsrR {
        BitBtActRisingIsrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When BT_ACT signal is falling edge, it will trigger an interrupt."]
    #[inline(always)]
    pub fn bit_bt_act_falling_isr(&self) -> BitBtActFallingIsrR {
        BitBtActFallingIsrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:23 - BT eisr enable"]
    #[inline(always)]
    pub fn bit_bt_eisr_en(&self) -> BitBtEisrEnR {
        BitBtEisrEnR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Decide the GNT_BT_TX / RX 0: GNT_BT_TX = GNT_BT & BT_TX ; GNT_BT_RX = GNT_BT & BT_RX 1: GNT_BT_TX = GNT_BT & (BT_TX | BT_BLE) ; GNT_BT_RX = GNT_BT & (BT_RX | BT_BLE)"]
    #[inline(always)]
    pub fn bit_b_ax_bt_ble_en(&self) -> BitBAxBtBleEnR {
        BitBAxBtBleEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 16:23 - BT eisr enable"]
    #[inline(always)]
    pub fn bit_bt_eisr_en(&mut self) -> BitBtEisrEnW<'_, RegBtActRegisterSpec> {
        BitBtEisrEnW::new(self, 16)
    }
    #[doc = "Bit 24 - Decide the GNT_BT_TX / RX 0: GNT_BT_TX = GNT_BT & BT_TX ; GNT_BT_RX = GNT_BT & BT_RX 1: GNT_BT_TX = GNT_BT & (BT_TX | BT_BLE) ; GNT_BT_RX = GNT_BT & (BT_RX | BT_BLE)"]
    #[inline(always)]
    pub fn bit_b_ax_bt_ble_en(&mut self) -> BitBAxBtBleEnW<'_, RegBtActRegisterSpec> {
        BitBAxBtBleEnW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bt_act_register::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bt_act_register::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBtActRegisterSpec;
impl crate::RegisterSpec for RegBtActRegisterSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bt_act_register::R`](R) reader structure"]
impl crate::Readable for RegBtActRegisterSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bt_act_register::W`](W) writer structure"]
impl crate::Writable for RegBtActRegisterSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BT_ACT_Register to value 0x7f"]
impl crate::Resettable for RegBtActRegisterSpec {
    const RESET_VALUE: u32 = 0x7f;
}
