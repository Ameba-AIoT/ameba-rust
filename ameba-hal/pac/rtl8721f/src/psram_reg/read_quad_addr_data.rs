#[doc = "Register `READ_QUAD_ADDR_DATA` reader"]
pub type R = crate::R<ReadQuadAddrDataSpec>;
#[doc = "Register `READ_QUAD_ADDR_DATA` writer"]
pub type W = crate::W<ReadQuadAddrDataSpec>;
#[doc = "Field `RD_QUAD_IO_CMD` reader - Indicates SPI Flash command value of quad address and data read command."]
pub type RdQuadIoCmdR = crate::FieldReader;
#[doc = "Field `RD_QUAD_IO_CMD` writer - Indicates SPI Flash command value of quad address and data read command."]
pub type RdQuadIoCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXIT_PRM_DUM_LEN` reader - If Exit PRM commend is more than 1 byte, use EXIT_PRM_DUM_LEN to extend the commend. It is referenced by bus_clk. Ex. In qpi mode (4-4-4), FFFFFFFFh data cycle should be issued (4-byte data in 8 spi cycles). EXIT_PRM_CMD only push 1 byte (2 spi cycles), so use EXIT_PRM_DUM_LEN to extend 6 spi cycles. EXIT_PRM_DUM_LEN = 6 (spi_cycle) * (2 * baurd_rate) Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
pub type ExitPrmDumLenR = crate::FieldReader;
#[doc = "Field `EXIT_PRM_DUM_LEN` writer - If Exit PRM commend is more than 1 byte, use EXIT_PRM_DUM_LEN to extend the commend. It is referenced by bus_clk. Ex. In qpi mode (4-4-4), FFFFFFFFh data cycle should be issued (4-byte data in 8 spi cycles). EXIT_PRM_CMD only push 1 byte (2 spi cycles), so use EXIT_PRM_DUM_LEN to extend 6 spi cycles. EXIT_PRM_DUM_LEN = 6 (spi_cycle) * (2 * baurd_rate) Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
pub type ExitPrmDumLenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PRM_VAL` reader - High Performance Read Mode Value. Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
pub type PrmValR = crate::FieldReader;
#[doc = "Field `PRM_VAL` writer - High Performance Read Mode Value. Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
pub type PrmValW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EXIT_PRM_CMD` reader - Exit High Performance Read Mode commend. Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
pub type ExitPrmCmdR = crate::FieldReader;
#[doc = "Field `EXIT_PRM_CMD` writer - Exit High Performance Read Mode commend. Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
pub type ExitPrmCmdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of quad address and data read command."]
    #[inline(always)]
    pub fn rd_quad_io_cmd(&self) -> RdQuadIoCmdR {
        RdQuadIoCmdR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - If Exit PRM commend is more than 1 byte, use EXIT_PRM_DUM_LEN to extend the commend. It is referenced by bus_clk. Ex. In qpi mode (4-4-4), FFFFFFFFh data cycle should be issued (4-byte data in 8 spi cycles). EXIT_PRM_CMD only push 1 byte (2 spi cycles), so use EXIT_PRM_DUM_LEN to extend 6 spi cycles. EXIT_PRM_DUM_LEN = 6 (spi_cycle) * (2 * baurd_rate) Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
    #[inline(always)]
    pub fn exit_prm_dum_len(&self) -> ExitPrmDumLenR {
        ExitPrmDumLenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - High Performance Read Mode Value. Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
    #[inline(always)]
    pub fn prm_val(&self) -> PrmValR {
        PrmValR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Exit High Performance Read Mode commend. Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
    #[inline(always)]
    pub fn exit_prm_cmd(&self) -> ExitPrmCmdR {
        ExitPrmCmdR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indicates SPI Flash command value of quad address and data read command."]
    #[inline(always)]
    pub fn rd_quad_io_cmd(&mut self) -> RdQuadIoCmdW<'_, ReadQuadAddrDataSpec> {
        RdQuadIoCmdW::new(self, 0)
    }
    #[doc = "Bits 8:15 - If Exit PRM commend is more than 1 byte, use EXIT_PRM_DUM_LEN to extend the commend. It is referenced by bus_clk. Ex. In qpi mode (4-4-4), FFFFFFFFh data cycle should be issued (4-byte data in 8 spi cycles). EXIT_PRM_CMD only push 1 byte (2 spi cycles), so use EXIT_PRM_DUM_LEN to extend 6 spi cycles. EXIT_PRM_DUM_LEN = 6 (spi_cycle) * (2 * baurd_rate) Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
    #[inline(always)]
    pub fn exit_prm_dum_len(&mut self) -> ExitPrmDumLenW<'_, ReadQuadAddrDataSpec> {
        ExitPrmDumLenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - High Performance Read Mode Value. Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
    #[inline(always)]
    pub fn prm_val(&mut self) -> PrmValW<'_, ReadQuadAddrDataSpec> {
        PrmValW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Exit High Performance Read Mode commend. Note: Available only if configuration of PRM_EN is defined (SPIC Lite ver)."]
    #[inline(always)]
    pub fn exit_prm_cmd(&mut self) -> ExitPrmCmdW<'_, ReadQuadAddrDataSpec> {
        ExitPrmCmdW::new(self, 24)
    }
}
#[doc = "Quad I/O Read Command of SPI Flash\n\nYou can [`read`](crate::Reg::read) this register and get [`read_quad_addr_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`read_quad_addr_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReadQuadAddrDataSpec;
impl crate::RegisterSpec for ReadQuadAddrDataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`read_quad_addr_data::R`](R) reader structure"]
impl crate::Readable for ReadQuadAddrDataSpec {}
#[doc = "`write(|w| ..)` method takes [`read_quad_addr_data::W`](W) writer structure"]
impl crate::Writable for ReadQuadAddrDataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets READ_QUAD_ADDR_DATA to value 0"]
impl crate::Resettable for ReadQuadAddrDataSpec {}
