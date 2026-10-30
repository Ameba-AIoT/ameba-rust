#[doc = "Register `REG_STATE_MON` reader"]
pub type R = crate::R<RegStateMonSpec>;
#[doc = "Register `REG_STATE_MON` writer"]
pub type W = crate::W<RegStateMonSpec>;
#[doc = "Field `BIT_CUR_STATE0` reader - when BIT_WRITE_FSM_EN is valid,Indicate the FSM current state to write. When BIT_WRITE_FSM_EN is not valid, means read status"]
pub type BitCurState0R = crate::FieldReader;
#[doc = "Field `BIT_CUR_STATE0` writer - when BIT_WRITE_FSM_EN is valid,Indicate the FSM current state to write. When BIT_WRITE_FSM_EN is not valid, means read status"]
pub type BitCurState0W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_WRITE_FSM_EN` reader - when set 1, generate write pulse, otherwise means read status"]
pub type BitWriteFsmEnR = crate::BitReader;
#[doc = "Field `BIT_CUR_STATE1` reader - when BIT_WRITE_FSM_EN is valid,is rsvd. When BIT_WRITE_FSM_EN is not valid, means read status"]
pub type BitCurState1R = crate::FieldReader;
#[doc = "Field `BIT_STATE_SEL` reader - WMAC read and write FSM selection: 000,001:read mactx status 010,011:read mactx status 100:read response status 00xxx: write \\[6:0\\]to mactx fsm when \\[7\\] valid 01xxx:write \\[6:0\\]to macrx fsm when \\[7\\] valid 100xx: write \\[6:0\\]to response fsm when \\[7\\] valid"]
pub type BitStateSelR = crate::FieldReader;
#[doc = "Field `BIT_STATE_SEL` writer - WMAC read and write FSM selection: 000,001:read mactx status 010,011:read mactx status 100:read response status 00xxx: write \\[6:0\\]to mactx fsm when \\[7\\] valid 01xxx:write \\[6:0\\]to macrx fsm when \\[7\\] valid 100xx: write \\[6:0\\]to response fsm when \\[7\\] valid"]
pub type BitStateSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - when BIT_WRITE_FSM_EN is valid,Indicate the FSM current state to write. When BIT_WRITE_FSM_EN is not valid, means read status"]
    #[inline(always)]
    pub fn bit_cur_state0(&self) -> BitCurState0R {
        BitCurState0R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - when set 1, generate write pulse, otherwise means read status"]
    #[inline(always)]
    pub fn bit_write_fsm_en(&self) -> BitWriteFsmEnR {
        BitWriteFsmEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - when BIT_WRITE_FSM_EN is valid,is rsvd. When BIT_WRITE_FSM_EN is not valid, means read status"]
    #[inline(always)]
    pub fn bit_cur_state1(&self) -> BitCurState1R {
        BitCurState1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - WMAC read and write FSM selection: 000,001:read mactx status 010,011:read mactx status 100:read response status 00xxx: write \\[6:0\\]to mactx fsm when \\[7\\] valid 01xxx:write \\[6:0\\]to macrx fsm when \\[7\\] valid 100xx: write \\[6:0\\]to response fsm when \\[7\\] valid"]
    #[inline(always)]
    pub fn bit_state_sel(&self) -> BitStateSelR {
        BitStateSelR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - when BIT_WRITE_FSM_EN is valid,Indicate the FSM current state to write. When BIT_WRITE_FSM_EN is not valid, means read status"]
    #[inline(always)]
    pub fn bit_cur_state0(&mut self) -> BitCurState0W<'_, RegStateMonSpec> {
        BitCurState0W::new(self, 0)
    }
    #[doc = "Bits 24:28 - WMAC read and write FSM selection: 000,001:read mactx status 010,011:read mactx status 100:read response status 00xxx: write \\[6:0\\]to mactx fsm when \\[7\\] valid 01xxx:write \\[6:0\\]to macrx fsm when \\[7\\] valid 100xx: write \\[6:0\\]to response fsm when \\[7\\] valid"]
    #[inline(always)]
    pub fn bit_state_sel(&mut self) -> BitStateSelW<'_, RegStateMonSpec> {
        BitStateSelW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_state_mon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_state_mon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegStateMonSpec;
impl crate::RegisterSpec for RegStateMonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_state_mon::R`](R) reader structure"]
impl crate::Readable for RegStateMonSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_state_mon::W`](W) writer structure"]
impl crate::Writable for RegStateMonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_STATE_MON to value 0"]
impl crate::Resettable for RegStateMonSpec {}
