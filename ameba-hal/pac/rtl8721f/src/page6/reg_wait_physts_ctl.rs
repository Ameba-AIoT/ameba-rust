#[doc = "Register `REG_WAIT_PHYSTS_CTL` reader"]
pub type R = crate::R<RegWaitPhystsCtlSpec>;
#[doc = "Register `REG_WAIT_PHYSTS_CTL` writer"]
pub type W = crate::W<RegWaitPhystsCtlSpec>;
#[doc = "Field `BIT_WAIT_PHYSTS_TIME` reader - Wait physts rpt timeout ,unit us(6us is the recommeded value)"]
pub type BitWaitPhystsTimeR = crate::FieldReader;
#[doc = "Field `BIT_WAIT_PHYSTS_TIME` writer - Wait physts rpt timeout ,unit us(6us is the recommeded value)"]
pub type BitWaitPhystsTimeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BIT_WAIT_PHYSTS_EN` reader - When set to 1,enable wait physts rpt valid function"]
pub type BitWaitPhystsEnR = crate::BitReader;
#[doc = "Field `BIT_WAIT_PHYSTS_EN` writer - When set to 1,enable wait physts rpt valid function"]
pub type BitWaitPhystsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_RDRDY_NUM_CHK_EN` reader - When set, enable check rdrdy number is enough or not. If number is not enough will rst macrx"]
pub type BitRdrdyNumChkEnR = crate::BitReader;
#[doc = "Field `BIT_RDRDY_NUM_CHK_EN` writer - When set, enable check rdrdy number is enough or not. If number is not enough will rst macrx"]
pub type BitRdrdyNumChkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_CCA_SPOOFING_EN` reader - When set to 1,enable cca_spoofing reset macrx"]
pub type BitCcaSpoofingEnR = crate::BitReader;
#[doc = "Field `BIT_CCA_SPOOFING_EN` writer - When set to 1,enable cca_spoofing reset macrx"]
pub type BitCcaSpoofingEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BB_RST_RESP` reader - When set to 1,enable bb/rdrdy_err/cca_spoofing to reset mac and will not tx response"]
pub type BitBbRstRespR = crate::BitReader;
#[doc = "Field `BIT_BB_RST_RESP` writer - When set to 1,enable bb/rdrdy_err/cca_spoofing to reset mac and will not tx response"]
pub type BitBbRstRespW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Wait physts rpt timeout ,unit us(6us is the recommeded value)"]
    #[inline(always)]
    pub fn bit_wait_physts_time(&self) -> BitWaitPhystsTimeR {
        BitWaitPhystsTimeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - When set to 1,enable wait physts rpt valid function"]
    #[inline(always)]
    pub fn bit_wait_physts_en(&self) -> BitWaitPhystsEnR {
        BitWaitPhystsEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When set, enable check rdrdy number is enough or not. If number is not enough will rst macrx"]
    #[inline(always)]
    pub fn bit_rdrdy_num_chk_en(&self) -> BitRdrdyNumChkEnR {
        BitRdrdyNumChkEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set to 1,enable cca_spoofing reset macrx"]
    #[inline(always)]
    pub fn bit_cca_spoofing_en(&self) -> BitCcaSpoofingEnR {
        BitCcaSpoofingEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set to 1,enable bb/rdrdy_err/cca_spoofing to reset mac and will not tx response"]
    #[inline(always)]
    pub fn bit_bb_rst_resp(&self) -> BitBbRstRespR {
        BitBbRstRespR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Wait physts rpt timeout ,unit us(6us is the recommeded value)"]
    #[inline(always)]
    pub fn bit_wait_physts_time(&mut self) -> BitWaitPhystsTimeW<'_, RegWaitPhystsCtlSpec> {
        BitWaitPhystsTimeW::new(self, 0)
    }
    #[doc = "Bit 4 - When set to 1,enable wait physts rpt valid function"]
    #[inline(always)]
    pub fn bit_wait_physts_en(&mut self) -> BitWaitPhystsEnW<'_, RegWaitPhystsCtlSpec> {
        BitWaitPhystsEnW::new(self, 4)
    }
    #[doc = "Bit 5 - When set, enable check rdrdy number is enough or not. If number is not enough will rst macrx"]
    #[inline(always)]
    pub fn bit_rdrdy_num_chk_en(&mut self) -> BitRdrdyNumChkEnW<'_, RegWaitPhystsCtlSpec> {
        BitRdrdyNumChkEnW::new(self, 5)
    }
    #[doc = "Bit 6 - When set to 1,enable cca_spoofing reset macrx"]
    #[inline(always)]
    pub fn bit_cca_spoofing_en(&mut self) -> BitCcaSpoofingEnW<'_, RegWaitPhystsCtlSpec> {
        BitCcaSpoofingEnW::new(self, 6)
    }
    #[doc = "Bit 7 - When set to 1,enable bb/rdrdy_err/cca_spoofing to reset mac and will not tx response"]
    #[inline(always)]
    pub fn bit_bb_rst_resp(&mut self) -> BitBbRstRespW<'_, RegWaitPhystsCtlSpec> {
        BitBbRstRespW::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wait_physts_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wait_physts_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWaitPhystsCtlSpec;
impl crate::RegisterSpec for RegWaitPhystsCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wait_physts_ctl::R`](R) reader structure"]
impl crate::Readable for RegWaitPhystsCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wait_physts_ctl::W`](W) writer structure"]
impl crate::Writable for RegWaitPhystsCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WAIT_PHYSTS_CTL to value 0x24"]
impl crate::Resettable for RegWaitPhystsCtlSpec {
    const RESET_VALUE: u32 = 0x24;
}
