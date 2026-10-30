#[doc = "Register `REG_STATE_DEBUG_CONTROL` reader"]
pub type R = crate::R<RegStateDebugControlSpec>;
#[doc = "Register `REG_STATE_DEBUG_CONTROL` writer"]
pub type W = crate::W<RegStateDebugControlSpec>;
#[doc = "Field `BIT_MAC_DBG_SHIFT` reader - MACDEBUG\\[31:0\\] byte shift control signal 2'b00: MACDEBUG\\[31:0\\] 2'b01:{MACDEBUG\\[7:0\\],MACDEBUG\\[31:8\\]} 2'b10:{MACDEBUG\\[15:0\\],MACDEBUG\\[31:16\\]} 2'b11:{MACDEBUG\\[23:0\\],MACDEBUG\\[31:24\\]}"]
pub type BitMacDbgShiftR = crate::FieldReader;
#[doc = "Field `BIT_MAC_DBG_SHIFT` writer - MACDEBUG\\[31:0\\] byte shift control signal 2'b00: MACDEBUG\\[31:0\\] 2'b01:{MACDEBUG\\[7:0\\],MACDEBUG\\[31:8\\]} 2'b10:{MACDEBUG\\[15:0\\],MACDEBUG\\[31:16\\]} 2'b11:{MACDEBUG\\[23:0\\],MACDEBUG\\[31:24\\]}"]
pub type BitMacDbgShiftW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_STATE_DBG_EN_0` reader - enable state machine 1 for debug port"]
pub type BitStateDbgEn0R = crate::BitReader;
#[doc = "Field `BIT_STATE_DBG_EN_0` writer - enable state machine 1 for debug port"]
pub type BitStateDbgEn0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_STATE_DBG_EN_1` reader - enable state machine 1 for debug port"]
pub type BitStateDbgEn1R = crate::BitReader;
#[doc = "Field `BIT_STATE_DBG_EN_1` writer - enable state machine 1 for debug port"]
pub type BitStateDbgEn1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_STATE_DBG_SEL_0` reader - select state machine 0 for debug port"]
pub type BitStateDbgSel0R = crate::FieldReader;
#[doc = "Field `BIT_STATE_DBG_SEL_0` writer - select state machine 0 for debug port"]
pub type BitStateDbgSel0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_STATE_DBG_SEL_1` reader - select state machine 1 for debug port"]
pub type BitStateDbgSel1R = crate::FieldReader;
#[doc = "Field `BIT_STATE_DBG_SEL_1` writer - select state machine 1 for debug port"]
pub type BitStateDbgSel1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - MACDEBUG\\[31:0\\] byte shift control signal 2'b00: MACDEBUG\\[31:0\\] 2'b01:{MACDEBUG\\[7:0\\],MACDEBUG\\[31:8\\]} 2'b10:{MACDEBUG\\[15:0\\],MACDEBUG\\[31:16\\]} 2'b11:{MACDEBUG\\[23:0\\],MACDEBUG\\[31:24\\]}"]
    #[inline(always)]
    pub fn bit_mac_dbg_shift(&self) -> BitMacDbgShiftR {
        BitMacDbgShiftR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - enable state machine 1 for debug port"]
    #[inline(always)]
    pub fn bit_state_dbg_en_0(&self) -> BitStateDbgEn0R {
        BitStateDbgEn0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable state machine 1 for debug port"]
    #[inline(always)]
    pub fn bit_state_dbg_en_1(&self) -> BitStateDbgEn1R {
        BitStateDbgEn1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:9 - select state machine 0 for debug port"]
    #[inline(always)]
    pub fn bit_state_dbg_sel_0(&self) -> BitStateDbgSel0R {
        BitStateDbgSel0R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 10:15 - select state machine 1 for debug port"]
    #[inline(always)]
    pub fn bit_state_dbg_sel_1(&self) -> BitStateDbgSel1R {
        BitStateDbgSel1R::new(((self.bits >> 10) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - MACDEBUG\\[31:0\\] byte shift control signal 2'b00: MACDEBUG\\[31:0\\] 2'b01:{MACDEBUG\\[7:0\\],MACDEBUG\\[31:8\\]} 2'b10:{MACDEBUG\\[15:0\\],MACDEBUG\\[31:16\\]} 2'b11:{MACDEBUG\\[23:0\\],MACDEBUG\\[31:24\\]}"]
    #[inline(always)]
    pub fn bit_mac_dbg_shift(&mut self) -> BitMacDbgShiftW<'_, RegStateDebugControlSpec> {
        BitMacDbgShiftW::new(self, 0)
    }
    #[doc = "Bit 2 - enable state machine 1 for debug port"]
    #[inline(always)]
    pub fn bit_state_dbg_en_0(&mut self) -> BitStateDbgEn0W<'_, RegStateDebugControlSpec> {
        BitStateDbgEn0W::new(self, 2)
    }
    #[doc = "Bit 3 - enable state machine 1 for debug port"]
    #[inline(always)]
    pub fn bit_state_dbg_en_1(&mut self) -> BitStateDbgEn1W<'_, RegStateDebugControlSpec> {
        BitStateDbgEn1W::new(self, 3)
    }
    #[doc = "Bits 4:9 - select state machine 0 for debug port"]
    #[inline(always)]
    pub fn bit_state_dbg_sel_0(&mut self) -> BitStateDbgSel0W<'_, RegStateDebugControlSpec> {
        BitStateDbgSel0W::new(self, 4)
    }
    #[doc = "Bits 10:15 - select state machine 1 for debug port"]
    #[inline(always)]
    pub fn bit_state_dbg_sel_1(&mut self) -> BitStateDbgSel1W<'_, RegStateDebugControlSpec> {
        BitStateDbgSel1W::new(self, 10)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_state_debug_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_state_debug_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegStateDebugControlSpec;
impl crate::RegisterSpec for RegStateDebugControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_state_debug_control::R`](R) reader structure"]
impl crate::Readable for RegStateDebugControlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_state_debug_control::W`](W) writer structure"]
impl crate::Writable for RegStateDebugControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_STATE_DEBUG_CONTROL to value 0"]
impl crate::Resettable for RegStateDebugControlSpec {}
