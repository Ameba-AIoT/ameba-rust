#[doc = "Register `REG_TXOPOWNER_L` reader"]
pub type R = crate::R<RegTxopownerLSpec>;
#[doc = "Register `REG_TXOPOWNER_L` writer"]
pub type W = crate::W<RegTxopownerLSpec>;
#[doc = "Field `r_txopowner_addr_l` reader - txopowner mac address Register BIT\\[31:0\\]"]
pub type RTxopownerAddrLR = crate::FieldReader<u32>;
#[doc = "Field `r_txopowner_addr_l` writer - txopowner mac address Register BIT\\[31:0\\]"]
pub type RTxopownerAddrLW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - txopowner mac address Register BIT\\[31:0\\]"]
    #[inline(always)]
    pub fn r_txopowner_addr_l(&self) -> RTxopownerAddrLR {
        RTxopownerAddrLR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - txopowner mac address Register BIT\\[31:0\\]"]
    #[inline(always)]
    pub fn r_txopowner_addr_l(&mut self) -> RTxopownerAddrLW<'_, RegTxopownerLSpec> {
        RTxopownerAddrLW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_txopowner_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_txopowner_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegTxopownerLSpec;
impl crate::RegisterSpec for RegTxopownerLSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_txopowner_l::R`](R) reader structure"]
impl crate::Readable for RegTxopownerLSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_txopowner_l::W`](W) writer structure"]
impl crate::Writable for RegTxopownerLSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_TXOPOWNER_L to value 0x0008_0010"]
impl crate::Resettable for RegTxopownerLSpec {
    const RESET_VALUE: u32 = 0x0008_0010;
}
