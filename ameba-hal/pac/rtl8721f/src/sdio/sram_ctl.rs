#[doc = "Register `SRAM_CTL` reader"]
pub type R = crate::R<SramCtlSpec>;
#[doc = "Register `SRAM_CTL` writer"]
pub type W = crate::W<SramCtlSpec>;
#[doc = "Field `mcu_sel` reader - 0: elbi I/F (dword access for SDIO host standard register) 1: mcu I/F (byte access for SDIO IP register)"]
pub type McuSelR = crate::BitReader;
#[doc = "Field `mcu_sel` writer - 0: elbi I/F (dword access for SDIO host standard register) 1: mcu I/F (byte access for SDIO IP register)"]
pub type McuSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mcu_buf_access` reader - in bit\\[0\\] = 1 condition, bit\\[2:1\\] = 'b00 : buf access disable. bit\\[2:1\\] = 'b01 : buf abs access enable, ring buffer address is direct mapping to 0x4005CB00 ~ 0x4005CEFF. (mcu_addr\\[15:0\\]=0000~3FF, mcu_cs_buf_absolute) bit\\[2:1\\] = 'b10: buf auto access enable, ring buffer could be access by register (mcu_addr=FE30~FE33, mcu_cs_buf_auto) 0x4005CB30 (sram data \\[7:0\\]), 0x4005CB31 (sram data \\[15:8\\]), 0x4005CB32 (sram data \\[23:16\\]), 0x4005CB33 (sram data \\[31:24\\]), address will auto increase by 4. bit\\[2:1\\] = 'b11 : buf access disable."]
pub type McuBufAccessR = crate::FieldReader;
#[doc = "Field `mcu_buf_access` writer - in bit\\[0\\] = 1 condition, bit\\[2:1\\] = 'b00 : buf access disable. bit\\[2:1\\] = 'b01 : buf abs access enable, ring buffer address is direct mapping to 0x4005CB00 ~ 0x4005CEFF. (mcu_addr\\[15:0\\]=0000~3FF, mcu_cs_buf_absolute) bit\\[2:1\\] = 'b10: buf auto access enable, ring buffer could be access by register (mcu_addr=FE30~FE33, mcu_cs_buf_auto) 0x4005CB30 (sram data \\[7:0\\]), 0x4005CB31 (sram data \\[15:8\\]), 0x4005CB32 (sram data \\[23:16\\]), 0x4005CB33 (sram data \\[31:24\\]), address will auto increase by 4. bit\\[2:1\\] = 'b11 : buf access disable."]
pub type McuBufAccessW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Lx_burst_size` reader - 2'b11: Reserved 2'b10:burst length = 256 bytes (64 Dwords) 2'b01: burst length = 128 bytes (32 Dwords) 2'b00: burst length = 64 bytes (16 Dwords)"]
pub type LxBurstSizeR = crate::FieldReader;
#[doc = "Field `Lx_burst_size` writer - 2'b11: Reserved 2'b10:burst length = 256 bytes (64 Dwords) 2'b01: burst length = 128 bytes (32 Dwords) 2'b00: burst length = 64 bytes (16 Dwords)"]
pub type LxBurstSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Lx_swap` reader - Reserved"]
pub type LxSwapR = crate::BitReader;
#[doc = "Field `Lx_swap` writer - Reserved"]
pub type LxSwapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ahbm_hburst_cfg` reader - bit\\[3\\]: ena_incrx_align: alignment Burst Type enable bit\\[2\\]: ena_inc16: INCR16 Burst Type Enable bit\\[1\\]: ena_inc8: INCR8 Burst Type Enable bit\\[0\\]: ena_inc4: INCR4 Burst Type Enable"]
pub type AhbmHburstCfgR = crate::FieldReader;
#[doc = "Field `ahbm_hburst_cfg` writer - bit\\[3\\]: ena_incrx_align: alignment Burst Type enable bit\\[2\\]: ena_inc16: INCR16 Burst Type Enable bit\\[1\\]: ena_inc8: INCR8 Burst Type Enable bit\\[0\\]: ena_inc4: INCR4 Burst Type Enable"]
pub type AhbmHburstCfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ahbm_hprot_cfg` reader - AHB bus Protection control"]
pub type AhbmHprotCfgR = crate::FieldReader;
#[doc = "Field `ahbm_hprot_cfg` writer - AHB bus Protection control"]
pub type AhbmHprotCfgW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `buf_read_size` reader - Buffer Data Port Read Size (BDPRS) 2'b00: 1-byte access, port_a_rd_ptr increment by 1 2'b01: 2-byte access, port_a_rd_ptr increment by 2 2'b10: 4-byte access, port_a_rd_ptr increment by 4 2'b11: decoding byte enable(BE\\[3:0\\]) bus signal If block-size is odd-byte, please set buf_read_size=2'b00"]
pub type BufReadSizeR = crate::FieldReader;
#[doc = "Field `buf_read_size` writer - Buffer Data Port Read Size (BDPRS) 2'b00: 1-byte access, port_a_rd_ptr increment by 1 2'b01: 2-byte access, port_a_rd_ptr increment by 2 2'b10: 4-byte access, port_a_rd_ptr increment by 4 2'b11: decoding byte enable(BE\\[3:0\\]) bus signal If block-size is odd-byte, please set buf_read_size=2'b00"]
pub type BufReadSizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - 0: elbi I/F (dword access for SDIO host standard register) 1: mcu I/F (byte access for SDIO IP register)"]
    #[inline(always)]
    pub fn mcu_sel(&self) -> McuSelR {
        McuSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - in bit\\[0\\] = 1 condition, bit\\[2:1\\] = 'b00 : buf access disable. bit\\[2:1\\] = 'b01 : buf abs access enable, ring buffer address is direct mapping to 0x4005CB00 ~ 0x4005CEFF. (mcu_addr\\[15:0\\]=0000~3FF, mcu_cs_buf_absolute) bit\\[2:1\\] = 'b10: buf auto access enable, ring buffer could be access by register (mcu_addr=FE30~FE33, mcu_cs_buf_auto) 0x4005CB30 (sram data \\[7:0\\]), 0x4005CB31 (sram data \\[15:8\\]), 0x4005CB32 (sram data \\[23:16\\]), 0x4005CB33 (sram data \\[31:24\\]), address will auto increase by 4. bit\\[2:1\\] = 'b11 : buf access disable."]
    #[inline(always)]
    pub fn mcu_buf_access(&self) -> McuBufAccessR {
        McuBufAccessR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 8:9 - 2'b11: Reserved 2'b10:burst length = 256 bytes (64 Dwords) 2'b01: burst length = 128 bytes (32 Dwords) 2'b00: burst length = 64 bytes (16 Dwords)"]
    #[inline(always)]
    pub fn lx_burst_size(&self) -> LxBurstSizeR {
        LxBurstSizeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn lx_swap(&self) -> LxSwapR {
        LxSwapR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:19 - bit\\[3\\]: ena_incrx_align: alignment Burst Type enable bit\\[2\\]: ena_inc16: INCR16 Burst Type Enable bit\\[1\\]: ena_inc8: INCR8 Burst Type Enable bit\\[0\\]: ena_inc4: INCR4 Burst Type Enable"]
    #[inline(always)]
    pub fn ahbm_hburst_cfg(&self) -> AhbmHburstCfgR {
        AhbmHburstCfgR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AHB bus Protection control"]
    #[inline(always)]
    pub fn ahbm_hprot_cfg(&self) -> AhbmHprotCfgR {
        AhbmHprotCfgR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:25 - Buffer Data Port Read Size (BDPRS) 2'b00: 1-byte access, port_a_rd_ptr increment by 1 2'b01: 2-byte access, port_a_rd_ptr increment by 2 2'b10: 4-byte access, port_a_rd_ptr increment by 4 2'b11: decoding byte enable(BE\\[3:0\\]) bus signal If block-size is odd-byte, please set buf_read_size=2'b00"]
    #[inline(always)]
    pub fn buf_read_size(&self) -> BufReadSizeR {
        BufReadSizeR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - 0: elbi I/F (dword access for SDIO host standard register) 1: mcu I/F (byte access for SDIO IP register)"]
    #[inline(always)]
    pub fn mcu_sel(&mut self) -> McuSelW<'_, SramCtlSpec> {
        McuSelW::new(self, 0)
    }
    #[doc = "Bits 1:2 - in bit\\[0\\] = 1 condition, bit\\[2:1\\] = 'b00 : buf access disable. bit\\[2:1\\] = 'b01 : buf abs access enable, ring buffer address is direct mapping to 0x4005CB00 ~ 0x4005CEFF. (mcu_addr\\[15:0\\]=0000~3FF, mcu_cs_buf_absolute) bit\\[2:1\\] = 'b10: buf auto access enable, ring buffer could be access by register (mcu_addr=FE30~FE33, mcu_cs_buf_auto) 0x4005CB30 (sram data \\[7:0\\]), 0x4005CB31 (sram data \\[15:8\\]), 0x4005CB32 (sram data \\[23:16\\]), 0x4005CB33 (sram data \\[31:24\\]), address will auto increase by 4. bit\\[2:1\\] = 'b11 : buf access disable."]
    #[inline(always)]
    pub fn mcu_buf_access(&mut self) -> McuBufAccessW<'_, SramCtlSpec> {
        McuBufAccessW::new(self, 1)
    }
    #[doc = "Bits 8:9 - 2'b11: Reserved 2'b10:burst length = 256 bytes (64 Dwords) 2'b01: burst length = 128 bytes (32 Dwords) 2'b00: burst length = 64 bytes (16 Dwords)"]
    #[inline(always)]
    pub fn lx_burst_size(&mut self) -> LxBurstSizeW<'_, SramCtlSpec> {
        LxBurstSizeW::new(self, 8)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn lx_swap(&mut self) -> LxSwapW<'_, SramCtlSpec> {
        LxSwapW::new(self, 10)
    }
    #[doc = "Bits 16:19 - bit\\[3\\]: ena_incrx_align: alignment Burst Type enable bit\\[2\\]: ena_inc16: INCR16 Burst Type Enable bit\\[1\\]: ena_inc8: INCR8 Burst Type Enable bit\\[0\\]: ena_inc4: INCR4 Burst Type Enable"]
    #[inline(always)]
    pub fn ahbm_hburst_cfg(&mut self) -> AhbmHburstCfgW<'_, SramCtlSpec> {
        AhbmHburstCfgW::new(self, 16)
    }
    #[doc = "Bits 20:23 - AHB bus Protection control"]
    #[inline(always)]
    pub fn ahbm_hprot_cfg(&mut self) -> AhbmHprotCfgW<'_, SramCtlSpec> {
        AhbmHprotCfgW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Buffer Data Port Read Size (BDPRS) 2'b00: 1-byte access, port_a_rd_ptr increment by 1 2'b01: 2-byte access, port_a_rd_ptr increment by 2 2'b10: 4-byte access, port_a_rd_ptr increment by 4 2'b11: decoding byte enable(BE\\[3:0\\]) bus signal If block-size is odd-byte, please set buf_read_size=2'b00"]
    #[inline(always)]
    pub fn buf_read_size(&mut self) -> BufReadSizeW<'_, SramCtlSpec> {
        BufReadSizeW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sram_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sram_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SramCtlSpec;
impl crate::RegisterSpec for SramCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sram_ctl::R`](R) reader structure"]
impl crate::Readable for SramCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`sram_ctl::W`](W) writer structure"]
impl crate::Writable for SramCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAM_CTL to value 0x021f_0200"]
impl crate::Resettable for SramCtlSpec {
    const RESET_VALUE: u32 = 0x021f_0200;
}
