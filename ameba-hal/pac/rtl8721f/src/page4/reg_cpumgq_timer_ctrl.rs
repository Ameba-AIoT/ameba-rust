#[doc = "Register `REG_CPUMGQ_TIMER_CTRL` reader"]
pub type R = crate::R<RegCpumgqTimerCtrlSpec>;
#[doc = "Register `REG_CPUMGQ_TIMER_CTRL` writer"]
pub type W = crate::W<RegCpumgqTimerCtrlSpec>;
#[doc = "Field `BIT_CPUMGQ_TRI_LIFETIME` reader - LIFETIME Threshold: Unit = 256us. When CPU MgntQ is triggered, If BIT_CPUMGQ_TRI_LIFETIME_EN = 1, count down. If BIT_CPUMGQ_TRI_LIFETIME_EN = 0, do nothing."]
pub type BitCpumgqTriLifetimeR = crate::FieldReader;
#[doc = "Field `BIT_CPUMGQ_TRI_LIFETIME` writer - LIFETIME Threshold: Unit = 256us. When CPU MgntQ is triggered, If BIT_CPUMGQ_TRI_LIFETIME_EN = 1, count down. If BIT_CPUMGQ_TRI_LIFETIME_EN = 0, do nothing."]
pub type BitCpumgqTriLifetimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_CPUMGQ_TRI_LIFETIME_EN` reader - When this bit is set, enable the lifetime function. This lifetime threshold only affect CPUMGQ trigger from Tx_Timer."]
pub type BitCpumgqTriLifetimeEnR = crate::BitReader;
#[doc = "Field `BIT_CPUMGQ_TRI_LIFETIME_EN` writer - When this bit is set, enable the lifetime function. This lifetime threshold only affect CPUMGQ trigger from Tx_Timer."]
pub type BitCpumgqTriLifetimeEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dummy_2` reader - "]
pub type Dummy2R = crate::FieldReader;
#[doc = "Field `dummy_2` writer - "]
pub type Dummy2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_MGQ_TRI_HEAD` reader - The head page of the CPUMGQ pkt If the CPUMGQ is triggered by TX TIMER."]
pub type BitMgqTriHeadR = crate::FieldReader<u16>;
#[doc = "Field `BIT_MGQ_TRI_HEAD` writer - The head page of the CPUMGQ pkt If the CPUMGQ is triggered by TX TIMER."]
pub type BitMgqTriHeadW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `dummy` reader - "]
pub type DummyR = crate::FieldReader;
#[doc = "Field `dummy` writer - "]
pub type DummyW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - LIFETIME Threshold: Unit = 256us. When CPU MgntQ is triggered, If BIT_CPUMGQ_TRI_LIFETIME_EN = 1, count down. If BIT_CPUMGQ_TRI_LIFETIME_EN = 0, do nothing."]
    #[inline(always)]
    pub fn bit_cpumgq_tri_lifetime(&self) -> BitCpumgqTriLifetimeR {
        BitCpumgqTriLifetimeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - When this bit is set, enable the lifetime function. This lifetime threshold only affect CPUMGQ trigger from Tx_Timer."]
    #[inline(always)]
    pub fn bit_cpumgq_tri_lifetime_en(&self) -> BitCpumgqTriLifetimeEnR {
        BitCpumgqTriLifetimeEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn dummy_2(&self) -> Dummy2R {
        Dummy2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - The head page of the CPUMGQ pkt If the CPUMGQ is triggered by TX TIMER."]
    #[inline(always)]
    pub fn bit_mgq_tri_head(&self) -> BitMgqTriHeadR {
        BitMgqTriHeadR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dummy(&self) -> DummyR {
        DummyR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LIFETIME Threshold: Unit = 256us. When CPU MgntQ is triggered, If BIT_CPUMGQ_TRI_LIFETIME_EN = 1, count down. If BIT_CPUMGQ_TRI_LIFETIME_EN = 0, do nothing."]
    #[inline(always)]
    pub fn bit_cpumgq_tri_lifetime(&mut self) -> BitCpumgqTriLifetimeW<'_, RegCpumgqTimerCtrlSpec> {
        BitCpumgqTriLifetimeW::new(self, 0)
    }
    #[doc = "Bit 8 - When this bit is set, enable the lifetime function. This lifetime threshold only affect CPUMGQ trigger from Tx_Timer."]
    #[inline(always)]
    pub fn bit_cpumgq_tri_lifetime_en(
        &mut self,
    ) -> BitCpumgqTriLifetimeEnW<'_, RegCpumgqTimerCtrlSpec> {
        BitCpumgqTriLifetimeEnW::new(self, 8)
    }
    #[doc = "Bits 9:15"]
    #[inline(always)]
    pub fn dummy_2(&mut self) -> Dummy2W<'_, RegCpumgqTimerCtrlSpec> {
        Dummy2W::new(self, 9)
    }
    #[doc = "Bits 16:27 - The head page of the CPUMGQ pkt If the CPUMGQ is triggered by TX TIMER."]
    #[inline(always)]
    pub fn bit_mgq_tri_head(&mut self) -> BitMgqTriHeadW<'_, RegCpumgqTimerCtrlSpec> {
        BitMgqTriHeadW::new(self, 16)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn dummy(&mut self) -> DummyW<'_, RegCpumgqTimerCtrlSpec> {
        DummyW::new(self, 28)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_cpumgq_timer_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_cpumgq_timer_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegCpumgqTimerCtrlSpec;
impl crate::RegisterSpec for RegCpumgqTimerCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_cpumgq_timer_ctrl::R`](R) reader structure"]
impl crate::Readable for RegCpumgqTimerCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_cpumgq_timer_ctrl::W`](W) writer structure"]
impl crate::Writable for RegCpumgqTimerCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CPUMGQ_TIMER_CTRL to value 0"]
impl crate::Resettable for RegCpumgqTimerCtrlSpec {}
