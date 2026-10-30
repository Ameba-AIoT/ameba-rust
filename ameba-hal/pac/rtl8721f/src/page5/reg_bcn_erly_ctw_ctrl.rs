#[doc = "Register `REG_BCN_ERLY_CTW_CTRL` reader"]
pub type R = crate::R<RegBcnErlyCtwCtrlSpec>;
#[doc = "Register `REG_BCN_ERLY_CTW_CTRL` writer"]
pub type W = crate::W<RegBcnErlyCtwCtrlSpec>;
#[doc = "Field `BIT_ATIMWND1` reader - ATIM window width of VAP1 in units of TU."]
pub type BitAtimwnd1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_ATIMWND1` writer - ATIM window width of VAP1 in units of TU."]
pub type BitAtimwnd1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BIT_CTWND_P0` reader - CTWindow width in units of TU. PORT0"]
pub type BitCtwndP0R = crate::FieldReader;
#[doc = "Field `BIT_CTWND_P0` writer - CTWindow width in units of TU. PORT0"]
pub type BitCtwndP0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_BCNIVLCUNT_P0` reader - Be used in multiple BCNIVL early INT. When this counter =1, multiple BCNIVL early INT will be triggered."]
pub type BitBcnivlcuntP0R = crate::FieldReader;
#[doc = "Field `ELY_IGNORE_LSB` reader - 1:add1ms, add128us, add32us only check tsft\\[10\\], tsft\\[7\\], tsft\\[5\\], ignore LSB 0: add1ms, add128us, add32us check tsft\\[9:0\\], tsft\\[6:0\\], tsft\\[4:0\\] when active"]
pub type ElyIgnoreLsbR = crate::BitReader;
#[doc = "Field `ELY_IGNORE_LSB` writer - 1:add1ms, add128us, add32us only check tsft\\[10\\], tsft\\[7\\], tsft\\[5\\], ignore LSB 0: add1ms, add128us, add32us check tsft\\[9:0\\], tsft\\[6:0\\], tsft\\[4:0\\] when active"]
pub type ElyIgnoreLsbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - ATIM window width of VAP1 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd1(&self) -> BitAtimwnd1R {
        BitAtimwnd1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - CTWindow width in units of TU. PORT0"]
    #[inline(always)]
    pub fn bit_ctwnd_p0(&self) -> BitCtwndP0R {
        BitCtwndP0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Be used in multiple BCNIVL early INT. When this counter =1, multiple BCNIVL early INT will be triggered."]
    #[inline(always)]
    pub fn bit_bcnivlcunt_p0(&self) -> BitBcnivlcuntP0R {
        BitBcnivlcuntP0R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - 1:add1ms, add128us, add32us only check tsft\\[10\\], tsft\\[7\\], tsft\\[5\\], ignore LSB 0: add1ms, add128us, add32us check tsft\\[9:0\\], tsft\\[6:0\\], tsft\\[4:0\\] when active"]
    #[inline(always)]
    pub fn ely_ignore_lsb(&self) -> ElyIgnoreLsbR {
        ElyIgnoreLsbR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - ATIM window width of VAP1 in units of TU."]
    #[inline(always)]
    pub fn bit_atimwnd1(&mut self) -> BitAtimwnd1W<'_, RegBcnErlyCtwCtrlSpec> {
        BitAtimwnd1W::new(self, 0)
    }
    #[doc = "Bits 16:23 - CTWindow width in units of TU. PORT0"]
    #[inline(always)]
    pub fn bit_ctwnd_p0(&mut self) -> BitCtwndP0W<'_, RegBcnErlyCtwCtrlSpec> {
        BitCtwndP0W::new(self, 16)
    }
    #[doc = "Bit 31 - 1:add1ms, add128us, add32us only check tsft\\[10\\], tsft\\[7\\], tsft\\[5\\], ignore LSB 0: add1ms, add128us, add32us check tsft\\[9:0\\], tsft\\[6:0\\], tsft\\[4:0\\] when active"]
    #[inline(always)]
    pub fn ely_ignore_lsb(&mut self) -> ElyIgnoreLsbW<'_, RegBcnErlyCtwCtrlSpec> {
        ElyIgnoreLsbW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_bcn_erly_ctw_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_bcn_erly_ctw_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegBcnErlyCtwCtrlSpec;
impl crate::RegisterSpec for RegBcnErlyCtwCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_bcn_erly_ctw_ctrl::R`](R) reader structure"]
impl crate::Readable for RegBcnErlyCtwCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_bcn_erly_ctw_ctrl::W`](W) writer structure"]
impl crate::Writable for RegBcnErlyCtwCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_BCN_ERLY_CTW_CTRL to value 0xff0a_0002"]
impl crate::Resettable for RegBcnErlyCtwCtrlSpec {
    const RESET_VALUE: u32 = 0xff0a_0002;
}
