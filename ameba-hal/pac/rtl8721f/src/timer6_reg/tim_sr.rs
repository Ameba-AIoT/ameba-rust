#[doc = "Register `TIM_SR` reader"]
pub type R = crate::R<TimSrSpec>;
#[doc = "Field `UIF` reader - update interrupt flag"]
pub type UifR = crate::BitReader;
#[doc = "Field `CC0IF` reader - Capture/Compare 0 interrupt flag channel CC0 configured as output: This flag is set by hardware when the counter TIM_CNT value matches the value of TIM_CCR0 register. It is cleared by software. Channel CC0 configured as input: This flag is set by hardware and the counter value has been captured in CCR0 field of TIM_CCR0 register when an active edge has been detected. It is cleared by software."]
pub type Cc0ifR = crate::BitReader;
#[doc = "Field `CC1IF` reader - refer to CC0IF description"]
pub type Cc1ifR = crate::BitReader;
#[doc = "Field `CC2IF` reader - refer to CC0IF description"]
pub type Cc2ifR = crate::BitReader;
#[doc = "Field `CC3IF` reader - refer to CC0IF description"]
pub type Cc3ifR = crate::BitReader;
#[doc = "Field `UIF0` reader - update interrupt flag of channel0 This flag is set by hardware when the UIE0 bit in the TIMx_EGR register is set, and update interrupt of channel0 happens."]
pub type Uif0R = crate::BitReader;
#[doc = "Field `UIF1` reader - refer to UIF0 description"]
pub type Uif1R = crate::BitReader;
#[doc = "Field `UIF2` reader - refer to UIF0 description"]
pub type Uif2R = crate::BitReader;
#[doc = "Field `UIF3` reader - refer to UIF0 description"]
pub type Uif3R = crate::BitReader;
#[doc = "Field `UG_DONE` reader - UG operation status This bit is cleared by hardware when the UG bit in the TIMx_EGR register is set. When the UG operation is done, hardware set this bit to '1'. So, software can poll this bit to see the UG operation status."]
pub type UgDoneR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - update interrupt flag"]
    #[inline(always)]
    pub fn uif(&self) -> UifR {
        UifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture/Compare 0 interrupt flag channel CC0 configured as output: This flag is set by hardware when the counter TIM_CNT value matches the value of TIM_CCR0 register. It is cleared by software. Channel CC0 configured as input: This flag is set by hardware and the counter value has been captured in CCR0 field of TIM_CCR0 register when an active edge has been detected. It is cleared by software."]
    #[inline(always)]
    pub fn cc0if(&self) -> Cc0ifR {
        Cc0ifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - refer to CC0IF description"]
    #[inline(always)]
    pub fn cc1if(&self) -> Cc1ifR {
        Cc1ifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - refer to CC0IF description"]
    #[inline(always)]
    pub fn cc2if(&self) -> Cc2ifR {
        Cc2ifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - refer to CC0IF description"]
    #[inline(always)]
    pub fn cc3if(&self) -> Cc3ifR {
        Cc3ifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - update interrupt flag of channel0 This flag is set by hardware when the UIE0 bit in the TIMx_EGR register is set, and update interrupt of channel0 happens."]
    #[inline(always)]
    pub fn uif0(&self) -> Uif0R {
        Uif0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - refer to UIF0 description"]
    #[inline(always)]
    pub fn uif1(&self) -> Uif1R {
        Uif1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - refer to UIF0 description"]
    #[inline(always)]
    pub fn uif2(&self) -> Uif2R {
        Uif2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - refer to UIF0 description"]
    #[inline(always)]
    pub fn uif3(&self) -> Uif3R {
        Uif3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 31 - UG operation status This bit is cleared by hardware when the UG bit in the TIMx_EGR register is set. When the UG operation is done, hardware set this bit to '1'. So, software can poll this bit to see the UG operation status."]
    #[inline(always)]
    pub fn ug_done(&self) -> UgDoneR {
        UgDoneR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "TIM status register\n\nYou can [`read`](crate::Reg::read) this register and get [`tim_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimSrSpec;
impl crate::RegisterSpec for TimSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim_sr::R`](R) reader structure"]
impl crate::Readable for TimSrSpec {}
#[doc = "`reset()` method sets TIM_SR to value 0x8000_0000"]
impl crate::Resettable for TimSrSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
