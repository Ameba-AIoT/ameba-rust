#[doc = "Register `SHA_Control` reader"]
pub type R = crate::R<ShaControlSpec>;
#[doc = "Register `SHA_Control` writer"]
pub type W = crate::W<ShaControlSpec>;
#[doc = "Field `SHA2_mode` reader - 0x0: SHA-224 0x1: SHA-256 0x2: SHA-384 0x3: SHA-512"]
pub type Sha2ModeR = crate::FieldReader;
#[doc = "Field `SHA2_mode` writer - 0x0: SHA-224 0x1: SHA-256 0x2: SHA-384 0x3: SHA-512"]
pub type Sha2ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMA_mode` reader - 0x0: read-only mode 0x1: copy mode"]
pub type DmaModeR = crate::BitReader;
#[doc = "Field `DMA_mode` writer - 0x0: read-only mode 0x1: copy mode"]
pub type DmaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMAC_mode` reader - Only available when config HMAC_MODE 0x0: use HASH mode. 0x1: use HMAC mode."]
pub type HmacModeR = crate::BitReader;
#[doc = "Field `HMAC_mode` writer - Only available when config HMAC_MODE 0x0: use HASH mode. 0x1: use HMAC mode."]
pub type HmacModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICG_enable` reader - Enable auto clock gating"]
pub type IcgEnableR = crate::BitReader;
#[doc = "Field `ICG_enable` writer - Enable auto clock gating"]
pub type IcgEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Slave_mode` reader - 0: DMA mode. 1: slave mode. SW can push data by wrting FIFO Data Register in slave mode."]
pub type SlaveModeR = crate::BitReader;
#[doc = "Field `Slave_mode` writer - 0: DMA mode. 1: slave mode. SW can push data by wrting FIFO Data Register in slave mode."]
pub type SlaveModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMAC_Key_size` reader - Only available when config HMAC_MODE 0x0: 128 bits 0x1: 192 bits 0x2: 256 bits"]
pub type HmacKeySizeR = crate::FieldReader;
#[doc = "Field `HMAC_Key_size` writer - Only available when config HMAC_MODE 0x0: 128 bits 0x1: 192 bits 0x2: 256 bits"]
pub type HmacKeySizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SHA512_truncate_en` reader - Only available when config SUPPORT_TRUNCATED_SHA512 and SHA_mode = 0x3 (SHA-512) 0: normal SHA-512 1: truncated SHA-512"]
pub type Sha512TruncateEnR = crate::BitReader;
#[doc = "Field `SHA512_truncate_en` writer - Only available when config SUPPORT_TRUNCATED_SHA512 and SHA_mode = 0x3 (SHA-512) 0: normal SHA-512 1: truncated SHA-512"]
pub type Sha512TruncateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tSHA512_use_256` reader - Only available when config SUPPORT_TRUNCATED_SHA512 and SHA512_truncate_en= 0x1 (truncated SHA-512) 0: SHA-512/224 mode 1: SHA-512/256 mode"]
pub type TSha512Use256R = crate::BitReader;
#[doc = "Field `tSHA512_use_256` writer - Only available when config SUPPORT_TRUNCATED_SHA512 and SHA512_truncate_en= 0x1 (truncated SHA-512) 0: SHA-512/224 mode 1: SHA-512/256 mode"]
pub type TSha512Use256W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Payload_byte_inversion_enable` reader - 0x0: use native byte order in memory 0x1: inverse byte order of read data"]
pub type PayloadByteInversionEnableR = crate::BitReader;
#[doc = "Field `Payload_byte_inversion_enable` writer - 0x0: use native byte order in memory 0x1: inverse byte order of read data"]
pub type PayloadByteInversionEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Key_byte_inversion_enable` reader - 0x0: use native byte order in memory 0x1: inverse byte order of read data"]
pub type KeyByteInversionEnableR = crate::BitReader;
#[doc = "Field `Key_byte_inversion_enable` writer - 0x0: use native byte order in memory 0x1: inverse byte order of read data"]
pub type KeyByteInversionEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_full_suspend` reader - User writes data to SHA_FIFO_in but the FIFO space is full. 0x0: accept signal is always asserted 0x1: accept signal is not asserted until FIFO space is available Note: This function is only available when Slave_mode = 0x1"]
pub type FifoFullSuspendR = crate::BitReader;
#[doc = "Field `FIFO_full_suspend` writer - User writes data to SHA_FIFO_in but the FIFO space is full. 0x0: accept signal is always asserted 0x1: accept signal is not asserted until FIFO space is available Note: This function is only available when Slave_mode = 0x1"]
pub type FifoFullSuspendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD_2` reader - Reserved"]
pub type Rsvd2R = crate::FieldReader<u16>;
#[doc = "Field `soft_reset_trigger_busy` reader - Write 1 to trigger a soft reset. All registers except the mutex register will be reset. Read the bit until 0 to determine if the soft reset is complete."]
pub type SoftResetTriggerBusyR = crate::BitReader;
#[doc = "Field `soft_reset_trigger_busy` writer - Write 1 to trigger a soft reset. All registers except the mutex register will be reset. Read the bit until 0 to determine if the soft reset is complete."]
pub type SoftResetTriggerBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSVD` reader - Reserved"]
pub type RsvdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - 0x0: SHA-224 0x1: SHA-256 0x2: SHA-384 0x3: SHA-512"]
    #[inline(always)]
    pub fn sha2_mode(&self) -> Sha2ModeR {
        Sha2ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 0x0: read-only mode 0x1: copy mode"]
    #[inline(always)]
    pub fn dma_mode(&self) -> DmaModeR {
        DmaModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Only available when config HMAC_MODE 0x0: use HASH mode. 0x1: use HMAC mode."]
    #[inline(always)]
    pub fn hmac_mode(&self) -> HmacModeR {
        HmacModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable auto clock gating"]
    #[inline(always)]
    pub fn icg_enable(&self) -> IcgEnableR {
        IcgEnableR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: DMA mode. 1: slave mode. SW can push data by wrting FIFO Data Register in slave mode."]
    #[inline(always)]
    pub fn slave_mode(&self) -> SlaveModeR {
        SlaveModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Only available when config HMAC_MODE 0x0: 128 bits 0x1: 192 bits 0x2: 256 bits"]
    #[inline(always)]
    pub fn hmac_key_size(&self) -> HmacKeySizeR {
        HmacKeySizeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Only available when config SUPPORT_TRUNCATED_SHA512 and SHA_mode = 0x3 (SHA-512) 0: normal SHA-512 1: truncated SHA-512"]
    #[inline(always)]
    pub fn sha512_truncate_en(&self) -> Sha512TruncateEnR {
        Sha512TruncateEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Only available when config SUPPORT_TRUNCATED_SHA512 and SHA512_truncate_en= 0x1 (truncated SHA-512) 0: SHA-512/224 mode 1: SHA-512/256 mode"]
    #[inline(always)]
    pub fn t_sha512_use_256(&self) -> TSha512Use256R {
        TSha512Use256R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 0x0: use native byte order in memory 0x1: inverse byte order of read data"]
    #[inline(always)]
    pub fn payload_byte_inversion_enable(&self) -> PayloadByteInversionEnableR {
        PayloadByteInversionEnableR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 0x0: use native byte order in memory 0x1: inverse byte order of read data"]
    #[inline(always)]
    pub fn key_byte_inversion_enable(&self) -> KeyByteInversionEnableR {
        KeyByteInversionEnableR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - User writes data to SHA_FIFO_in but the FIFO space is full. 0x0: accept signal is always asserted 0x1: accept signal is not asserted until FIFO space is available Note: This function is only available when Slave_mode = 0x1"]
    #[inline(always)]
    pub fn fifo_full_suspend(&self) -> FifoFullSuspendR {
        FifoFullSuspendR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:23 - Reserved"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> Rsvd2R {
        Rsvd2R::new(((self.bits >> 15) & 0x01ff) as u16)
    }
    #[doc = "Bit 24 - Write 1 to trigger a soft reset. All registers except the mutex register will be reset. Read the bit until 0 to determine if the soft reset is complete."]
    #[inline(always)]
    pub fn soft_reset_trigger_busy(&self) -> SoftResetTriggerBusyR {
        SoftResetTriggerBusyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:31 - Reserved"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 0x0: SHA-224 0x1: SHA-256 0x2: SHA-384 0x3: SHA-512"]
    #[inline(always)]
    pub fn sha2_mode(&mut self) -> Sha2ModeW<'_, ShaControlSpec> {
        Sha2ModeW::new(self, 0)
    }
    #[doc = "Bit 2 - 0x0: read-only mode 0x1: copy mode"]
    #[inline(always)]
    pub fn dma_mode(&mut self) -> DmaModeW<'_, ShaControlSpec> {
        DmaModeW::new(self, 2)
    }
    #[doc = "Bit 4 - Only available when config HMAC_MODE 0x0: use HASH mode. 0x1: use HMAC mode."]
    #[inline(always)]
    pub fn hmac_mode(&mut self) -> HmacModeW<'_, ShaControlSpec> {
        HmacModeW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable auto clock gating"]
    #[inline(always)]
    pub fn icg_enable(&mut self) -> IcgEnableW<'_, ShaControlSpec> {
        IcgEnableW::new(self, 5)
    }
    #[doc = "Bit 6 - 0: DMA mode. 1: slave mode. SW can push data by wrting FIFO Data Register in slave mode."]
    #[inline(always)]
    pub fn slave_mode(&mut self) -> SlaveModeW<'_, ShaControlSpec> {
        SlaveModeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Only available when config HMAC_MODE 0x0: 128 bits 0x1: 192 bits 0x2: 256 bits"]
    #[inline(always)]
    pub fn hmac_key_size(&mut self) -> HmacKeySizeW<'_, ShaControlSpec> {
        HmacKeySizeW::new(self, 8)
    }
    #[doc = "Bit 10 - Only available when config SUPPORT_TRUNCATED_SHA512 and SHA_mode = 0x3 (SHA-512) 0: normal SHA-512 1: truncated SHA-512"]
    #[inline(always)]
    pub fn sha512_truncate_en(&mut self) -> Sha512TruncateEnW<'_, ShaControlSpec> {
        Sha512TruncateEnW::new(self, 10)
    }
    #[doc = "Bit 11 - Only available when config SUPPORT_TRUNCATED_SHA512 and SHA512_truncate_en= 0x1 (truncated SHA-512) 0: SHA-512/224 mode 1: SHA-512/256 mode"]
    #[inline(always)]
    pub fn t_sha512_use_256(&mut self) -> TSha512Use256W<'_, ShaControlSpec> {
        TSha512Use256W::new(self, 11)
    }
    #[doc = "Bit 12 - 0x0: use native byte order in memory 0x1: inverse byte order of read data"]
    #[inline(always)]
    pub fn payload_byte_inversion_enable(
        &mut self,
    ) -> PayloadByteInversionEnableW<'_, ShaControlSpec> {
        PayloadByteInversionEnableW::new(self, 12)
    }
    #[doc = "Bit 13 - 0x0: use native byte order in memory 0x1: inverse byte order of read data"]
    #[inline(always)]
    pub fn key_byte_inversion_enable(&mut self) -> KeyByteInversionEnableW<'_, ShaControlSpec> {
        KeyByteInversionEnableW::new(self, 13)
    }
    #[doc = "Bit 14 - User writes data to SHA_FIFO_in but the FIFO space is full. 0x0: accept signal is always asserted 0x1: accept signal is not asserted until FIFO space is available Note: This function is only available when Slave_mode = 0x1"]
    #[inline(always)]
    pub fn fifo_full_suspend(&mut self) -> FifoFullSuspendW<'_, ShaControlSpec> {
        FifoFullSuspendW::new(self, 14)
    }
    #[doc = "Bit 24 - Write 1 to trigger a soft reset. All registers except the mutex register will be reset. Read the bit until 0 to determine if the soft reset is complete."]
    #[inline(always)]
    pub fn soft_reset_trigger_busy(&mut self) -> SoftResetTriggerBusyW<'_, ShaControlSpec> {
        SoftResetTriggerBusyW::new(self, 24)
    }
}
#[doc = "HMAC engine control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sha_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sha_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShaControlSpec;
impl crate::RegisterSpec for ShaControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sha_control::R`](R) reader structure"]
impl crate::Readable for ShaControlSpec {}
#[doc = "`write(|w| ..)` method takes [`sha_control::W`](W) writer structure"]
impl crate::Writable for ShaControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHA_Control to value 0x7020"]
impl crate::Resettable for ShaControlSpec {
    const RESET_VALUE: u32 = 0x7020;
}
