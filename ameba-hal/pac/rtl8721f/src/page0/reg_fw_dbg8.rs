#[doc = "Register `REG_FW_DBG8` reader"]
pub type R = crate::R<RegFwDbg8Spec>;
#[doc = "Register `REG_FW_DBG8` writer"]
pub type W = crate::W<RegFwDbg8Spec>;
#[doc = "Field `BIT_BLK_TST` reader - fw debug8"]
pub type BitBlkTstR = crate::FieldReader<u32>;
#[doc = "Field `BIT_BLK_TST` writer - fw debug8"]
pub type BitBlkTstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - fw debug8"]
    #[inline(always)]
    pub fn bit_blk_tst(&self) -> BitBlkTstR {
        BitBlkTstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - fw debug8"]
    #[inline(always)]
    pub fn bit_blk_tst(&mut self) -> BitBlkTstW<'_, RegFwDbg8Spec> {
        BitBlkTstW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fw_dbg8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fw_dbg8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFwDbg8Spec;
impl crate::RegisterSpec for RegFwDbg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fw_dbg8::R`](R) reader structure"]
impl crate::Readable for RegFwDbg8Spec {}
#[doc = "`write(|w| ..)` method takes [`reg_fw_dbg8::W`](W) writer structure"]
impl crate::Writable for RegFwDbg8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FW_DBG8 to value 0"]
impl crate::Resettable for RegFwDbg8Spec {}
