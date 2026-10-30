#[doc = "Register `CAN_TEST` reader"]
pub type R = crate::R<CanTestSpec>;
#[doc = "Register `CAN_TEST` writer"]
pub type W = crate::W<CanTestSpec>;
#[doc = "Field `can_test_cfg` reader - 2b'00: silence_mode 2b'01: external loopback mode (enable can_tx_so ) 2b'10: internal loopback mode (can_tx_so tie 1) 2b'11: reserved"]
pub type CanTestCfgR = crate::FieldReader;
#[doc = "Field `can_test_cfg` writer - 2b'00: silence_mode 2b'01: external loopback mode (enable can_tx_so ) 2b'10: internal loopback mode (can_tx_so tie 1) 2b'11: reserved"]
pub type CanTestCfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - 2b'00: silence_mode 2b'01: external loopback mode (enable can_tx_so ) 2b'10: internal loopback mode (can_tx_so tie 1) 2b'11: reserved"]
    #[inline(always)]
    pub fn can_test_cfg(&self) -> CanTestCfgR {
        CanTestCfgR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - 2b'00: silence_mode 2b'01: external loopback mode (enable can_tx_so ) 2b'10: internal loopback mode (can_tx_so tie 1) 2b'11: reserved"]
    #[inline(always)]
    pub fn can_test_cfg(&mut self) -> CanTestCfgW<'_, CanTestSpec> {
        CanTestCfgW::new(self, 0)
    }
}
#[doc = "test configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`can_test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanTestSpec;
impl crate::RegisterSpec for CanTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_test::R`](R) reader structure"]
impl crate::Readable for CanTestSpec {}
#[doc = "`write(|w| ..)` method takes [`can_test::W`](W) writer structure"]
impl crate::Writable for CanTestSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_TEST to value 0"]
impl crate::Resettable for CanTestSpec {}
