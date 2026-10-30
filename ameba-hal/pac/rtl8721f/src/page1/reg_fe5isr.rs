#[doc = "Register `REG_FE5ISR` reader"]
pub type R = crate::R<RegFe5isrSpec>;
#[doc = "Register `REG_FE5ISR` writer"]
pub type W = crate::W<RegFe5isrSpec>;
#[doc = "Field `BIT_FS_I2C_INTF4_EN` reader - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int form EXT BT,no use,tie 0"]
pub type BitFsI2cIntf4EnR = crate::BitReader;
#[doc = "Field `BIT_FS_bt_mailbox_i2c` reader - 1_wire Mailbox INT from internal BT"]
pub type BitFsBtMailboxI2cR = crate::BitReader;
#[doc = "Field `BIT_FS_bt2wl_sts_INT` reader - Mailbox INT from internal BT"]
pub type BitFsBt2wlStsIntR = crate::BitReader;
#[doc = "Field `BIT_FS_gnt_bt_bb_rise` reader - gnt bt rising edge"]
pub type BitFsGntBtBbRiseR = crate::BitReader;
#[doc = "Field `BIT_FS_gnt_bt_bb_fall` reader - gnt_bt falling edge"]
pub type BitFsGntBtBbFallR = crate::BitReader;
#[doc = "Field `BIT_FS_EXT_INTF4_EN` reader - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int from internal BT"]
pub type BitFsExtIntf4EnR = crate::BitReader;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
#[doc = "Field `BIT_FS_ZIGBEE2WLAN_MAILBOX` reader - Mailbox INT from ZIGBEE CPU"]
pub type BitFsZigbee2wlanMailboxR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int form EXT BT,no use,tie 0"]
    #[inline(always)]
    pub fn bit_fs_i2c_intf4_en(&self) -> BitFsI2cIntf4EnR {
        BitFsI2cIntf4EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1_wire Mailbox INT from internal BT"]
    #[inline(always)]
    pub fn bit_fs_bt_mailbox_i2c(&self) -> BitFsBtMailboxI2cR {
        BitFsBtMailboxI2cR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox INT from internal BT"]
    #[inline(always)]
    pub fn bit_fs_bt2wl_sts_int(&self) -> BitFsBt2wlStsIntR {
        BitFsBt2wlStsIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - gnt bt rising edge"]
    #[inline(always)]
    pub fn bit_fs_gnt_bt_bb_rise(&self) -> BitFsGntBtBbRiseR {
        BitFsGntBtBbRiseR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - gnt_bt falling edge"]
    #[inline(always)]
    pub fn bit_fs_gnt_bt_bb_fall(&self) -> BitFsGntBtBbFallR {
        BitFsGntBtBbFallR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int from internal BT"]
    #[inline(always)]
    pub fn bit_fs_ext_intf4_en(&self) -> BitFsExtIntf4EnR {
        BitFsExtIntf4EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 31 - Mailbox INT from ZIGBEE CPU"]
    #[inline(always)]
    pub fn bit_fs_zigbee2wlan_mailbox(&self) -> BitFsZigbee2wlanMailboxR {
        BitFsZigbee2wlanMailboxR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 6:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegFe5isrSpec> {
        RsvdW::new(self, 6)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe5isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fe5isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe5isrSpec;
impl crate::RegisterSpec for RegFe5isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe5isr::R`](R) reader structure"]
impl crate::Readable for RegFe5isrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fe5isr::W`](W) writer structure"]
impl crate::Writable for RegFe5isrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FE5ISR to value 0"]
impl crate::Resettable for RegFe5isrSpec {}
