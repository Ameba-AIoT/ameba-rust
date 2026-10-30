#[doc = "Register `LA_EVENT_OPTION` reader"]
pub type R = crate::R<LaEventOptionSpec>;
#[doc = "Register `LA_EVENT_OPTION` writer"]
pub type W = crate::W<LaEventOptionSpec>;
#[doc = "Field `r_lastop_rx_incomp` reader - stop LA record when RX content is incompleted, include PLCP is incompleted or PSDU is incompleted"]
pub type RLastopRxIncompR = crate::BitReader;
#[doc = "Field `r_lastop_rx_incomp` writer - stop LA record when RX content is incompleted, include PLCP is incompleted or PSDU is incompleted"]
pub type RLastopRxIncompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_lastop_plcp_error` reader - stop LA record when RX PLCP error occur, include Length error, parity error or crc error"]
pub type RLastopPlcpErrorR = crate::BitReader;
#[doc = "Field `r_lastop_plcp_error` writer - stop LA record when RX PLCP error occur, include Length error, parity error or crc error"]
pub type RLastopPlcpErrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_lastop_rx_fsm_hang` reader - stop LA record when RX FSM timeout"]
pub type RLastopRxFsmHangR = crate::BitReader;
#[doc = "Field `r_lastop_rx_fsm_hang` writer - stop LA record when RX FSM timeout"]
pub type RLastopRxFsmHangW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_lastop_fcs_error` reader - stop LA record when continue fcs error >= threshold 0:disable 1:4 2:8 3:12"]
pub type RLastopFcsErrorR = crate::FieldReader;
#[doc = "Field `r_lastop_fcs_error` writer - stop LA record when continue fcs error >= threshold 0:disable 1:4 2:8 3:12"]
pub type RLastopFcsErrorW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - stop LA record when RX content is incompleted, include PLCP is incompleted or PSDU is incompleted"]
    #[inline(always)]
    pub fn r_lastop_rx_incomp(&self) -> RLastopRxIncompR {
        RLastopRxIncompR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - stop LA record when RX PLCP error occur, include Length error, parity error or crc error"]
    #[inline(always)]
    pub fn r_lastop_plcp_error(&self) -> RLastopPlcpErrorR {
        RLastopPlcpErrorR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - stop LA record when RX FSM timeout"]
    #[inline(always)]
    pub fn r_lastop_rx_fsm_hang(&self) -> RLastopRxFsmHangR {
        RLastopRxFsmHangR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - stop LA record when continue fcs error >= threshold 0:disable 1:4 2:8 3:12"]
    #[inline(always)]
    pub fn r_lastop_fcs_error(&self) -> RLastopFcsErrorR {
        RLastopFcsErrorR::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - stop LA record when RX content is incompleted, include PLCP is incompleted or PSDU is incompleted"]
    #[inline(always)]
    pub fn r_lastop_rx_incomp(&mut self) -> RLastopRxIncompW<'_, LaEventOptionSpec> {
        RLastopRxIncompW::new(self, 0)
    }
    #[doc = "Bit 1 - stop LA record when RX PLCP error occur, include Length error, parity error or crc error"]
    #[inline(always)]
    pub fn r_lastop_plcp_error(&mut self) -> RLastopPlcpErrorW<'_, LaEventOptionSpec> {
        RLastopPlcpErrorW::new(self, 1)
    }
    #[doc = "Bit 2 - stop LA record when RX FSM timeout"]
    #[inline(always)]
    pub fn r_lastop_rx_fsm_hang(&mut self) -> RLastopRxFsmHangW<'_, LaEventOptionSpec> {
        RLastopRxFsmHangW::new(self, 2)
    }
    #[doc = "Bits 3:4 - stop LA record when continue fcs error >= threshold 0:disable 1:4 2:8 3:12"]
    #[inline(always)]
    pub fn r_lastop_fcs_error(&mut self) -> RLastopFcsErrorW<'_, LaEventOptionSpec> {
        RLastopFcsErrorW::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`la_event_option::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`la_event_option::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LaEventOptionSpec;
impl crate::RegisterSpec for LaEventOptionSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`la_event_option::R`](R) reader structure"]
impl crate::Readable for LaEventOptionSpec {}
#[doc = "`write(|w| ..)` method takes [`la_event_option::W`](W) writer structure"]
impl crate::Writable for LaEventOptionSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LA_EVENT_OPTION to value 0"]
impl crate::Resettable for LaEventOptionSpec {}
