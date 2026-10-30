#[doc = "Register `REG_PHY_INFO` reader"]
pub type R = crate::R<RegPhyInfoSpec>;
#[doc = "Register `REG_PHY_INFO` writer"]
pub type W = crate::W<RegPhyInfoSpec>;
#[doc = "Field `CMD_INFO` reader - Command information, 1st Byte of Command/Address transaction"]
pub type CmdInfoR = crate::FieldReader;
#[doc = "Field `RX_TIME_OUT` reader - read time out interrupt if there's no DQS/DQ from psram, write 1 to clear"]
pub type RxTimeOutR = crate::BitReader;
#[doc = "Field `RX_TIME_OUT_MASK` reader - read time out interrupt mask"]
pub type RxTimeOutMaskR = crate::BitReader;
#[doc = "Field `RX_TIME_OUT_MASK` writer - read time out interrupt mask"]
pub type RxTimeOutMaskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Command information, 1st Byte of Command/Address transaction"]
    #[inline(always)]
    pub fn cmd_info(&self) -> CmdInfoR {
        CmdInfoR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 16 - read time out interrupt if there's no DQS/DQ from psram, write 1 to clear"]
    #[inline(always)]
    pub fn rx_time_out(&self) -> RxTimeOutR {
        RxTimeOutR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - read time out interrupt mask"]
    #[inline(always)]
    pub fn rx_time_out_mask(&self) -> RxTimeOutMaskR {
        RxTimeOutMaskR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - read time out interrupt mask"]
    #[inline(always)]
    pub fn rx_time_out_mask(&mut self) -> RxTimeOutMaskW<'_, RegPhyInfoSpec> {
        RxTimeOutMaskW::new(self, 20)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_phy_info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_phy_info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegPhyInfoSpec;
impl crate::RegisterSpec for RegPhyInfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_phy_info::R`](R) reader structure"]
impl crate::Readable for RegPhyInfoSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_phy_info::W`](W) writer structure"]
impl crate::Writable for RegPhyInfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_PHY_INFO to value 0x0010_0000"]
impl crate::Resettable for RegPhyInfoSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
