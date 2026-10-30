#[doc = "Register `REG_IQ_DUMP` reader"]
pub type R = crate::R<RegIqDumpSpec>;
#[doc = "Register `REG_IQ_DUMP` writer"]
pub type W = crate::W<RegIqDumpSpec>;
#[doc = "Field `BIT_r_IQDATA_DUMP` reader - enable data dump for txpktbuffer high 64K"]
pub type BitRIqdataDumpR = crate::BitReader;
#[doc = "Field `BIT_r_IQDATA_DUMP` writer - enable data dump for txpktbuffer high 64K"]
pub type BitRIqdataDumpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_EN_IQDUMP` reader - enable dump bb ADC signal"]
pub type BitREnIqdumpR = crate::BitReader;
#[doc = "Field `BIT_r_EN_IQDUMP` writer - enable dump bb ADC signal"]
pub type BitREnIqdumpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_en_LA_MAC` reader - enable dump mac dbg signal"]
pub type BitEnLaMacR = crate::BitReader;
#[doc = "Field `BIT_en_LA_MAC` writer - enable dump mac dbg signal"]
pub type BitEnLaMacW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_LEVEL_PULSE_SEL` reader - if dump bb ADC signal : \\[3\\] for sel mac trigger event if dump mac dbg signal \\[4:3\\] : 2'b00 : level signal match \\[4:3\\] : 2'b01 : posedge match \\[4:3\\] : 2'b10 : negdge match"]
pub type BitRLevelPulseSelR = crate::FieldReader;
#[doc = "Field `BIT_r_LEVEL_PULSE_SEL` writer - if dump bb ADC signal : \\[3\\] for sel mac trigger event if dump mac dbg signal \\[4:3\\] : 2'b00 : level signal match \\[4:3\\] : 2'b01 : posedge match \\[4:3\\] : 2'b10 : negdge match"]
pub type BitRLevelPulseSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_MAC_TRIG_REG` reader - dump bb ADC sel mac strigger even write this reg to 1 to trig dump finish"]
pub type BitMacTrigRegR = crate::BitReader;
#[doc = "Field `BIT_MAC_TRIG_REG` writer - dump bb ADC sel mac strigger even write this reg to 1 to trig dump finish"]
pub type BitMacTrigRegW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_r_MAC_TRIG_SEL` reader - dump bb ADC sel mac strigger even 2'b00 : rxpkt ok trig dump ok 2'b01 : rxpkt_fauil trig dump ok 2'b10 : rxpkt cca rise edge to trig dump ok"]
pub type BitRMacTrigSelR = crate::FieldReader;
#[doc = "Field `BIT_r_MAC_TRIG_SEL` writer - dump bb ADC sel mac strigger even 2'b00 : rxpkt ok trig dump ok 2'b01 : rxpkt_fauil trig dump ok 2'b10 : rxpkt cca rise edge to trig dump ok"]
pub type BitRMacTrigSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_r_TRIG_TIME_SEL` reader - dump ok time to finish dump signal(unit is 1us)"]
pub type BitRTrigTimeSelR = crate::FieldReader;
#[doc = "Field `BIT_r_TRIG_TIME_SEL` writer - dump ok time to finish dump signal(unit is 1us)"]
pub type BitRTrigTimeSelW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_MACDBG_TRIG_IQDUMP` reader - Enable dump bb ADC signal trigger from MAC debug port which controled by r_LEVEL_PULSE_SEL."]
pub type BitMacdbgTrigIqdumpR = crate::BitReader;
#[doc = "Field `BIT_MACDBG_TRIG_IQDUMP` writer - Enable dump bb ADC signal trigger from MAC debug port which controled by r_LEVEL_PULSE_SEL."]
pub type BitMacdbgTrigIqdumpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_DUMP_OK_ADDR` reader - 31\\] : dump signal addr has round up \\[30:16\\] : dump finish addr in txpktbuffer"]
pub type BitDumpOkAddrR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - enable data dump for txpktbuffer high 64K"]
    #[inline(always)]
    pub fn bit_r_iqdata_dump(&self) -> BitRIqdataDumpR {
        BitRIqdataDumpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enable dump bb ADC signal"]
    #[inline(always)]
    pub fn bit_r_en_iqdump(&self) -> BitREnIqdumpR {
        BitREnIqdumpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - enable dump mac dbg signal"]
    #[inline(always)]
    pub fn bit_en_la_mac(&self) -> BitEnLaMacR {
        BitEnLaMacR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - if dump bb ADC signal : \\[3\\] for sel mac trigger event if dump mac dbg signal \\[4:3\\] : 2'b00 : level signal match \\[4:3\\] : 2'b01 : posedge match \\[4:3\\] : 2'b10 : negdge match"]
    #[inline(always)]
    pub fn bit_r_level_pulse_sel(&self) -> BitRLevelPulseSelR {
        BitRLevelPulseSelR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - dump bb ADC sel mac strigger even write this reg to 1 to trig dump finish"]
    #[inline(always)]
    pub fn bit_mac_trig_reg(&self) -> BitMacTrigRegR {
        BitMacTrigRegR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - dump bb ADC sel mac strigger even 2'b00 : rxpkt ok trig dump ok 2'b01 : rxpkt_fauil trig dump ok 2'b10 : rxpkt cca rise edge to trig dump ok"]
    #[inline(always)]
    pub fn bit_r_mac_trig_sel(&self) -> BitRMacTrigSelR {
        BitRMacTrigSelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - dump ok time to finish dump signal(unit is 1us)"]
    #[inline(always)]
    pub fn bit_r_trig_time_sel(&self) -> BitRTrigTimeSelR {
        BitRTrigTimeSelR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Enable dump bb ADC signal trigger from MAC debug port which controled by r_LEVEL_PULSE_SEL."]
    #[inline(always)]
    pub fn bit_macdbg_trig_iqdump(&self) -> BitMacdbgTrigIqdumpR {
        BitMacdbgTrigIqdumpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31\\] : dump signal addr has round up \\[30:16\\] : dump finish addr in txpktbuffer"]
    #[inline(always)]
    pub fn bit_dump_ok_addr(&self) -> BitDumpOkAddrR {
        BitDumpOkAddrR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - enable data dump for txpktbuffer high 64K"]
    #[inline(always)]
    pub fn bit_r_iqdata_dump(&mut self) -> BitRIqdataDumpW<'_, RegIqDumpSpec> {
        BitRIqdataDumpW::new(self, 0)
    }
    #[doc = "Bit 1 - enable dump bb ADC signal"]
    #[inline(always)]
    pub fn bit_r_en_iqdump(&mut self) -> BitREnIqdumpW<'_, RegIqDumpSpec> {
        BitREnIqdumpW::new(self, 1)
    }
    #[doc = "Bit 2 - enable dump mac dbg signal"]
    #[inline(always)]
    pub fn bit_en_la_mac(&mut self) -> BitEnLaMacW<'_, RegIqDumpSpec> {
        BitEnLaMacW::new(self, 2)
    }
    #[doc = "Bits 3:4 - if dump bb ADC signal : \\[3\\] for sel mac trigger event if dump mac dbg signal \\[4:3\\] : 2'b00 : level signal match \\[4:3\\] : 2'b01 : posedge match \\[4:3\\] : 2'b10 : negdge match"]
    #[inline(always)]
    pub fn bit_r_level_pulse_sel(&mut self) -> BitRLevelPulseSelW<'_, RegIqDumpSpec> {
        BitRLevelPulseSelW::new(self, 3)
    }
    #[doc = "Bit 5 - dump bb ADC sel mac strigger even write this reg to 1 to trig dump finish"]
    #[inline(always)]
    pub fn bit_mac_trig_reg(&mut self) -> BitMacTrigRegW<'_, RegIqDumpSpec> {
        BitMacTrigRegW::new(self, 5)
    }
    #[doc = "Bits 6:7 - dump bb ADC sel mac strigger even 2'b00 : rxpkt ok trig dump ok 2'b01 : rxpkt_fauil trig dump ok 2'b10 : rxpkt cca rise edge to trig dump ok"]
    #[inline(always)]
    pub fn bit_r_mac_trig_sel(&mut self) -> BitRMacTrigSelW<'_, RegIqDumpSpec> {
        BitRMacTrigSelW::new(self, 6)
    }
    #[doc = "Bits 8:14 - dump ok time to finish dump signal(unit is 1us)"]
    #[inline(always)]
    pub fn bit_r_trig_time_sel(&mut self) -> BitRTrigTimeSelW<'_, RegIqDumpSpec> {
        BitRTrigTimeSelW::new(self, 8)
    }
    #[doc = "Bit 15 - Enable dump bb ADC signal trigger from MAC debug port which controled by r_LEVEL_PULSE_SEL."]
    #[inline(always)]
    pub fn bit_macdbg_trig_iqdump(&mut self) -> BitMacdbgTrigIqdumpW<'_, RegIqDumpSpec> {
        BitMacdbgTrigIqdumpW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_iq_dump::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_iq_dump::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegIqDumpSpec;
impl crate::RegisterSpec for RegIqDumpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_iq_dump::R`](R) reader structure"]
impl crate::Readable for RegIqDumpSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_iq_dump::W`](W) writer structure"]
impl crate::Writable for RegIqDumpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_IQ_DUMP to value 0"]
impl crate::Resettable for RegIqDumpSpec {}
