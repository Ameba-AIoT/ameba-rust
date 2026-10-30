#[doc = "Register `REG_CPU_MGQ_INFO` reader"]
pub type R = crate::R<RegCpuMgqInfoSpec>;
#[doc = "Register `REG_CPU_MGQ_INFO` writer"]
pub type W = crate::W<RegCpuMgqInfoSpec>;
#[doc = "Field `BIT_CPUMGQ_HEAD_PG` reader - The head page of CPU MGQ"]
pub type BitCpumgqHeadPgR = crate::FieldReader;
#[doc = "Field `BIT_CPUMGQ_FW_NUM` reader - The total cpu mgq fw number in TxPktbuffer"]
pub type BitCpumgqFwNumR = crate::BitReader;
#[doc = "Field `BIT_CMDQ_POLL` writer - polling CMD packet"]
pub type BitCmdqPollW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_EVTQ_VLD` reader - SW set to 1, HW clear to 0 when evtq timeout or tx_finish"]
pub type BitEvtqVldR = crate::BitReader;
#[doc = "Field `BIT_EVTQ_VLD` writer - SW set to 1, HW clear to 0 when evtq timeout or tx_finish"]
pub type BitEvtqVldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CPUMGT_POLL_CLR` writer - Polling to clear CPU MGT Queue packet"]
pub type BitCpumgtPollClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BCN_POLL` writer - Polling BCN Queue Packet"]
pub type BitBcnPollW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CPUMGT_POLL_SET` writer - Polling to set CPU MGT Queue packet"]
pub type BitCpumgtPollSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BCN_POLL1` writer - Polling BCN Queue1 Packet"]
pub type BitBcnPoll1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BCN_POLL2` writer - Polling BCN Queue2 Packet. Not used in 8723F"]
pub type BitBcnPoll2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - The head page of CPU MGQ"]
    #[inline(always)]
    pub fn bit_cpumgq_head_pg(&self) -> BitCpumgqHeadPgR {
        BitCpumgqHeadPgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - The total cpu mgq fw number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_cpumgq_fw_num(&self) -> BitCpumgqFwNumR {
        BitCpumgqFwNumR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 26 - SW set to 1, HW clear to 0 when evtq timeout or tx_finish"]
    #[inline(always)]
    pub fn bit_evtq_vld(&self) -> BitEvtqVldR {
        BitEvtqVldR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 24 - polling CMD packet"]
    #[inline(always)]
    pub fn bit_cmdq_poll(&mut self) -> BitCmdqPollW<'_, RegCpuMgqInfoSpec> {
        BitCmdqPollW::new(self, 24)
    }
    #[doc = "Bit 26 - SW set to 1, HW clear to 0 when evtq timeout or tx_finish"]
    #[inline(always)]
    pub fn bit_evtq_vld(&mut self) -> BitEvtqVldW<'_, RegCpuMgqInfoSpec> {
        BitEvtqVldW::new(self, 26)
    }
    #[doc = "Bit 27 - Polling to clear CPU MGT Queue packet"]
    #[inline(always)]
    pub fn bit_cpumgt_poll_clr(&mut self) -> BitCpumgtPollClrW<'_, RegCpuMgqInfoSpec> {
        BitCpumgtPollClrW::new(self, 27)
    }
    #[doc = "Bit 28 - Polling BCN Queue Packet"]
    #[inline(always)]
    pub fn bit_bcn_poll(&mut self) -> BitBcnPollW<'_, RegCpuMgqInfoSpec> {
        BitBcnPollW::new(self, 28)
    }
    #[doc = "Bit 29 - Polling to set CPU MGT Queue packet"]
    #[inline(always)]
    pub fn bit_cpumgt_poll_set(&mut self) -> BitCpumgtPollSetW<'_, RegCpuMgqInfoSpec> {
        BitCpumgtPollSetW::new(self, 29)
    }
    #[doc = "Bit 30 - Polling BCN Queue1 Packet"]
    #[inline(always)]
    pub fn bit_bcn_poll1(&mut self) -> BitBcnPoll1W<'_, RegCpuMgqInfoSpec> {
        BitBcnPoll1W::new(self, 30)
    }
    #[doc = "Bit 31 - Polling BCN Queue2 Packet. Not used in 8723F"]
    #[inline(always)]
    pub fn bit_bcn_poll2(&mut self) -> BitBcnPoll2W<'_, RegCpuMgqInfoSpec> {
        BitBcnPoll2W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cpu_mgq_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cpu_mgq_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCpuMgqInfoSpec;
impl crate::RegisterSpec for RegCpuMgqInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cpu_mgq_info::R`](R) reader structure"]
impl crate::Readable for RegCpuMgqInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cpu_mgq_info::W`](W) writer structure"]
impl crate::Writable for RegCpuMgqInfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CPU_MGQ_INFO to value 0xf6"]
impl crate::Resettable for RegCpuMgqInfoSpec {
    const RESET_VALUE: u32 = 0xf6;
}
