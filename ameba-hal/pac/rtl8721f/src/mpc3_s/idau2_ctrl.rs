#[doc = "Register `IDAU2_CTRL` reader"]
pub type R = crate::R<Idau2CtrlSpec>;
#[doc = "Register `IDAU2_CTRL` writer"]
pub type W = crate::W<Idau2CtrlSpec>;
#[doc = "Field `IDAU2_SECURE_CTRL` reader - Region entry enable After setting SECURE_CTRL bits as 1, corresponding regions is set as non-secure.Note that if SECURE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[0\\]: enable entry for ba0/la0 0: disable 1: enable \\[1\\]: enable entry for ba1/la1 0: disable 1: enable \\[2\\]: enable entry for ba2/la2 0: disable 1: enable \\[3\\]: enable entry for ba3/la3 0: disable 1: enable \\[4\\]: enable entry for ba4/la4 0: disable 1: enable \\[5\\]: enable entry for ba5/la5 0: disable 1: enable"]
pub type Idau2SecureCtrlR = crate::FieldReader;
#[doc = "Field `IDAU2_SECURE_CTRL` writer - Region entry enable After setting SECURE_CTRL bits as 1, corresponding regions is set as non-secure.Note that if SECURE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[0\\]: enable entry for ba0/la0 0: disable 1: enable \\[1\\]: enable entry for ba1/la1 0: disable 1: enable \\[2\\]: enable entry for ba2/la2 0: disable 1: enable \\[3\\]: enable entry for ba3/la3 0: disable 1: enable \\[4\\]: enable entry for ba4/la4 0: disable 1: enable \\[5\\]: enable entry for ba5/la5 0: disable 1: enable"]
pub type Idau2SecureCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `IDAU2_WRITE_CTRL` reader - Region entry enable After setting WRITE_CTRL bits as 1, corresponding regions is set as write-forbidden.Note that if WRITE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[16\\]: enable entry for ba0/la0 0: disable 1: enable \\[17\\]: enable entry for ba1/la1 0: disable 1: enable \\[18\\]: enable entry for ba2/la2 0: disable 1: enable \\[19\\]: enable entry for ba3/la3 0: disable 1: enable \\[20\\]: enable entry for ba4/la4 0: disable 1: enable \\[21\\]: enable entry for ba5/la5 0: disable 1: enable"]
pub type Idau2WriteCtrlR = crate::FieldReader;
#[doc = "Field `IDAU2_WRITE_CTRL` writer - Region entry enable After setting WRITE_CTRL bits as 1, corresponding regions is set as write-forbidden.Note that if WRITE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[16\\]: enable entry for ba0/la0 0: disable 1: enable \\[17\\]: enable entry for ba1/la1 0: disable 1: enable \\[18\\]: enable entry for ba2/la2 0: disable 1: enable \\[19\\]: enable entry for ba3/la3 0: disable 1: enable \\[20\\]: enable entry for ba4/la4 0: disable 1: enable \\[21\\]: enable entry for ba5/la5 0: disable 1: enable"]
pub type Idau2WriteCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Region entry enable After setting SECURE_CTRL bits as 1, corresponding regions is set as non-secure.Note that if SECURE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[0\\]: enable entry for ba0/la0 0: disable 1: enable \\[1\\]: enable entry for ba1/la1 0: disable 1: enable \\[2\\]: enable entry for ba2/la2 0: disable 1: enable \\[3\\]: enable entry for ba3/la3 0: disable 1: enable \\[4\\]: enable entry for ba4/la4 0: disable 1: enable \\[5\\]: enable entry for ba5/la5 0: disable 1: enable"]
    #[inline(always)]
    pub fn idau2_secure_ctrl(&self) -> Idau2SecureCtrlR {
        Idau2SecureCtrlR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Region entry enable After setting WRITE_CTRL bits as 1, corresponding regions is set as write-forbidden.Note that if WRITE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[16\\]: enable entry for ba0/la0 0: disable 1: enable \\[17\\]: enable entry for ba1/la1 0: disable 1: enable \\[18\\]: enable entry for ba2/la2 0: disable 1: enable \\[19\\]: enable entry for ba3/la3 0: disable 1: enable \\[20\\]: enable entry for ba4/la4 0: disable 1: enable \\[21\\]: enable entry for ba5/la5 0: disable 1: enable"]
    #[inline(always)]
    pub fn idau2_write_ctrl(&self) -> Idau2WriteCtrlR {
        Idau2WriteCtrlR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Region entry enable After setting SECURE_CTRL bits as 1, corresponding regions is set as non-secure.Note that if SECURE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[0\\]: enable entry for ba0/la0 0: disable 1: enable \\[1\\]: enable entry for ba1/la1 0: disable 1: enable \\[2\\]: enable entry for ba2/la2 0: disable 1: enable \\[3\\]: enable entry for ba3/la3 0: disable 1: enable \\[4\\]: enable entry for ba4/la4 0: disable 1: enable \\[5\\]: enable entry for ba5/la5 0: disable 1: enable"]
    #[inline(always)]
    pub fn idau2_secure_ctrl(&mut self) -> Idau2SecureCtrlW<'_, Idau2CtrlSpec> {
        Idau2SecureCtrlW::new(self, 0)
    }
    #[doc = "Bits 16:21 - Region entry enable After setting WRITE_CTRL bits as 1, corresponding regions is set as write-forbidden.Note that if WRITE_CTRL bits are enabled, the region remains secure even if ba*i/la*i is set. \\[16\\]: enable entry for ba0/la0 0: disable 1: enable \\[17\\]: enable entry for ba1/la1 0: disable 1: enable \\[18\\]: enable entry for ba2/la2 0: disable 1: enable \\[19\\]: enable entry for ba3/la3 0: disable 1: enable \\[20\\]: enable entry for ba4/la4 0: disable 1: enable \\[21\\]: enable entry for ba5/la5 0: disable 1: enable"]
    #[inline(always)]
    pub fn idau2_write_ctrl(&mut self) -> Idau2WriteCtrlW<'_, Idau2CtrlSpec> {
        Idau2WriteCtrlW::new(self, 16)
    }
}
#[doc = "Enable control register in IDAU2\n\nYou can [`read`](crate::Reg::read) this register and get [`idau2_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idau2_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Idau2CtrlSpec;
impl crate::RegisterSpec for Idau2CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idau2_ctrl::R`](R) reader structure"]
impl crate::Readable for Idau2CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`idau2_ctrl::W`](W) writer structure"]
impl crate::Writable for Idau2CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IDAU2_CTRL to value 0"]
impl crate::Resettable for Idau2CtrlSpec {}
