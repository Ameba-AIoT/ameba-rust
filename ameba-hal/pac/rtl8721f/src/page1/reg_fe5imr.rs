#[doc = "Register `REG_FE5IMR` reader"]
pub type R = crate::R<RegFe5imrSpec>;
#[doc = "Register `REG_FE5IMR` writer"]
pub type W = crate::W<RegFe5imrSpec>;
#[doc = "Field `BIT_FS_I2C_INTF4_EN_MSK` reader - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int form EXT BT,no use,tie 0"]
pub type BitFsI2cIntf4EnMskR = crate::BitReader;
#[doc = "Field `BIT_FS_I2C_INTF4_EN_MSK` writer - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int form EXT BT,no use,tie 0"]
pub type BitFsI2cIntf4EnMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_bt_mailbox_i2c_MSK` reader - 1_wire Mailbox INT from internal BT"]
pub type BitFsBtMailboxI2cMskR = crate::BitReader;
#[doc = "Field `BIT_FS_bt_mailbox_i2c_MSK` writer - 1_wire Mailbox INT from internal BT"]
pub type BitFsBtMailboxI2cMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_bt2wl_sts_INT_MSK` reader - Mailbox INT from internal BT"]
pub type BitFsBt2wlStsIntMskR = crate::BitReader;
#[doc = "Field `BIT_FS_bt2wl_sts_INT_MSK` writer - Mailbox INT from internal BT"]
pub type BitFsBt2wlStsIntMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_gnt_bt_bb_rise_MSK` reader - gnt bt rising edge"]
pub type BitFsGntBtBbRiseMskR = crate::BitReader;
#[doc = "Field `BIT_FS_gnt_bt_bb_rise_MSK` writer - gnt bt rising edge"]
pub type BitFsGntBtBbRiseMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_gnt_bt_bb_fall_MSK` reader - gnt_bt falling edge"]
pub type BitFsGntBtBbFallMskR = crate::BitReader;
#[doc = "Field `BIT_FS_gnt_bt_bb_fall_MSK` writer - gnt_bt falling edge"]
pub type BitFsGntBtBbFallMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FS_EXT_INTF4_EN_MSK` reader - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int from internal BT"]
pub type BitFsExtIntf4EnMskR = crate::BitReader;
#[doc = "Field `BIT_FS_EXT_INTF4_EN_MSK` writer - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int from internal BT"]
pub type BitFsExtIntf4EnMskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - RSVD"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - RSVD"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
#[doc = "Field `BIT_FS_ZIGBEE2WLAN_MAILBOX_MSK` reader - Mailbox INT from ZIGBEE CPU"]
pub type BitFsZigbee2wlanMailboxMskR = crate::BitReader;
#[doc = "Field `BIT_FS_ZIGBEE2WLAN_MAILBOX_MSK` writer - Mailbox INT from ZIGBEE CPU"]
pub type BitFsZigbee2wlanMailboxMskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int form EXT BT,no use,tie 0"]
    #[inline(always)]
    pub fn bit_fs_i2c_intf4_en_msk(&self) -> BitFsI2cIntf4EnMskR {
        BitFsI2cIntf4EnMskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1_wire Mailbox INT from internal BT"]
    #[inline(always)]
    pub fn bit_fs_bt_mailbox_i2c_msk(&self) -> BitFsBtMailboxI2cMskR {
        BitFsBtMailboxI2cMskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Mailbox INT from internal BT"]
    #[inline(always)]
    pub fn bit_fs_bt2wl_sts_int_msk(&self) -> BitFsBt2wlStsIntMskR {
        BitFsBt2wlStsIntMskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - gnt bt rising edge"]
    #[inline(always)]
    pub fn bit_fs_gnt_bt_bb_rise_msk(&self) -> BitFsGntBtBbRiseMskR {
        BitFsGntBtBbRiseMskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - gnt_bt falling edge"]
    #[inline(always)]
    pub fn bit_fs_gnt_bt_bb_fall_msk(&self) -> BitFsGntBtBbFallMskR {
        BitFsGntBtBbFallMskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int from internal BT"]
    #[inline(always)]
    pub fn bit_fs_ext_intf4_en_msk(&self) -> BitFsExtIntf4EnMskR {
        BitFsExtIntf4EnMskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 6) & 0x01ff_ffff)
    }
    #[doc = "Bit 31 - Mailbox INT from ZIGBEE CPU"]
    #[inline(always)]
    pub fn bit_fs_zigbee2wlan_mailbox_msk(&self) -> BitFsZigbee2wlanMailboxMskR {
        BitFsZigbee2wlanMailboxMskR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int form EXT BT,no use,tie 0"]
    #[inline(always)]
    pub fn bit_fs_i2c_intf4_en_msk(&mut self) -> BitFsI2cIntf4EnMskW<'_, RegFe5imrSpec> {
        BitFsI2cIntf4EnMskW::new(self, 0)
    }
    #[doc = "Bit 1 - 1_wire Mailbox INT from internal BT"]
    #[inline(always)]
    pub fn bit_fs_bt_mailbox_i2c_msk(&mut self) -> BitFsBtMailboxI2cMskW<'_, RegFe5imrSpec> {
        BitFsBtMailboxI2cMskW::new(self, 1)
    }
    #[doc = "Bit 2 - Mailbox INT from internal BT"]
    #[inline(always)]
    pub fn bit_fs_bt2wl_sts_int_msk(&mut self) -> BitFsBt2wlStsIntMskW<'_, RegFe5imrSpec> {
        BitFsBt2wlStsIntMskW::new(self, 2)
    }
    #[doc = "Bit 3 - gnt bt rising edge"]
    #[inline(always)]
    pub fn bit_fs_gnt_bt_bb_rise_msk(&mut self) -> BitFsGntBtBbRiseMskW<'_, RegFe5imrSpec> {
        BitFsGntBtBbRiseMskW::new(self, 3)
    }
    #[doc = "Bit 4 - gnt_bt falling edge"]
    #[inline(always)]
    pub fn bit_fs_gnt_bt_bb_fall_msk(&mut self) -> BitFsGntBtBbFallMskW<'_, RegFe5imrSpec> {
        BitFsGntBtBbFallMskW::new(self, 4)
    }
    #[doc = "Bit 5 - bt mailbox int,mailbox_outempty_int| mailbox_in_rdy_int from internal BT"]
    #[inline(always)]
    pub fn bit_fs_ext_intf4_en_msk(&mut self) -> BitFsExtIntf4EnMskW<'_, RegFe5imrSpec> {
        BitFsExtIntf4EnMskW::new(self, 5)
    }
    #[doc = "Bits 6:30 - RSVD"]
    #[inline(always)]
    pub fn rsvd(&mut self) -> RsvdW<'_, RegFe5imrSpec> {
        RsvdW::new(self, 6)
    }
    #[doc = "Bit 31 - Mailbox INT from ZIGBEE CPU"]
    #[inline(always)]
    pub fn bit_fs_zigbee2wlan_mailbox_msk(
        &mut self,
    ) -> BitFsZigbee2wlanMailboxMskW<'_, RegFe5imrSpec> {
        BitFsZigbee2wlanMailboxMskW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fe5imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fe5imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFe5imrSpec;
impl crate::RegisterSpec for RegFe5imrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fe5imr::R`](R) reader structure"]
impl crate::Readable for RegFe5imrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fe5imr::W`](W) writer structure"]
impl crate::Writable for RegFe5imrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FE5IMR to value 0"]
impl crate::Resettable for RegFe5imrSpec {}
