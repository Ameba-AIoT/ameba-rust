#[doc = "Register `PDEX_CTRL2` reader"]
pub type R = crate::R<PdexCtrl2Spec>;
#[doc = "Register `PDEX_CTRL2` writer"]
pub type W = crate::W<PdexCtrl2Spec>;
#[doc = "Field `PDX_CMD` reader - Indicate the \\[1-0\\] byte of SPI PDX command value. If device is PSRAM, PDX command value is not care. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdxCmdR = crate::FieldReader<u16>;
#[doc = "Field `PDX_CMD` writer - Indicate the \\[1-0\\] byte of SPI PDX command value. If device is PSRAM, PDX command value is not care. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdxCmdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PDX_CMD_LEN` reader - Indicate the SPI PDX command length. The PDE command length indicates the valid command in PDX_CMD. If device is PSRAM, the equation is CR_PDX_CMD_LEN = (SPIC_CLK_FRQ * tXDPD)/1000. (SPIC_CLK_FRQ unit is MHz, tXDPD unit is ns) Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdxCmdLenR = crate::FieldReader<u16>;
#[doc = "Field `PDX_CMD_LEN` writer - Indicate the SPI PDX command length. The PDE command length indicates the valid command in PDX_CMD. If device is PSRAM, the equation is CR_PDX_CMD_LEN = (SPIC_CLK_FRQ * tXDPD)/1000. (SPIC_CLK_FRQ unit is MHz, tXDPD unit is ns) Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdxCmdLenW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `PDE_CMD_LEN` reader - Indicate the SPI PDE command length. The PDE command length indicates the valid command in PDE_CMD0 and PDE_CMD1 Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdeCmdLenR = crate::FieldReader;
#[doc = "Field `PDE_CMD_LEN` writer - Indicate the SPI PDE command length. The PDE command length indicates the valid command in PDE_CMD0 and PDE_CMD1 Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
pub type PdeCmdLenW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:15 - Indicate the \\[1-0\\] byte of SPI PDX command value. If device is PSRAM, PDX command value is not care. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pdx_cmd(&self) -> PdxCmdR {
        PdxCmdR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:27 - Indicate the SPI PDX command length. The PDE command length indicates the valid command in PDX_CMD. If device is PSRAM, the equation is CR_PDX_CMD_LEN = (SPIC_CLK_FRQ * tXDPD)/1000. (SPIC_CLK_FRQ unit is MHz, tXDPD unit is ns) Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pdx_cmd_len(&self) -> PdxCmdLenR {
        PdxCmdLenR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31 - Indicate the SPI PDE command length. The PDE command length indicates the valid command in PDE_CMD0 and PDE_CMD1 Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pde_cmd_len(&self) -> PdeCmdLenR {
        PdeCmdLenR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Indicate the \\[1-0\\] byte of SPI PDX command value. If device is PSRAM, PDX command value is not care. Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pdx_cmd(&mut self) -> PdxCmdW<'_, PdexCtrl2Spec> {
        PdxCmdW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Indicate the SPI PDX command length. The PDE command length indicates the valid command in PDX_CMD. If device is PSRAM, the equation is CR_PDX_CMD_LEN = (SPIC_CLK_FRQ * tXDPD)/1000. (SPIC_CLK_FRQ unit is MHz, tXDPD unit is ns) Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pdx_cmd_len(&mut self) -> PdxCmdLenW<'_, PdexCtrl2Spec> {
        PdxCmdLenW::new(self, 16)
    }
    #[doc = "Bits 28:31 - Indicate the SPI PDE command length. The PDE command length indicates the valid command in PDE_CMD0 and PDE_CMD1 Exists: TAG >= 2.0.5. Available only if configuration of SPI_PDEX_ENABLE is defined."]
    #[inline(always)]
    pub fn pde_cmd_len(&mut self) -> PdeCmdLenW<'_, PdexCtrl2Spec> {
        PdeCmdLenW::new(self, 28)
    }
}
#[doc = "PDEX Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`pdex_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdex_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PdexCtrl2Spec;
impl crate::RegisterSpec for PdexCtrl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pdex_ctrl2::R`](R) reader structure"]
impl crate::Readable for PdexCtrl2Spec {}
#[doc = "`write(|w| ..)` method takes [`pdex_ctrl2::W`](W) writer structure"]
impl crate::Writable for PdexCtrl2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PDEX_CTRL2 to value 0x1001_0000"]
impl crate::Resettable for PdexCtrl2Spec {
    const RESET_VALUE: u32 = 0x1001_0000;
}
