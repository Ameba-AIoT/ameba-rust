#[doc = "Register `DCX_CMD` reader"]
pub type R = crate::R<DcxCmdSpec>;
#[doc = "Register `DCX_CMD` writer"]
pub type W = crate::W<DcxCmdSpec>;
#[doc = "Field `CMD_HEAD_VAL` reader - Value of DCX header control bit in cmd phase. In 4-wire format cmd phase, spi_dcx = CMD_HEAD_VAL\\[0\\]."]
pub type CmdHeadValR = crate::FieldReader<u16>;
#[doc = "Field `CMD_HEAD_VAL` writer - Value of DCX header control bit in cmd phase. In 4-wire format cmd phase, spi_dcx = CMD_HEAD_VAL\\[0\\]."]
pub type CmdHeadValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CMD_TAIL_VAL` reader - Value of DCX tail control bit in cmd phase."]
pub type CmdTailValR = crate::FieldReader<u16>;
#[doc = "Field `CMD_TAIL_VAL` writer - Value of DCX tail control bit in cmd phase."]
pub type CmdTailValW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Value of DCX header control bit in cmd phase. In 4-wire format cmd phase, spi_dcx = CMD_HEAD_VAL\\[0\\]."]
    #[inline(always)]
    pub fn cmd_head_val(&self) -> CmdHeadValR {
        CmdHeadValR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Value of DCX tail control bit in cmd phase."]
    #[inline(always)]
    pub fn cmd_tail_val(&self) -> CmdTailValR {
        CmdTailValR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value of DCX header control bit in cmd phase. In 4-wire format cmd phase, spi_dcx = CMD_HEAD_VAL\\[0\\]."]
    #[inline(always)]
    pub fn cmd_head_val(&mut self) -> CmdHeadValW<'_, DcxCmdSpec> {
        CmdHeadValW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Value of DCX tail control bit in cmd phase."]
    #[inline(always)]
    pub fn cmd_tail_val(&mut self) -> CmdTailValW<'_, DcxCmdSpec> {
        CmdTailValW::new(self, 16)
    }
}
#[doc = "LCDC cmd phase DCX control bit value register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcx_cmd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcx_cmd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcxCmdSpec;
impl crate::RegisterSpec for DcxCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcx_cmd::R`](R) reader structure"]
impl crate::Readable for DcxCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`dcx_cmd::W`](W) writer structure"]
impl crate::Writable for DcxCmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCX_CMD to value 0"]
impl crate::Resettable for DcxCmdSpec {}
