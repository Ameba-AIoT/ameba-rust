#[doc = "Register `DCX_CTRL` reader"]
pub type R = crate::R<DcxCtrlSpec>;
#[doc = "Register `DCX_CTRL` writer"]
pub type W = crate::W<DcxCtrlSpec>;
#[doc = "Field `CMD_HEAD_EN` reader - 0: without header in cmd phase 1: insert header in cmd phase"]
pub type CmdHeadEnR = crate::BitReader;
#[doc = "Field `CMD_HEAD_EN` writer - 0: without header in cmd phase 1: insert header in cmd phase"]
pub type CmdHeadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_TAIL_EN` reader - 0: without tail in cmd phase 1: insert tail in cmd phase"]
pub type CmdTailEnR = crate::BitReader;
#[doc = "Field `CMD_TAIL_EN` writer - 0: without tail in cmd phase 1: insert tail in cmd phase"]
pub type CmdTailEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_CTRL_BIT` reader - DCX control bit for cmd phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
pub type CmdCtrlBitR = crate::FieldReader;
#[doc = "Field `CMD_CTRL_BIT` writer - DCX control bit for cmd phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
pub type CmdCtrlBitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CMD_CTRL` reader - This field can only be active when CMD_CTRL_BIT is not 0. 0: only one DCX control bit in cmd phase; 1: DCX control bit for each byte in cmd phase; 2: DCX control bit for every 2-byte in cmd phase; 3: DCX control bit for every 4-byte in cmd phase; 4: DCX control bit for every 8-byte in cmd phase; 5: DCX control bit for every 16-byte in cmd phase;"]
pub type CmdCtrlR = crate::FieldReader;
#[doc = "Field `CMD_CTRL` writer - This field can only be active when CMD_CTRL_BIT is not 0. 0: only one DCX control bit in cmd phase; 1: DCX control bit for each byte in cmd phase; 2: DCX control bit for every 2-byte in cmd phase; 3: DCX control bit for every 4-byte in cmd phase; 4: DCX control bit for every 8-byte in cmd phase; 5: DCX control bit for every 16-byte in cmd phase;"]
pub type CmdCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADDR_HEAD_EN` reader - 0: without header in address phase 1: insert header in address phase"]
pub type AddrHeadEnR = crate::BitReader;
#[doc = "Field `ADDR_HEAD_EN` writer - 0: without header in address phase 1: insert header in address phase"]
pub type AddrHeadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_TAIL_EN` reader - 0: without tail in address phase 1: insert tail in address phase"]
pub type AddrTailEnR = crate::BitReader;
#[doc = "Field `ADDR_TAIL_EN` writer - 0: without tail in address phase 1: insert tail in address phase"]
pub type AddrTailEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDR_CTRL_BIT` reader - DCX control bit for addr phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
pub type AddrCtrlBitR = crate::FieldReader;
#[doc = "Field `ADDR_CTRL_BIT` writer - DCX control bit for addr phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
pub type AddrCtrlBitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADDR_CTRL` reader - This field can only be active when ADDR_CTRL_BIT is not 0. 0: only one DCX control bit in addr phase; 1: DCX control bit for each byte in addr phase; 2: DCX control bit for every 2-byte in addr phase; 3: DCX control bit for every 4-byte in addr phase; 4: DCX control bit for every 8-byte in addr phase; 5: DCX control bit for every 16-byte in addr phase;"]
pub type AddrCtrlR = crate::FieldReader;
#[doc = "Field `ADDR_CTRL` writer - This field can only be active when ADDR_CTRL_BIT is not 0. 0: only one DCX control bit in addr phase; 1: DCX control bit for each byte in addr phase; 2: DCX control bit for every 2-byte in addr phase; 3: DCX control bit for every 4-byte in addr phase; 4: DCX control bit for every 8-byte in addr phase; 5: DCX control bit for every 16-byte in addr phase;"]
pub type AddrCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DATA_HEAD_EN` reader - 0: without header in data phase 1: insert header in data phase"]
pub type DataHeadEnR = crate::BitReader;
#[doc = "Field `DATA_HEAD_EN` writer - 0: without header in data phase 1: insert header in data phase"]
pub type DataHeadEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_TAIL_EN` reader - 0: without tail in data phase 1: insert tail in data phase"]
pub type DataTailEnR = crate::BitReader;
#[doc = "Field `DATA_TAIL_EN` writer - 0: without tail in data phase 1: insert tail in data phase"]
pub type DataTailEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATA_CTRL_BIT` reader - DCX control bit for data phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
pub type DataCtrlBitR = crate::FieldReader;
#[doc = "Field `DATA_CTRL_BIT` writer - DCX control bit for data phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
pub type DataCtrlBitW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DATA_CTRL` reader - This field can only be active when DATA_CTRL_BIT is not 0. 0: only one DCX control bit in data phase; 1: DCX control bit for each byte in data phase; 2: DCX control bit for every 2-byte in data phase; 3: DCX control bit for every 4-byte in data phase; 4: DCX control bit for every 8-byte in data phase; 5: DCX control bit for every 16-byte in data phase;"]
pub type DataCtrlR = crate::FieldReader;
#[doc = "Field `DATA_CTRL` writer - This field can only be active when DATA_CTRL_BIT is not 0. 0: only one DCX control bit in data phase; 1: DCX control bit for each byte in data phase; 2: DCX control bit for every 2-byte in data phase; 3: DCX control bit for every 4-byte in data phase; 4: DCX control bit for every 8-byte in data phase; 5: DCX control bit for every 16-byte in data phase;"]
pub type DataCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - 0: without header in cmd phase 1: insert header in cmd phase"]
    #[inline(always)]
    pub fn cmd_head_en(&self) -> CmdHeadEnR {
        CmdHeadEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: without tail in cmd phase 1: insert tail in cmd phase"]
    #[inline(always)]
    pub fn cmd_tail_en(&self) -> CmdTailEnR {
        CmdTailEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - DCX control bit for cmd phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
    #[inline(always)]
    pub fn cmd_ctrl_bit(&self) -> CmdCtrlBitR {
        CmdCtrlBitR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:7 - This field can only be active when CMD_CTRL_BIT is not 0. 0: only one DCX control bit in cmd phase; 1: DCX control bit for each byte in cmd phase; 2: DCX control bit for every 2-byte in cmd phase; 3: DCX control bit for every 4-byte in cmd phase; 4: DCX control bit for every 8-byte in cmd phase; 5: DCX control bit for every 16-byte in cmd phase;"]
    #[inline(always)]
    pub fn cmd_ctrl(&self) -> CmdCtrlR {
        CmdCtrlR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - 0: without header in address phase 1: insert header in address phase"]
    #[inline(always)]
    pub fn addr_head_en(&self) -> AddrHeadEnR {
        AddrHeadEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: without tail in address phase 1: insert tail in address phase"]
    #[inline(always)]
    pub fn addr_tail_en(&self) -> AddrTailEnR {
        AddrTailEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - DCX control bit for addr phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
    #[inline(always)]
    pub fn addr_ctrl_bit(&self) -> AddrCtrlBitR {
        AddrCtrlBitR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - This field can only be active when ADDR_CTRL_BIT is not 0. 0: only one DCX control bit in addr phase; 1: DCX control bit for each byte in addr phase; 2: DCX control bit for every 2-byte in addr phase; 3: DCX control bit for every 4-byte in addr phase; 4: DCX control bit for every 8-byte in addr phase; 5: DCX control bit for every 16-byte in addr phase;"]
    #[inline(always)]
    pub fn addr_ctrl(&self) -> AddrCtrlR {
        AddrCtrlR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 16 - 0: without header in data phase 1: insert header in data phase"]
    #[inline(always)]
    pub fn data_head_en(&self) -> DataHeadEnR {
        DataHeadEnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 0: without tail in data phase 1: insert tail in data phase"]
    #[inline(always)]
    pub fn data_tail_en(&self) -> DataTailEnR {
        DataTailEnR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - DCX control bit for data phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
    #[inline(always)]
    pub fn data_ctrl_bit(&self) -> DataCtrlBitR {
        DataCtrlBitR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - This field can only be active when DATA_CTRL_BIT is not 0. 0: only one DCX control bit in data phase; 1: DCX control bit for each byte in data phase; 2: DCX control bit for every 2-byte in data phase; 3: DCX control bit for every 4-byte in data phase; 4: DCX control bit for every 8-byte in data phase; 5: DCX control bit for every 16-byte in data phase;"]
    #[inline(always)]
    pub fn data_ctrl(&self) -> DataCtrlR {
        DataCtrlR::new(((self.bits >> 21) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: without header in cmd phase 1: insert header in cmd phase"]
    #[inline(always)]
    pub fn cmd_head_en(&mut self) -> CmdHeadEnW<'_, DcxCtrlSpec> {
        CmdHeadEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 0: without tail in cmd phase 1: insert tail in cmd phase"]
    #[inline(always)]
    pub fn cmd_tail_en(&mut self) -> CmdTailEnW<'_, DcxCtrlSpec> {
        CmdTailEnW::new(self, 1)
    }
    #[doc = "Bits 2:4 - DCX control bit for cmd phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
    #[inline(always)]
    pub fn cmd_ctrl_bit(&mut self) -> CmdCtrlBitW<'_, DcxCtrlSpec> {
        CmdCtrlBitW::new(self, 2)
    }
    #[doc = "Bits 5:7 - This field can only be active when CMD_CTRL_BIT is not 0. 0: only one DCX control bit in cmd phase; 1: DCX control bit for each byte in cmd phase; 2: DCX control bit for every 2-byte in cmd phase; 3: DCX control bit for every 4-byte in cmd phase; 4: DCX control bit for every 8-byte in cmd phase; 5: DCX control bit for every 16-byte in cmd phase;"]
    #[inline(always)]
    pub fn cmd_ctrl(&mut self) -> CmdCtrlW<'_, DcxCtrlSpec> {
        CmdCtrlW::new(self, 5)
    }
    #[doc = "Bit 8 - 0: without header in address phase 1: insert header in address phase"]
    #[inline(always)]
    pub fn addr_head_en(&mut self) -> AddrHeadEnW<'_, DcxCtrlSpec> {
        AddrHeadEnW::new(self, 8)
    }
    #[doc = "Bit 9 - 0: without tail in address phase 1: insert tail in address phase"]
    #[inline(always)]
    pub fn addr_tail_en(&mut self) -> AddrTailEnW<'_, DcxCtrlSpec> {
        AddrTailEnW::new(self, 9)
    }
    #[doc = "Bits 10:12 - DCX control bit for addr phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
    #[inline(always)]
    pub fn addr_ctrl_bit(&mut self) -> AddrCtrlBitW<'_, DcxCtrlSpec> {
        AddrCtrlBitW::new(self, 10)
    }
    #[doc = "Bits 13:15 - This field can only be active when ADDR_CTRL_BIT is not 0. 0: only one DCX control bit in addr phase; 1: DCX control bit for each byte in addr phase; 2: DCX control bit for every 2-byte in addr phase; 3: DCX control bit for every 4-byte in addr phase; 4: DCX control bit for every 8-byte in addr phase; 5: DCX control bit for every 16-byte in addr phase;"]
    #[inline(always)]
    pub fn addr_ctrl(&mut self) -> AddrCtrlW<'_, DcxCtrlSpec> {
        AddrCtrlW::new(self, 13)
    }
    #[doc = "Bit 16 - 0: without header in data phase 1: insert header in data phase"]
    #[inline(always)]
    pub fn data_head_en(&mut self) -> DataHeadEnW<'_, DcxCtrlSpec> {
        DataHeadEnW::new(self, 16)
    }
    #[doc = "Bit 17 - 0: without tail in data phase 1: insert tail in data phase"]
    #[inline(always)]
    pub fn data_tail_en(&mut self) -> DataTailEnW<'_, DcxCtrlSpec> {
        DataTailEnW::new(self, 17)
    }
    #[doc = "Bits 18:20 - DCX control bit for data phase. 0: independent DCX signal 1: DCX control bit = 1-bit; 2: DCX control bit = 2 bit; 3: DCX control bit = 4-bit; 4: DCX control bit = 8-bit; 5: DCX control bit = 16-bit;"]
    #[inline(always)]
    pub fn data_ctrl_bit(&mut self) -> DataCtrlBitW<'_, DcxCtrlSpec> {
        DataCtrlBitW::new(self, 18)
    }
    #[doc = "Bits 21:23 - This field can only be active when DATA_CTRL_BIT is not 0. 0: only one DCX control bit in data phase; 1: DCX control bit for each byte in data phase; 2: DCX control bit for every 2-byte in data phase; 3: DCX control bit for every 4-byte in data phase; 4: DCX control bit for every 8-byte in data phase; 5: DCX control bit for every 16-byte in data phase;"]
    #[inline(always)]
    pub fn data_ctrl(&mut self) -> DataCtrlW<'_, DcxCtrlSpec> {
        DataCtrlW::new(self, 21)
    }
}
#[doc = "LCDC DCX control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcx_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcx_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcxCtrlSpec;
impl crate::RegisterSpec for DcxCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcx_ctrl::R`](R) reader structure"]
impl crate::Readable for DcxCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcx_ctrl::W`](W) writer structure"]
impl crate::Writable for DcxCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCX_CTRL to value 0"]
impl crate::Resettable for DcxCtrlSpec {}
