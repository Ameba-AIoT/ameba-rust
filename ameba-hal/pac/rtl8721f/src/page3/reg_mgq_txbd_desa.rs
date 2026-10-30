#[doc = "Register `REG_MGQ_TXBD_DESA` reader"]
pub type R = crate::R<RegMgqTxbdDesaSpec>;
#[doc = "Register `REG_MGQ_TXBD_DESA` writer"]
pub type W = crate::W<RegMgqTxbdDesaSpec>;
#[doc = "Field `BIT_MGQDESA` reader - Buffer Descriptor Base Address for Manage Queue"]
pub type BitMgqdesaR = crate::FieldReader<u32>;
#[doc = "Field `BIT_MGQDESA` writer - Buffer Descriptor Base Address for Manage Queue"]
pub type BitMgqdesaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for Manage Queue"]
    #[inline(always)]
    pub fn bit_mgqdesa(&self) -> BitMgqdesaR {
        BitMgqdesaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer Descriptor Base Address for Manage Queue"]
    #[inline(always)]
    pub fn bit_mgqdesa(&mut self) -> BitMgqdesaW<'_, RegMgqTxbdDesaSpec> {
        BitMgqdesaW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_mgq_txbd_desa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_mgq_txbd_desa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegMgqTxbdDesaSpec;
impl crate::RegisterSpec for RegMgqTxbdDesaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_mgq_txbd_desa::R`](R) reader structure"]
impl crate::Readable for RegMgqTxbdDesaSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_mgq_txbd_desa::W`](W) writer structure"]
impl crate::Writable for RegMgqTxbdDesaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_MGQ_TXBD_DESA to value 0"]
impl crate::Resettable for RegMgqTxbdDesaSpec {}
