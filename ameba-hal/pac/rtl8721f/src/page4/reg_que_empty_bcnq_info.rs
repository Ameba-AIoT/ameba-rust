#[doc = "Register `REG_QUE_EMPTY_BCNQ_INFO` reader"]
pub type R = crate::R<RegQueEmptyBcnqInfoSpec>;
#[doc = "Field `BIT_BCNQ_HEAD_PG` reader - The head page of the first packet of Bcnq"]
pub type BitBcnqHeadPgR = crate::FieldReader;
#[doc = "Field `BIT_PKT_Num_BCNQ` reader - The total BcnQ pkt number in TxPktbuffer"]
pub type BitPktNumBcnqR = crate::FieldReader;
#[doc = "Field `BIT_AC0Q_EMPTY` reader - When this bit is raised, AC0 queue is empty. Report by MAC."]
pub type BitAc0qEmptyR = crate::BitReader;
#[doc = "Field `BIT_AC1Q_EMPTY` reader - When this bit is raised, AC1 queue is empty. Report by MAC."]
pub type BitAc1qEmptyR = crate::BitReader;
#[doc = "Field `BIT_AC2Q_EMPTY` reader - When this bit is raised, AC2 queue is empty. Report by MAC."]
pub type BitAc2qEmptyR = crate::BitReader;
#[doc = "Field `BIT_AC3Q_EMPTY` reader - When this bit is raised, AC3 queue is empty. Report by MAC."]
pub type BitAc3qEmptyR = crate::BitReader;
#[doc = "Field `BIT_AC4Q_EMPTY` reader - When this bit is raised, AC4 queue is empty. Report by MAC."]
pub type BitAc4qEmptyR = crate::BitReader;
#[doc = "Field `BIT_AC5Q_EMPTY` reader - When this bit is raised, AC5 queue is empty. Report by MAC."]
pub type BitAc5qEmptyR = crate::BitReader;
#[doc = "Field `BIT_AC6Q_EMPTY` reader - When this bit is raised, AC6 queue is empty. Report by MAC."]
pub type BitAc6qEmptyR = crate::BitReader;
#[doc = "Field `BIT_AC7Q_EMPTY` reader - When this bit is raised, AC7 queue is empty. Report by MAC."]
pub type BitAc7qEmptyR = crate::BitReader;
#[doc = "Field `BIT_MGQ_cpu_EMPTY` reader - When this bit is raised, MGQ queue poll by cpu is empty. Report by MAC."]
pub type BitMgqCpuEmptyR = crate::BitReader;
#[doc = "Field `BIT_MQQ_EMPTY` reader - When this bit is raised, MGQ queue is empty. Report by MAC."]
pub type BitMqqEmptyR = crate::BitReader;
#[doc = "Field `BIT_HQQ_EMPTY` reader - When this bit is raised, High queue is empty. Report by MAC."]
pub type BitHqqEmptyR = crate::BitReader;
#[doc = "Field `BIT_BCNQ_EMPTY` reader - When this bit is raised, beacon queue is empty. Report by MAC."]
pub type BitBcnqEmptyR = crate::BitReader;
#[doc = "Field `BIT_EVTQ_EMPTY` reader - When this bit is raised, event queue is empty. Report by MAC."]
pub type BitEvtqEmptyR = crate::BitReader;
#[doc = "Field `BIT_BCNQ_EMPTY_P0` reader - When this bit is raised, beacon queue of PORT0 is empty. Report by MAC."]
pub type BitBcnqEmptyP0R = crate::BitReader;
#[doc = "Field `BIT_BCNQ_EMPTY_P1` reader - When this bit is raised, beacon queue of PORT1 is empty. Report by MAC."]
pub type BitBcnqEmptyP1R = crate::BitReader;
#[doc = "Field `BIT_BCNQ_EMPTY_P2` reader - When this bit is raised, beacon queue of PORT2 is empty. Report by MAC."]
pub type BitBcnqEmptyP2R = crate::BitReader;
impl R {
    #[doc = "Bits 0:7 - The head page of the first packet of Bcnq"]
    #[inline(always)]
    pub fn bit_bcnq_head_pg(&self) -> BitBcnqHeadPgR {
        BitBcnqHeadPgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The total BcnQ pkt number in TxPktbuffer"]
    #[inline(always)]
    pub fn bit_pkt_num_bcnq(&self) -> BitPktNumBcnqR {
        BitPktNumBcnqR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - When this bit is raised, AC0 queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac0q_empty(&self) -> BitAc0qEmptyR {
        BitAc0qEmptyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When this bit is raised, AC1 queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac1q_empty(&self) -> BitAc1qEmptyR {
        BitAc1qEmptyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - When this bit is raised, AC2 queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac2q_empty(&self) -> BitAc2qEmptyR {
        BitAc2qEmptyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - When this bit is raised, AC3 queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac3q_empty(&self) -> BitAc3qEmptyR {
        BitAc3qEmptyR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - When this bit is raised, AC4 queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac4q_empty(&self) -> BitAc4qEmptyR {
        BitAc4qEmptyR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - When this bit is raised, AC5 queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac5q_empty(&self) -> BitAc5qEmptyR {
        BitAc5qEmptyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - When this bit is raised, AC6 queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac6q_empty(&self) -> BitAc6qEmptyR {
        BitAc6qEmptyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - When this bit is raised, AC7 queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_ac7q_empty(&self) -> BitAc7qEmptyR {
        BitAc7qEmptyR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - When this bit is raised, MGQ queue poll by cpu is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_mgq_cpu_empty(&self) -> BitMgqCpuEmptyR {
        BitMgqCpuEmptyR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - When this bit is raised, MGQ queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_mqq_empty(&self) -> BitMqqEmptyR {
        BitMqqEmptyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - When this bit is raised, High queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_hqq_empty(&self) -> BitHqqEmptyR {
        BitHqqEmptyR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - When this bit is raised, beacon queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_bcnq_empty(&self) -> BitBcnqEmptyR {
        BitBcnqEmptyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - When this bit is raised, event queue is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_evtq_empty(&self) -> BitEvtqEmptyR {
        BitEvtqEmptyR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - When this bit is raised, beacon queue of PORT0 is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_bcnq_empty_p0(&self) -> BitBcnqEmptyP0R {
        BitBcnqEmptyP0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - When this bit is raised, beacon queue of PORT1 is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_bcnq_empty_p1(&self) -> BitBcnqEmptyP1R {
        BitBcnqEmptyP1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When this bit is raised, beacon queue of PORT2 is empty. Report by MAC."]
    #[inline(always)]
    pub fn bit_bcnq_empty_p2(&self) -> BitBcnqEmptyP2R {
        BitBcnqEmptyP2R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_que_empty_bcnq_info::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegQueEmptyBcnqInfoSpec;
impl crate::RegisterSpec for RegQueEmptyBcnqInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_que_empty_bcnq_info::R`](R) reader structure"]
impl crate::Readable for RegQueEmptyBcnqInfoSpec {}
#[doc = "`reset()` method sets REG_QUE_EMPTY_BCNQ_INFO to value 0xffff_00f6"]
impl crate::Resettable for RegQueEmptyBcnqInfoSpec {
    const RESET_VALUE: u32 = 0xffff_00f6;
}
