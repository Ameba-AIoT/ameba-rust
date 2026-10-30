#[doc = "Register `REG_32K_CAL_REG0` reader"]
pub type R = crate::R<Reg32kCalReg0Spec>;
#[doc = "Register `REG_32K_CAL_REG0` writer"]
pub type W = crate::W<Reg32kCalReg0Spec>;
#[doc = "Field `BIT_RXBCN_TIMEUP_THRES_INIT` reader - HW auto update threshold for rxbcn timeout,UNIT:us"]
pub type BitRxbcnTimeupThresInitR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RXBCN_TIMEUP_THRES` reader - threshold for rxbcn timeout,UNIT:us default:8ms"]
pub type BitRxbcnTimeupThresR = crate::FieldReader<u16>;
#[doc = "Field `BIT_RXBCN_TIMEUP_THRES` writer - threshold for rxbcn timeout,UNIT:us default:8ms"]
pub type BitRxbcnTimeupThresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - HW auto update threshold for rxbcn timeout,UNIT:us"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_thres_init(&self) -> BitRxbcnTimeupThresInitR {
        BitRxbcnTimeupThresInitR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - threshold for rxbcn timeout,UNIT:us default:8ms"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_thres(&self) -> BitRxbcnTimeupThresR {
        BitRxbcnTimeupThresR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - threshold for rxbcn timeout,UNIT:us default:8ms"]
    #[inline(always)]
    pub fn bit_rxbcn_timeup_thres(&mut self) -> BitRxbcnTimeupThresW<'_, Reg32kCalReg0Spec> {
        BitRxbcnTimeupThresW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_32k_cal_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_32k_cal_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Reg32kCalReg0Spec;
impl crate::RegisterSpec for Reg32kCalReg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_32k_cal_reg0::R`](R) reader structure"]
impl crate::Readable for Reg32kCalReg0Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_32k_cal_reg0::W`](W) writer structure"]
impl crate::Writable for Reg32kCalReg0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_32K_CAL_REG0 to value 0x1f40_0000"]
impl crate::Resettable for Reg32kCalReg0Spec {
    const RESET_VALUE: u32 = 0x1f40_0000;
}
