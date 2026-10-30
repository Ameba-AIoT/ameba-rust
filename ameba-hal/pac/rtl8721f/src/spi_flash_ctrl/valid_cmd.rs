#[doc = "Register `VALID_CMD` reader"]
pub type R = crate::R<ValidCmdSpec>;
#[doc = "Register `VALID_CMD` writer"]
pub type W = crate::W<ValidCmdSpec>;
#[doc = "Field `FRD_SINGLE` reader - Indicates fast read command is a valid command to execute. (known as (1-1-1))"]
pub type FrdSingleR = crate::BitReader;
#[doc = "Field `FRD_SINGLE` writer - Indicates fast read command is a valid command to execute. (known as (1-1-1))"]
pub type FrdSingleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_DUAL_I` reader - Indicates dual data read is a valid command to execute. (known as (1-1-2))"]
pub type RdDualIR = crate::BitReader;
#[doc = "Field `RD_DUAL_I` writer - Indicates dual data read is a valid command to execute. (known as (1-1-2))"]
pub type RdDualIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_DUAL_IO` reader - Indicates dual address/data read is a valid command to execute. (known as (1-2-2))"]
pub type RdDualIoR = crate::BitReader;
#[doc = "Field `RD_DUAL_IO` writer - Indicates dual address/data read is a valid command to execute. (known as (1-2-2))"]
pub type RdDualIoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_QUAD_O` reader - Indicates quad data read is a valid command to execute. (known as (1-1-4))"]
pub type RdQuadOR = crate::BitReader;
#[doc = "Field `RD_QUAD_O` writer - Indicates quad data read is a valid command to execute. (known as (1-1-4))"]
pub type RdQuadOW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD_QUAD_IO` reader - Indicates quad address/data read is a valid command to execute. (known as (1-4-4))"]
pub type RdQuadIoR = crate::BitReader;
#[doc = "Field `RD_QUAD_IO` writer - Indicates quad address/data read is a valid command to execute. (known as (1-4-4))"]
pub type RdQuadIoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_DUAL_I` reader - Indicates dual data write is a valid command to execute. (known as (1-1-2))"]
pub type WrDualIR = crate::BitReader;
#[doc = "Field `WR_DUAL_I` writer - Indicates dual data write is a valid command to execute. (known as (1-1-2))"]
pub type WrDualIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_DUAL_II` reader - Indicates dual address/data write is a valid command to execute. (known as (1-2-2))"]
pub type WrDualIiR = crate::BitReader;
#[doc = "Field `WR_DUAL_II` writer - Indicates dual address/data write is a valid command to execute. (known as (1-2-2))"]
pub type WrDualIiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_QUAD_I` reader - Indicates quad data write is a valid command to execute. (known as (1-1-4))"]
pub type WrQuadIR = crate::BitReader;
#[doc = "Field `WR_QUAD_I` writer - Indicates quad data write is a valid command to execute. (known as (1-1-4))"]
pub type WrQuadIW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_QUAD_II` reader - Indicates quad address/data write is a valid command to execute. (known as (1-4-4))"]
pub type WrQuadIiR = crate::BitReader;
#[doc = "Field `WR_QUAD_II` writer - Indicates quad address/data write is a valid command to execute. (known as (1-4-4))"]
pub type WrQuadIiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RM_RDSR` reader - Remove read status register in auto write."]
pub type RmRdsrR = crate::BitReader;
#[doc = "Field `RM_RDSR` writer - Remove read status register in auto write."]
pub type RmRdsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RM_WEN` reader - Remove write enable command in auto write."]
pub type RmWenR = crate::BitReader;
#[doc = "Field `RM_WEN` writer - Remove write enable command in auto write."]
pub type RmWenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRM_EN` reader - Set to enable SPIC performance read mode in Auto Mode. RXI-312 SPIC will auto exit performance read mode before auto write or enter to user mode. Enter PRM Flow: 1. Set CR_VALID_CMD\\[11\\] (PRM_EN) with CR_VALID_CMD\\[4\\] (RD_QUAD_IO) or with (CR_VALID_CMD\\[0\\] and CR_VALID_CMD\\[12\\]) 2. Set CR_READ_QUAD_ADDR_DATA\\[23:16\\] (PRM_VALUE) and \\[7:0\\] (RD_QUAD_IO_CMD) 3. Set correct Dummy Cycle, and Valid Command then access in Auto Mode address range If PRM_EN, at 1st time Auto Rd, it will access with CMD, and store this condition. At 2nd Auto Rd or later, it will access without CMD but with Address and Mode Value. Our SPIC will check PRM_VALUE (CR_READ_QUAD_ADDR_DATA \\[23:16\\]), if PRM_VALUE ==00 (not a valid PRM value), and PRM_EN is set, SPIC will always access SPI Flash with (CMD+ADDR+MODE+DUMMY+...) format."]
pub type PrmEnR = crate::BitReader;
#[doc = "Field `PRM_EN` writer - Set to enable SPIC performance read mode in Auto Mode. RXI-312 SPIC will auto exit performance read mode before auto write or enter to user mode. Enter PRM Flow: 1. Set CR_VALID_CMD\\[11\\] (PRM_EN) with CR_VALID_CMD\\[4\\] (RD_QUAD_IO) or with (CR_VALID_CMD\\[0\\] and CR_VALID_CMD\\[12\\]) 2. Set CR_READ_QUAD_ADDR_DATA\\[23:16\\] (PRM_VALUE) and \\[7:0\\] (RD_QUAD_IO_CMD) 3. Set correct Dummy Cycle, and Valid Command then access in Auto Mode address range If PRM_EN, at 1st time Auto Rd, it will access with CMD, and store this condition. At 2nd Auto Rd or later, it will access without CMD but with Address and Mode Value. Our SPIC will check PRM_VALUE (CR_READ_QUAD_ADDR_DATA \\[23:16\\]), if PRM_VALUE ==00 (not a valid PRM value), and PRM_EN is set, SPIC will always access SPI Flash with (CMD+ADDR+MODE+DUMMY+...) format."]
pub type PrmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTRLR0_CH` reader - Set (1) to use CTRLR0 CMD_CH/DATA_CH/ADDR_CH and DDR_EN field in Auto mode; Otherwise, SPIC will decode according to which VALID_CMD you choose in auto mode. Suggestion while use this bit field: Read Flash mode: (1-4-8), (4-4-8), (1-4D-8D), (4-4D-8D) with using RD_QUAD_IO\\[7:0\\] Read Flash mode (with 2-Byte CMD): (1-8-8), (8-8-8), (8D-8D-8D), (4D-4D-4D) with using RD_FAST_SINGLE_IO\\[15:0\\] (always using FBAUD) Write Flash mode: (1-4-8), (4-4-8) with using WR_QUAD_IO\\[7:0\\] Write Flash mode (with 2-Byte CMD): (1-8-8), (8-8-8), (8D-8D-8D) with using WR_SINGLE_IO\\[15:0\\]"]
pub type Ctrlr0ChR = crate::BitReader;
#[doc = "Field `CTRLR0_CH` writer - Set (1) to use CTRLR0 CMD_CH/DATA_CH/ADDR_CH and DDR_EN field in Auto mode; Otherwise, SPIC will decode according to which VALID_CMD you choose in auto mode. Suggestion while use this bit field: Read Flash mode: (1-4-8), (4-4-8), (1-4D-8D), (4-4D-8D) with using RD_QUAD_IO\\[7:0\\] Read Flash mode (with 2-Byte CMD): (1-8-8), (8-8-8), (8D-8D-8D), (4D-4D-4D) with using RD_FAST_SINGLE_IO\\[15:0\\] (always using FBAUD) Write Flash mode: (1-4-8), (4-4-8) with using WR_QUAD_IO\\[7:0\\] Write Flash mode (with 2-Byte CMD): (1-8-8), (8-8-8), (8D-8D-8D) with using WR_SINGLE_IO\\[15:0\\]"]
pub type Ctrlr0ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DUM_EN` reader - Enable to push one dummy byte (DUM_BYTE_VAL) after pushing address to Flash in auto read. If PRM_EN is active, it will push PRM_Value instead of DUM_BYTE_VAL. If RD_DUMMY_LENGTH = 0, it won't push the dummy byte."]
pub type DumEnR = crate::BitReader;
#[doc = "Field `DUM_EN` writer - Enable to push one dummy byte (DUM_BYTE_VAL) after pushing address to Flash in auto read. If PRM_EN is active, it will push PRM_Value instead of DUM_BYTE_VAL. If RD_DUMMY_LENGTH = 0, it won't push the dummy byte."]
pub type DumEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEQ_TRANS_DIS` reader - Set (1) to disable sequential read and sequential write function in auto mode. If two bus read/write transactions are sequential (address is consecutive), SPIC can access second transaction data without sending CMD/ADDR/DUMMY. Once disable (1) this bit field, spi_csn will inactive immediately and also disable this function."]
pub type SeqTransDisR = crate::BitReader;
#[doc = "Field `SEQ_TRANS_DIS` writer - Set (1) to disable sequential read and sequential write function in auto mode. If two bus read/write transactions are sequential (address is consecutive), SPIC can access second transaction data without sending CMD/ADDR/DUMMY. Once disable (1) this bit field, spi_csn will inactive immediately and also disable this function."]
pub type SeqTransDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_PUIN` reader - Set (1) to pull in first address to negedge of spi_clk. (negedge of last cmd) Set (0) indicate first address start at next posedge of last cmd phase. Used when DDR address and need to pull in address. Exists: TAG >= 2.0.6."]
pub type AddrPuinR = crate::BitReader;
#[doc = "Field `ADDR_PUIN` writer - Set (1) to pull in first address to negedge of spi_clk. (negedge of last cmd) Set (0) indicate first address start at next posedge of last cmd phase. Used when DDR address and need to pull in address. Exists: TAG >= 2.0.6."]
pub type AddrPuinW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates fast read command is a valid command to execute. (known as (1-1-1))"]
    #[inline(always)]
    pub fn frd_single(&self) -> FrdSingleR {
        FrdSingleR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates dual data read is a valid command to execute. (known as (1-1-2))"]
    #[inline(always)]
    pub fn rd_dual_i(&self) -> RdDualIR {
        RdDualIR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates dual address/data read is a valid command to execute. (known as (1-2-2))"]
    #[inline(always)]
    pub fn rd_dual_io(&self) -> RdDualIoR {
        RdDualIoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates quad data read is a valid command to execute. (known as (1-1-4))"]
    #[inline(always)]
    pub fn rd_quad_o(&self) -> RdQuadOR {
        RdQuadOR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates quad address/data read is a valid command to execute. (known as (1-4-4))"]
    #[inline(always)]
    pub fn rd_quad_io(&self) -> RdQuadIoR {
        RdQuadIoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates dual data write is a valid command to execute. (known as (1-1-2))"]
    #[inline(always)]
    pub fn wr_dual_i(&self) -> WrDualIR {
        WrDualIR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates dual address/data write is a valid command to execute. (known as (1-2-2))"]
    #[inline(always)]
    pub fn wr_dual_ii(&self) -> WrDualIiR {
        WrDualIiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates quad data write is a valid command to execute. (known as (1-1-4))"]
    #[inline(always)]
    pub fn wr_quad_i(&self) -> WrQuadIR {
        WrQuadIR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates quad address/data write is a valid command to execute. (known as (1-4-4))"]
    #[inline(always)]
    pub fn wr_quad_ii(&self) -> WrQuadIiR {
        WrQuadIiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remove read status register in auto write."]
    #[inline(always)]
    pub fn rm_rdsr(&self) -> RmRdsrR {
        RmRdsrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Remove write enable command in auto write."]
    #[inline(always)]
    pub fn rm_wen(&self) -> RmWenR {
        RmWenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set to enable SPIC performance read mode in Auto Mode. RXI-312 SPIC will auto exit performance read mode before auto write or enter to user mode. Enter PRM Flow: 1. Set CR_VALID_CMD\\[11\\] (PRM_EN) with CR_VALID_CMD\\[4\\] (RD_QUAD_IO) or with (CR_VALID_CMD\\[0\\] and CR_VALID_CMD\\[12\\]) 2. Set CR_READ_QUAD_ADDR_DATA\\[23:16\\] (PRM_VALUE) and \\[7:0\\] (RD_QUAD_IO_CMD) 3. Set correct Dummy Cycle, and Valid Command then access in Auto Mode address range If PRM_EN, at 1st time Auto Rd, it will access with CMD, and store this condition. At 2nd Auto Rd or later, it will access without CMD but with Address and Mode Value. Our SPIC will check PRM_VALUE (CR_READ_QUAD_ADDR_DATA \\[23:16\\]), if PRM_VALUE ==00 (not a valid PRM value), and PRM_EN is set, SPIC will always access SPI Flash with (CMD+ADDR+MODE+DUMMY+...) format."]
    #[inline(always)]
    pub fn prm_en(&self) -> PrmEnR {
        PrmEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set (1) to use CTRLR0 CMD_CH/DATA_CH/ADDR_CH and DDR_EN field in Auto mode; Otherwise, SPIC will decode according to which VALID_CMD you choose in auto mode. Suggestion while use this bit field: Read Flash mode: (1-4-8), (4-4-8), (1-4D-8D), (4-4D-8D) with using RD_QUAD_IO\\[7:0\\] Read Flash mode (with 2-Byte CMD): (1-8-8), (8-8-8), (8D-8D-8D), (4D-4D-4D) with using RD_FAST_SINGLE_IO\\[15:0\\] (always using FBAUD) Write Flash mode: (1-4-8), (4-4-8) with using WR_QUAD_IO\\[7:0\\] Write Flash mode (with 2-Byte CMD): (1-8-8), (8-8-8), (8D-8D-8D) with using WR_SINGLE_IO\\[15:0\\]"]
    #[inline(always)]
    pub fn ctrlr0_ch(&self) -> Ctrlr0ChR {
        Ctrlr0ChR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable to push one dummy byte (DUM_BYTE_VAL) after pushing address to Flash in auto read. If PRM_EN is active, it will push PRM_Value instead of DUM_BYTE_VAL. If RD_DUMMY_LENGTH = 0, it won't push the dummy byte."]
    #[inline(always)]
    pub fn dum_en(&self) -> DumEnR {
        DumEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Set (1) to disable sequential read and sequential write function in auto mode. If two bus read/write transactions are sequential (address is consecutive), SPIC can access second transaction data without sending CMD/ADDR/DUMMY. Once disable (1) this bit field, spi_csn will inactive immediately and also disable this function."]
    #[inline(always)]
    pub fn seq_trans_dis(&self) -> SeqTransDisR {
        SeqTransDisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set (1) to pull in first address to negedge of spi_clk. (negedge of last cmd) Set (0) indicate first address start at next posedge of last cmd phase. Used when DDR address and need to pull in address. Exists: TAG >= 2.0.6."]
    #[inline(always)]
    pub fn addr_puin(&self) -> AddrPuinR {
        AddrPuinR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates fast read command is a valid command to execute. (known as (1-1-1))"]
    #[inline(always)]
    pub fn frd_single(&mut self) -> FrdSingleW<'_, ValidCmdSpec> {
        FrdSingleW::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates dual data read is a valid command to execute. (known as (1-1-2))"]
    #[inline(always)]
    pub fn rd_dual_i(&mut self) -> RdDualIW<'_, ValidCmdSpec> {
        RdDualIW::new(self, 1)
    }
    #[doc = "Bit 2 - Indicates dual address/data read is a valid command to execute. (known as (1-2-2))"]
    #[inline(always)]
    pub fn rd_dual_io(&mut self) -> RdDualIoW<'_, ValidCmdSpec> {
        RdDualIoW::new(self, 2)
    }
    #[doc = "Bit 3 - Indicates quad data read is a valid command to execute. (known as (1-1-4))"]
    #[inline(always)]
    pub fn rd_quad_o(&mut self) -> RdQuadOW<'_, ValidCmdSpec> {
        RdQuadOW::new(self, 3)
    }
    #[doc = "Bit 4 - Indicates quad address/data read is a valid command to execute. (known as (1-4-4))"]
    #[inline(always)]
    pub fn rd_quad_io(&mut self) -> RdQuadIoW<'_, ValidCmdSpec> {
        RdQuadIoW::new(self, 4)
    }
    #[doc = "Bit 5 - Indicates dual data write is a valid command to execute. (known as (1-1-2))"]
    #[inline(always)]
    pub fn wr_dual_i(&mut self) -> WrDualIW<'_, ValidCmdSpec> {
        WrDualIW::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates dual address/data write is a valid command to execute. (known as (1-2-2))"]
    #[inline(always)]
    pub fn wr_dual_ii(&mut self) -> WrDualIiW<'_, ValidCmdSpec> {
        WrDualIiW::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates quad data write is a valid command to execute. (known as (1-1-4))"]
    #[inline(always)]
    pub fn wr_quad_i(&mut self) -> WrQuadIW<'_, ValidCmdSpec> {
        WrQuadIW::new(self, 7)
    }
    #[doc = "Bit 8 - Indicates quad address/data write is a valid command to execute. (known as (1-4-4))"]
    #[inline(always)]
    pub fn wr_quad_ii(&mut self) -> WrQuadIiW<'_, ValidCmdSpec> {
        WrQuadIiW::new(self, 8)
    }
    #[doc = "Bit 9 - Remove read status register in auto write."]
    #[inline(always)]
    pub fn rm_rdsr(&mut self) -> RmRdsrW<'_, ValidCmdSpec> {
        RmRdsrW::new(self, 9)
    }
    #[doc = "Bit 10 - Remove write enable command in auto write."]
    #[inline(always)]
    pub fn rm_wen(&mut self) -> RmWenW<'_, ValidCmdSpec> {
        RmWenW::new(self, 10)
    }
    #[doc = "Bit 11 - Set to enable SPIC performance read mode in Auto Mode. RXI-312 SPIC will auto exit performance read mode before auto write or enter to user mode. Enter PRM Flow: 1. Set CR_VALID_CMD\\[11\\] (PRM_EN) with CR_VALID_CMD\\[4\\] (RD_QUAD_IO) or with (CR_VALID_CMD\\[0\\] and CR_VALID_CMD\\[12\\]) 2. Set CR_READ_QUAD_ADDR_DATA\\[23:16\\] (PRM_VALUE) and \\[7:0\\] (RD_QUAD_IO_CMD) 3. Set correct Dummy Cycle, and Valid Command then access in Auto Mode address range If PRM_EN, at 1st time Auto Rd, it will access with CMD, and store this condition. At 2nd Auto Rd or later, it will access without CMD but with Address and Mode Value. Our SPIC will check PRM_VALUE (CR_READ_QUAD_ADDR_DATA \\[23:16\\]), if PRM_VALUE ==00 (not a valid PRM value), and PRM_EN is set, SPIC will always access SPI Flash with (CMD+ADDR+MODE+DUMMY+...) format."]
    #[inline(always)]
    pub fn prm_en(&mut self) -> PrmEnW<'_, ValidCmdSpec> {
        PrmEnW::new(self, 11)
    }
    #[doc = "Bit 12 - Set (1) to use CTRLR0 CMD_CH/DATA_CH/ADDR_CH and DDR_EN field in Auto mode; Otherwise, SPIC will decode according to which VALID_CMD you choose in auto mode. Suggestion while use this bit field: Read Flash mode: (1-4-8), (4-4-8), (1-4D-8D), (4-4D-8D) with using RD_QUAD_IO\\[7:0\\] Read Flash mode (with 2-Byte CMD): (1-8-8), (8-8-8), (8D-8D-8D), (4D-4D-4D) with using RD_FAST_SINGLE_IO\\[15:0\\] (always using FBAUD) Write Flash mode: (1-4-8), (4-4-8) with using WR_QUAD_IO\\[7:0\\] Write Flash mode (with 2-Byte CMD): (1-8-8), (8-8-8), (8D-8D-8D) with using WR_SINGLE_IO\\[15:0\\]"]
    #[inline(always)]
    pub fn ctrlr0_ch(&mut self) -> Ctrlr0ChW<'_, ValidCmdSpec> {
        Ctrlr0ChW::new(self, 12)
    }
    #[doc = "Bit 13 - Enable to push one dummy byte (DUM_BYTE_VAL) after pushing address to Flash in auto read. If PRM_EN is active, it will push PRM_Value instead of DUM_BYTE_VAL. If RD_DUMMY_LENGTH = 0, it won't push the dummy byte."]
    #[inline(always)]
    pub fn dum_en(&mut self) -> DumEnW<'_, ValidCmdSpec> {
        DumEnW::new(self, 13)
    }
    #[doc = "Bit 16 - Set (1) to disable sequential read and sequential write function in auto mode. If two bus read/write transactions are sequential (address is consecutive), SPIC can access second transaction data without sending CMD/ADDR/DUMMY. Once disable (1) this bit field, spi_csn will inactive immediately and also disable this function."]
    #[inline(always)]
    pub fn seq_trans_dis(&mut self) -> SeqTransDisW<'_, ValidCmdSpec> {
        SeqTransDisW::new(self, 16)
    }
    #[doc = "Bit 17 - Set (1) to pull in first address to negedge of spi_clk. (negedge of last cmd) Set (0) indicate first address start at next posedge of last cmd phase. Used when DDR address and need to pull in address. Exists: TAG >= 2.0.6."]
    #[inline(always)]
    pub fn addr_puin(&mut self) -> AddrPuinW<'_, ValidCmdSpec> {
        AddrPuinW::new(self, 17)
    }
}
#[doc = "Valid Command Register\n\nYou can [`read`](crate::Reg::read) this register and get [`valid_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`valid_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ValidCmdSpec;
impl crate::RegisterSpec for ValidCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`valid_cmd::R`](R) reader structure"]
impl crate::Readable for ValidCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`valid_cmd::W`](W) writer structure"]
impl crate::Writable for ValidCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VALID_CMD to value 0"]
impl crate::Resettable for ValidCmdSpec {}
