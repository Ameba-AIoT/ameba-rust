#[doc = "Register `IDAU_CTRL` reader"]
pub type R = crate::R<IdauCtrlSpec>;
#[doc = "Register `IDAU_CTRL` writer"]
pub type W = crate::W<IdauCtrlSpec>;
#[doc = "Field `IDAU_SECURE_CTRL` reader - Region entry enable After setting SECURE_CTRL bits as 1, corresponding regions is set as non-secure.Note that if SECURE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[0\\]: enable entry for ba0/la0 0: disable 1: enable \\[1\\]: enable entry for ba1/la1 0: disable 1: enable \\[2\\]: enable entry for ba2/la2 0: disable 1: enable \\[3\\]: enable entry for ba3/la3 0: disable 1: enable \\[4\\]: enable entry for ba4/la4 0: disable 1: enable \\[5\\]: enable entry for ba5/la5 0: disable 1: enable"]
pub type IdauSecureCtrlR = crate::FieldReader;
#[doc = "Field `IDAU_SECURE_CTRL` writer - Region entry enable After setting SECURE_CTRL bits as 1, corresponding regions is set as non-secure.Note that if SECURE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[0\\]: enable entry for ba0/la0 0: disable 1: enable \\[1\\]: enable entry for ba1/la1 0: disable 1: enable \\[2\\]: enable entry for ba2/la2 0: disable 1: enable \\[3\\]: enable entry for ba3/la3 0: disable 1: enable \\[4\\]: enable entry for ba4/la4 0: disable 1: enable \\[5\\]: enable entry for ba5/la5 0: disable 1: enable"]
pub type IdauSecureCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IDAU_WRITE_CTRL` reader - Region entry enable After setting WRITE_CTRL bits as 1, corresponding regions is set as write-forbidden.Note that if WRITE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[16\\]: enable entry for ba0/la0 0: disable 1: enable \\[17\\]: enable entry for ba1/la1 0: disable 1: enable \\[18\\]: enable entry for ba2/la2 0: disable 1: enable \\[19\\]: enable entry for ba3/la3 0: disable 1: enable \\[20\\]: enable entry for ba4/la4 0: disable 1: enable \\[21\\]: enable entry for ba5/la5 0: disable 1: enable"]
pub type IdauWriteCtrlR = crate::FieldReader;
#[doc = "Field `IDAU_WRITE_CTRL` writer - Region entry enable After setting WRITE_CTRL bits as 1, corresponding regions is set as write-forbidden.Note that if WRITE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[16\\]: enable entry for ba0/la0 0: disable 1: enable \\[17\\]: enable entry for ba1/la1 0: disable 1: enable \\[18\\]: enable entry for ba2/la2 0: disable 1: enable \\[19\\]: enable entry for ba3/la3 0: disable 1: enable \\[20\\]: enable entry for ba4/la4 0: disable 1: enable \\[21\\]: enable entry for ba5/la5 0: disable 1: enable"]
pub type IdauWriteCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Region entry enable After setting SECURE_CTRL bits as 1, corresponding regions is set as non-secure.Note that if SECURE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[0\\]: enable entry for ba0/la0 0: disable 1: enable \\[1\\]: enable entry for ba1/la1 0: disable 1: enable \\[2\\]: enable entry for ba2/la2 0: disable 1: enable \\[3\\]: enable entry for ba3/la3 0: disable 1: enable \\[4\\]: enable entry for ba4/la4 0: disable 1: enable \\[5\\]: enable entry for ba5/la5 0: disable 1: enable"]
    #[inline(always)]
    pub fn idau_secure_ctrl(&self) -> IdauSecureCtrlR {
        IdauSecureCtrlR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Region entry enable After setting WRITE_CTRL bits as 1, corresponding regions is set as write-forbidden.Note that if WRITE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[16\\]: enable entry for ba0/la0 0: disable 1: enable \\[17\\]: enable entry for ba1/la1 0: disable 1: enable \\[18\\]: enable entry for ba2/la2 0: disable 1: enable \\[19\\]: enable entry for ba3/la3 0: disable 1: enable \\[20\\]: enable entry for ba4/la4 0: disable 1: enable \\[21\\]: enable entry for ba5/la5 0: disable 1: enable"]
    #[inline(always)]
    pub fn idau_write_ctrl(&self) -> IdauWriteCtrlR {
        IdauWriteCtrlR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Region entry enable After setting SECURE_CTRL bits as 1, corresponding regions is set as non-secure.Note that if SECURE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[0\\]: enable entry for ba0/la0 0: disable 1: enable \\[1\\]: enable entry for ba1/la1 0: disable 1: enable \\[2\\]: enable entry for ba2/la2 0: disable 1: enable \\[3\\]: enable entry for ba3/la3 0: disable 1: enable \\[4\\]: enable entry for ba4/la4 0: disable 1: enable \\[5\\]: enable entry for ba5/la5 0: disable 1: enable"]
    #[inline(always)]
    pub fn idau_secure_ctrl(&mut self) -> IdauSecureCtrlW<'_, IdauCtrlSpec> {
        IdauSecureCtrlW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Region entry enable After setting WRITE_CTRL bits as 1, corresponding regions is set as write-forbidden.Note that if WRITE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[16\\]: enable entry for ba0/la0 0: disable 1: enable \\[17\\]: enable entry for ba1/la1 0: disable 1: enable \\[18\\]: enable entry for ba2/la2 0: disable 1: enable \\[19\\]: enable entry for ba3/la3 0: disable 1: enable \\[20\\]: enable entry for ba4/la4 0: disable 1: enable \\[21\\]: enable entry for ba5/la5 0: disable 1: enable"]
    #[inline(always)]
    pub fn idau_write_ctrl(&mut self) -> IdauWriteCtrlW<'_, IdauCtrlSpec> {
        IdauWriteCtrlW::new(self, 16)
    }
}
#[doc = "Enable control register in IDAU\n\nYou can [`read`](crate::Reg::read) this register and get [`idau_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IdauCtrlSpec;
impl crate::RegisterSpec for IdauCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau_ctrl::R`](R) reader structure"]
impl crate::Readable for IdauCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`idau_ctrl::W`](W) writer structure"]
impl crate::Writable for IdauCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU_CTRL to value 0"]
impl crate::Resettable for IdauCtrlSpec {}
