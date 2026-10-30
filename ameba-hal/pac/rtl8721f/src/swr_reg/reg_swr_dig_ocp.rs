#[doc = "Register `REG_SWR_DIG_OCP` reader"]
pub type R = crate::R<RegSwrDigOcpSpec>;
#[doc = "Register `REG_SWR_DIG_OCP` writer"]
pub type W = crate::W<RegSwrDigOcpSpec>;
#[doc = "Field `SPS_DIG_OCP_WND` reader - SPS OCP time window"]
pub type SpsDigOcpWndR = crate::FieldReader<u16>;
#[doc = "Field `SPS_DIG_OCP_WND` writer - SPS OCP time window"]
pub type SpsDigOcpWndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SPS_DIG_OCP_THD` reader - SPS OCP event threshold"]
pub type SpsDigOcpThdR = crate::FieldReader<u16>;
#[doc = "Field `SPS_DIG_OCP_THD` writer - SPS OCP event threshold"]
pub type SpsDigOcpThdW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `SPS_DIG_OCP_CKE` reader - SPS OCP clock enable,used as function enable"]
pub type SpsDigOcpCkeR = crate::BitReader;
#[doc = "Field `SPS_DIG_OCP_CKE` writer - SPS OCP clock enable,used as function enable"]
pub type SpsDigOcpCkeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - SPS OCP time window"]
    #[inline(always)]
    pub fn sps_dig_ocp_wnd(&self) -> SpsDigOcpWndR {
        SpsDigOcpWndR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:30 - SPS OCP event threshold"]
    #[inline(always)]
    pub fn sps_dig_ocp_thd(&self) -> SpsDigOcpThdR {
        SpsDigOcpThdR::new(((self.bits >> 16) & 0x7fff) as u16)
    }
    #[doc = "Bit 31 - SPS OCP clock enable,used as function enable"]
    #[inline(always)]
    pub fn sps_dig_ocp_cke(&self) -> SpsDigOcpCkeR {
        SpsDigOcpCkeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - SPS OCP time window"]
    #[inline(always)]
    pub fn sps_dig_ocp_wnd(&mut self) -> SpsDigOcpWndW<'_, RegSwrDigOcpSpec> {
        SpsDigOcpWndW::new(self, 0)
    }
    #[doc = "Bits 16:30 - SPS OCP event threshold"]
    #[inline(always)]
    pub fn sps_dig_ocp_thd(&mut self) -> SpsDigOcpThdW<'_, RegSwrDigOcpSpec> {
        SpsDigOcpThdW::new(self, 16)
    }
    #[doc = "Bit 31 - SPS OCP clock enable,used as function enable"]
    #[inline(always)]
    pub fn sps_dig_ocp_cke(&mut self) -> SpsDigOcpCkeW<'_, RegSwrDigOcpSpec> {
        SpsDigOcpCkeW::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_swr_dig_ocp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_swr_dig_ocp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwrDigOcpSpec;
impl crate::RegisterSpec for RegSwrDigOcpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_swr_dig_ocp::R`](R) reader structure"]
impl crate::Readable for RegSwrDigOcpSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_swr_dig_ocp::W`](W) writer structure"]
impl crate::Writable for RegSwrDigOcpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SWR_DIG_OCP to value 0"]
impl crate::Resettable for RegSwrDigOcpSpec {}
