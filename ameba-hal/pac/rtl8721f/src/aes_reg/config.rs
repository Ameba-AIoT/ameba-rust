#[doc = "Register `config` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `config` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `engine_mode` reader - 0: Slave mode 1: DMA mode"]
pub type EngineModeR = crate::BitReader;
#[doc = "Field `engine_mode` writer - 0: Slave mode 1: DMA mode"]
pub type EngineModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cipher_mode` reader - 4'b0000: ECB mode 4'b0001: CBC mode 4'b0010: CFB mode 4'b0011: OFB mode 4'b0100: CTR mode 4'b0101: GMAC mode (only available when SUPPORT_GCM=1) 4'b0110: GCM mode (only available when SUPPORT_GCM=1) 4'b0111: CMAC mode 4'b1000: XTS mode (only available when SUPPORT_XTS=1)"]
pub type CipherModeR = crate::FieldReader;
#[doc = "Field `cipher_mode` writer - 4'b0000: ECB mode 4'b0001: CBC mode 4'b0010: CFB mode 4'b0011: OFB mode 4'b0100: CTR mode 4'b0101: GMAC mode (only available when SUPPORT_GCM=1) 4'b0110: GCM mode (only available when SUPPORT_GCM=1) 4'b0111: CMAC mode 4'b1000: XTS mode (only available when SUPPORT_XTS=1)"]
pub type CipherModeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `key_size` reader - 2'b00: 128 bits 2'b01: 192 bits 2'b10: 256 bits"]
pub type KeySizeR = crate::FieldReader;
#[doc = "Field `key_size` writer - 2'b00: 128 bits 2'b01: 192 bits 2'b10: 256 bits"]
pub type KeySizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `en_de` reader - 1'b0: decrypt 1'b1: encrypt"]
pub type EnDeR = crate::BitReader;
#[doc = "Field `en_de` writer - 1'b0: decrypt 1'b1: encrypt"]
pub type EnDeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `first_block` reader - If using dma mode, the usage of \"first block\" is the same as original (before this version). If using slave mode, set \"first_block\" to 1 is meaningless. that is, it's dont care.When using slave mode, the last step after setting all registers is to set cpu datain. After setting cpu datain (128 bits) in total, AES engine will start computing automatically.(recommend to set to 0 if using slave mode)"]
pub type FirstBlockR = crate::BitReader;
#[doc = "Field `first_block` writer - If using dma mode, the usage of \"first block\" is the same as original (before this version). If using slave mode, set \"first_block\" to 1 is meaningless. that is, it's dont care.When using slave mode, the last step after setting all registers is to set cpu datain. After setting cpu datain (128 bits) in total, AES engine will start computing automatically.(recommend to set to 0 if using slave mode)"]
pub type FirstBlockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `de_key_gen` reader - In general, the decryption key will be derived when decrypting in ECB or CBC mode. If the user has prepared the decryption key, can set this register to 0 to skip the cycles of derived. If 1, it will generate a key when decrypting. If 0, it will not generate the key, but instead uses the input to start decrypting."]
pub type DeKeyGenR = crate::BitReader;
#[doc = "Field `de_key_gen` writer - In general, the decryption key will be derived when decrypting in ECB or CBC mode. If the user has prepared the decryption key, can set this register to 0 to skip the cycles of derived. If 1, it will generate a key when decrypting. If 0, it will not generate the key, but instead uses the input to start decrypting."]
pub type DeKeyGenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wait_for_slave_en` reader - Pready can be low or always stay high Set to 1: pready will always be 1 Set to 0: pready could be 0 (wait states), default: 0 The difference is that, if set to 1, whenever cpu writes data to input fifo when input fifo is full/read output data from output fifo when output fifo is empty,pready will stay 1, and respond an error (pslverr asserted) immediately. If set to 0, when cpu writes data to input fifo when input fifo is full, however, if there is still space in output fifo, pready will stay 0 untill aes consumes input data from input fifo, that is, there are spaces for incoming input data now. then, pready is asserted, and there will be no error response.When cpu reads data from output fifo when output fifo is empty, however, if there is still available data in input fifo, pready will stay 0 untill aes push output data to output fifo, that is, there is output data waiting to be read now. then, pready is asserted, and there will be no error response.if cpu attempts to write data to input fifo when input is full and output fifo is also full/read data from output fifo when output data is empty and input fifo is also empty, pready will be asserted and respond an error."]
pub type WaitForSlaveEnR = crate::BitReader;
#[doc = "Field `wait_for_slave_en` writer - Pready can be low or always stay high Set to 1: pready will always be 1 Set to 0: pready could be 0 (wait states), default: 0 The difference is that, if set to 1, whenever cpu writes data to input fifo when input fifo is full/read output data from output fifo when output fifo is empty,pready will stay 1, and respond an error (pslverr asserted) immediately. If set to 0, when cpu writes data to input fifo when input fifo is full, however, if there is still space in output fifo, pready will stay 0 untill aes consumes input data from input fifo, that is, there are spaces for incoming input data now. then, pready is asserted, and there will be no error response.When cpu reads data from output fifo when output fifo is empty, however, if there is still available data in input fifo, pready will stay 0 untill aes push output data to output fifo, that is, there is output data waiting to be read now. then, pready is asserted, and there will be no error response.if cpu attempts to write data to input fifo when input is full and output fifo is also full/read data from output fifo when output data is empty and input fifo is also empty, pready will be asserted and respond an error."]
pub type WaitForSlaveEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sca_mask_en` reader - SCA mask enable. SW should also set the chn0_sca_mask registers when system initalization. 1: enable SCA protection 0: disable SCA protection"]
pub type ScaMaskEnR = crate::BitReader;
#[doc = "Field `sca_mask_en` writer - SCA mask enable. SW should also set the chn0_sca_mask registers when system initalization. 1: enable SCA protection 0: disable SCA protection"]
pub type ScaMaskEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `core_select` reader - Cipher core select. 1: Select SM4 as cipher core 0: Select AES as cipher core"]
pub type CoreSelectR = crate::BitReader;
#[doc = "Field `core_select` writer - Cipher core select. 1: Select SM4 as cipher core 0: Select AES as cipher core"]
pub type CoreSelectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `random_op_cycle` reader - Random operation cycle. Range from 0~3. 0: Not insert any dummy operation. 1~3: Insert specific cycle of dummy operation."]
pub type RandomOpCycleR = crate::FieldReader;
#[doc = "Field `random_op_mode` reader - Random operation insertion mode. 1: Dynamically insert random cycle of dummy operation in each AES round. 0: Insert fixed cycle (specify by random_op_cycle) of dummy operation in each AES round."]
pub type RandomOpModeR = crate::BitReader;
#[doc = "Field `gcm_last` reader - In some cases, user would need to use slave mode to compute gcm last when all other computations (GMAC) are computed in DMA mode (see Lalu AES GCM Mode). In such cases, user should not set cpu_datain, user should set engine mode to slave mode first and then set gcm_last to trigger AES engine. Note that user should clear this be to zero after calculation."]
pub type GcmLastR = crate::BitReader;
#[doc = "Field `gcm_last` writer - In some cases, user would need to use slave mode to compute gcm last when all other computations (GMAC) are computed in DMA mode (see Lalu AES GCM Mode). In such cases, user should not set cpu_datain, user should set engine mode to slave mode first and then set gcm_last to trigger AES engine. Note that user should clear this be to zero after calculation."]
pub type GcmLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmac_last` reader - Only used in CMAC mode. CMAC mode can be divided into several blocks to calculate. This bit controls whether the last block. If set to 1, it means that the transaction is the last time, and the last block will XOR the subkey."]
pub type CmacLastR = crate::BitReader;
#[doc = "Field `cmac_last` writer - Only used in CMAC mode. CMAC mode can be divided into several blocks to calculate. This bit controls whether the last block. If set to 1, it means that the transaction is the last time, and the last block will XOR the subkey."]
pub type CmacLastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cpu_fifo_status` reader - CPU FIFO status (slave mode FIFO status) {input fifo full (1b), output fifo empty (1b), input data write cnt (2b), output data read cnt (2b)}, for user to check input/output fifo status. When aes_intr asserted (error_cpu_fifo), user can check FIFO status."]
pub type CpuFifoStatusR = crate::FieldReader;
#[doc = "Field `soft_reset_trigger_busy` reader - Writting 1 to trigger a soft reset. All registers except the mutex register will be reset. Read the bit until 0 to determine if the soft reset is complete."]
pub type SoftResetTriggerBusyR = crate::BitReader;
#[doc = "Field `soft_reset_trigger_busy` writer - Writting 1 to trigger a soft reset. All registers except the mutex register will be reset. Read the bit until 0 to determine if the soft reset is complete."]
pub type SoftResetTriggerBusyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aes_idle` reader - When using slave mode to compute gmac and cmac, since the result is a tag, there would be no data in output fifo, therefore, we use this bit to indicate that gmac/cmac has finished (finished means: no input data in input fifo waited to be processed and aes engine has finished all computation)user can polling this bit to check whether aes engine is not running when using GMAC/CMAC."]
pub type AesIdleR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 0: Slave mode 1: DMA mode"]
    #[inline(always)]
    pub fn engine_mode(&self) -> EngineModeR {
        EngineModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - 4'b0000: ECB mode 4'b0001: CBC mode 4'b0010: CFB mode 4'b0011: OFB mode 4'b0100: CTR mode 4'b0101: GMAC mode (only available when SUPPORT_GCM=1) 4'b0110: GCM mode (only available when SUPPORT_GCM=1) 4'b0111: CMAC mode 4'b1000: XTS mode (only available when SUPPORT_XTS=1)"]
    #[inline(always)]
    pub fn cipher_mode(&self) -> CipherModeR {
        CipherModeR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 5:6 - 2'b00: 128 bits 2'b01: 192 bits 2'b10: 256 bits"]
    #[inline(always)]
    pub fn key_size(&self) -> KeySizeR {
        KeySizeR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 1'b0: decrypt 1'b1: encrypt"]
    #[inline(always)]
    pub fn en_de(&self) -> EnDeR {
        EnDeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If using dma mode, the usage of \"first block\" is the same as original (before this version). If using slave mode, set \"first_block\" to 1 is meaningless. that is, it's dont care.When using slave mode, the last step after setting all registers is to set cpu datain. After setting cpu datain (128 bits) in total, AES engine will start computing automatically.(recommend to set to 0 if using slave mode)"]
    #[inline(always)]
    pub fn first_block(&self) -> FirstBlockR {
        FirstBlockR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - In general, the decryption key will be derived when decrypting in ECB or CBC mode. If the user has prepared the decryption key, can set this register to 0 to skip the cycles of derived. If 1, it will generate a key when decrypting. If 0, it will not generate the key, but instead uses the input to start decrypting."]
    #[inline(always)]
    pub fn de_key_gen(&self) -> DeKeyGenR {
        DeKeyGenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Pready can be low or always stay high Set to 1: pready will always be 1 Set to 0: pready could be 0 (wait states), default: 0 The difference is that, if set to 1, whenever cpu writes data to input fifo when input fifo is full/read output data from output fifo when output fifo is empty,pready will stay 1, and respond an error (pslverr asserted) immediately. If set to 0, when cpu writes data to input fifo when input fifo is full, however, if there is still space in output fifo, pready will stay 0 untill aes consumes input data from input fifo, that is, there are spaces for incoming input data now. then, pready is asserted, and there will be no error response.When cpu reads data from output fifo when output fifo is empty, however, if there is still available data in input fifo, pready will stay 0 untill aes push output data to output fifo, that is, there is output data waiting to be read now. then, pready is asserted, and there will be no error response.if cpu attempts to write data to input fifo when input is full and output fifo is also full/read data from output fifo when output data is empty and input fifo is also empty, pready will be asserted and respond an error."]
    #[inline(always)]
    pub fn wait_for_slave_en(&self) -> WaitForSlaveEnR {
        WaitForSlaveEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SCA mask enable. SW should also set the chn0_sca_mask registers when system initalization. 1: enable SCA protection 0: disable SCA protection"]
    #[inline(always)]
    pub fn sca_mask_en(&self) -> ScaMaskEnR {
        ScaMaskEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Cipher core select. 1: Select SM4 as cipher core 0: Select AES as cipher core"]
    #[inline(always)]
    pub fn core_select(&self) -> CoreSelectR {
        CoreSelectR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - Random operation cycle. Range from 0~3. 0: Not insert any dummy operation. 1~3: Insert specific cycle of dummy operation."]
    #[inline(always)]
    pub fn random_op_cycle(&self) -> RandomOpCycleR {
        RandomOpCycleR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Random operation insertion mode. 1: Dynamically insert random cycle of dummy operation in each AES round. 0: Insert fixed cycle (specify by random_op_cycle) of dummy operation in each AES round."]
    #[inline(always)]
    pub fn random_op_mode(&self) -> RandomOpModeR {
        RandomOpModeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - In some cases, user would need to use slave mode to compute gcm last when all other computations (GMAC) are computed in DMA mode (see Lalu AES GCM Mode). In such cases, user should not set cpu_datain, user should set engine mode to slave mode first and then set gcm_last to trigger AES engine. Note that user should clear this be to zero after calculation."]
    #[inline(always)]
    pub fn gcm_last(&self) -> GcmLastR {
        GcmLastR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Only used in CMAC mode. CMAC mode can be divided into several blocks to calculate. This bit controls whether the last block. If set to 1, it means that the transaction is the last time, and the last block will XOR the subkey."]
    #[inline(always)]
    pub fn cmac_last(&self) -> CmacLastR {
        CmacLastR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:23 - CPU FIFO status (slave mode FIFO status) {input fifo full (1b), output fifo empty (1b), input data write cnt (2b), output data read cnt (2b)}, for user to check input/output fifo status. When aes_intr asserted (error_cpu_fifo), user can check FIFO status."]
    #[inline(always)]
    pub fn cpu_fifo_status(&self) -> CpuFifoStatusR {
        CpuFifoStatusR::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Writting 1 to trigger a soft reset. All registers except the mutex register will be reset. Read the bit until 0 to determine if the soft reset is complete."]
    #[inline(always)]
    pub fn soft_reset_trigger_busy(&self) -> SoftResetTriggerBusyR {
        SoftResetTriggerBusyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 31 - When using slave mode to compute gmac and cmac, since the result is a tag, there would be no data in output fifo, therefore, we use this bit to indicate that gmac/cmac has finished (finished means: no input data in input fifo waited to be processed and aes engine has finished all computation)user can polling this bit to check whether aes engine is not running when using GMAC/CMAC."]
    #[inline(always)]
    pub fn aes_idle(&self) -> AesIdleR {
        AesIdleR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: Slave mode 1: DMA mode"]
    #[inline(always)]
    pub fn engine_mode(&mut self) -> EngineModeW<'_, ConfigSpec> {
        EngineModeW::new(self, 0)
    }
    #[doc = "Bits 1:4 - 4'b0000: ECB mode 4'b0001: CBC mode 4'b0010: CFB mode 4'b0011: OFB mode 4'b0100: CTR mode 4'b0101: GMAC mode (only available when SUPPORT_GCM=1) 4'b0110: GCM mode (only available when SUPPORT_GCM=1) 4'b0111: CMAC mode 4'b1000: XTS mode (only available when SUPPORT_XTS=1)"]
    #[inline(always)]
    pub fn cipher_mode(&mut self) -> CipherModeW<'_, ConfigSpec> {
        CipherModeW::new(self, 1)
    }
    #[doc = "Bits 5:6 - 2'b00: 128 bits 2'b01: 192 bits 2'b10: 256 bits"]
    #[inline(always)]
    pub fn key_size(&mut self) -> KeySizeW<'_, ConfigSpec> {
        KeySizeW::new(self, 5)
    }
    #[doc = "Bit 7 - 1'b0: decrypt 1'b1: encrypt"]
    #[inline(always)]
    pub fn en_de(&mut self) -> EnDeW<'_, ConfigSpec> {
        EnDeW::new(self, 7)
    }
    #[doc = "Bit 8 - If using dma mode, the usage of \"first block\" is the same as original (before this version). If using slave mode, set \"first_block\" to 1 is meaningless. that is, it's dont care.When using slave mode, the last step after setting all registers is to set cpu datain. After setting cpu datain (128 bits) in total, AES engine will start computing automatically.(recommend to set to 0 if using slave mode)"]
    #[inline(always)]
    pub fn first_block(&mut self) -> FirstBlockW<'_, ConfigSpec> {
        FirstBlockW::new(self, 8)
    }
    #[doc = "Bit 9 - In general, the decryption key will be derived when decrypting in ECB or CBC mode. If the user has prepared the decryption key, can set this register to 0 to skip the cycles of derived. If 1, it will generate a key when decrypting. If 0, it will not generate the key, but instead uses the input to start decrypting."]
    #[inline(always)]
    pub fn de_key_gen(&mut self) -> DeKeyGenW<'_, ConfigSpec> {
        DeKeyGenW::new(self, 9)
    }
    #[doc = "Bit 10 - Pready can be low or always stay high Set to 1: pready will always be 1 Set to 0: pready could be 0 (wait states), default: 0 The difference is that, if set to 1, whenever cpu writes data to input fifo when input fifo is full/read output data from output fifo when output fifo is empty,pready will stay 1, and respond an error (pslverr asserted) immediately. If set to 0, when cpu writes data to input fifo when input fifo is full, however, if there is still space in output fifo, pready will stay 0 untill aes consumes input data from input fifo, that is, there are spaces for incoming input data now. then, pready is asserted, and there will be no error response.When cpu reads data from output fifo when output fifo is empty, however, if there is still available data in input fifo, pready will stay 0 untill aes push output data to output fifo, that is, there is output data waiting to be read now. then, pready is asserted, and there will be no error response.if cpu attempts to write data to input fifo when input is full and output fifo is also full/read data from output fifo when output data is empty and input fifo is also empty, pready will be asserted and respond an error."]
    #[inline(always)]
    pub fn wait_for_slave_en(&mut self) -> WaitForSlaveEnW<'_, ConfigSpec> {
        WaitForSlaveEnW::new(self, 10)
    }
    #[doc = "Bit 11 - SCA mask enable. SW should also set the chn0_sca_mask registers when system initalization. 1: enable SCA protection 0: disable SCA protection"]
    #[inline(always)]
    pub fn sca_mask_en(&mut self) -> ScaMaskEnW<'_, ConfigSpec> {
        ScaMaskEnW::new(self, 11)
    }
    #[doc = "Bit 12 - Cipher core select. 1: Select SM4 as cipher core 0: Select AES as cipher core"]
    #[inline(always)]
    pub fn core_select(&mut self) -> CoreSelectW<'_, ConfigSpec> {
        CoreSelectW::new(self, 12)
    }
    #[doc = "Bit 16 - In some cases, user would need to use slave mode to compute gcm last when all other computations (GMAC) are computed in DMA mode (see Lalu AES GCM Mode). In such cases, user should not set cpu_datain, user should set engine mode to slave mode first and then set gcm_last to trigger AES engine. Note that user should clear this be to zero after calculation."]
    #[inline(always)]
    pub fn gcm_last(&mut self) -> GcmLastW<'_, ConfigSpec> {
        GcmLastW::new(self, 16)
    }
    #[doc = "Bit 17 - Only used in CMAC mode. CMAC mode can be divided into several blocks to calculate. This bit controls whether the last block. If set to 1, it means that the transaction is the last time, and the last block will XOR the subkey."]
    #[inline(always)]
    pub fn cmac_last(&mut self) -> CmacLastW<'_, ConfigSpec> {
        CmacLastW::new(self, 17)
    }
    #[doc = "Bit 24 - Writting 1 to trigger a soft reset. All registers except the mutex register will be reset. Read the bit until 0 to determine if the soft reset is complete."]
    #[inline(always)]
    pub fn soft_reset_trigger_busy(&mut self) -> SoftResetTriggerBusyW<'_, ConfigSpec> {
        SoftResetTriggerBusyW::new(self, 24)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets config to value 0x8040_0000"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0x8040_0000;
}
