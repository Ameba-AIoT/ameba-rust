#[doc = "Register `REG_WL_MCUFW_CTRL` reader"]
pub type R = crate::R<RegWlMcufwCtrlSpec>;
#[doc = "Register `REG_WL_MCUFW_CTRL` writer"]
pub type W = crate::W<RegWlMcufwCtrlSpec>;
#[doc = "Field `BIT_FW_INIT_RDY` reader - CPU fw initialize ready, set by CPU(txrpt info update,used in ptcl)"]
pub type BitFwInitRdyR = crate::BitReader;
#[doc = "Field `BIT_FW_INIT_RDY` writer - CPU fw initialize ready, set by CPU(txrpt info update,used in ptcl)"]
pub type BitFwInitRdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 15 - CPU fw initialize ready, set by CPU(txrpt info update,used in ptcl)"]
    #[inline(always)]
    pub fn bit_fw_init_rdy(&self) -> BitFwInitRdyR {
        BitFwInitRdyR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - CPU fw initialize ready, set by CPU(txrpt info update,used in ptcl)"]
    #[inline(always)]
    pub fn bit_fw_init_rdy(&mut self) -> BitFwInitRdyW<'_, RegWlMcufwCtrlSpec> {
        BitFwInitRdyW::new(self, 15)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wl_mcufw_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wl_mcufw_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWlMcufwCtrlSpec;
impl crate::RegisterSpec for RegWlMcufwCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wl_mcufw_ctrl::R`](R) reader structure"]
impl crate::Readable for RegWlMcufwCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wl_mcufw_ctrl::W`](W) writer structure"]
impl crate::Writable for RegWlMcufwCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WL_MCUFW_CTRL to value 0"]
impl crate::Resettable for RegWlMcufwCtrlSpec {}
