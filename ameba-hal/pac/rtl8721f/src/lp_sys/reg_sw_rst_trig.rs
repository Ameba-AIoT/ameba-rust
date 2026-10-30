#[doc = "Register `REG_SW_RST_TRIG` reader"]
pub type R = crate::R<RegSwRstTrigSpec>;
#[doc = "Register `REG_SW_RST_TRIG` writer"]
pub type W = crate::W<RegSwRstTrigSpec>;
#[doc = "Field `trig_rst` reader - System reset trigger register Write 0x96969696 to enable the write of reset control Write 0x69696969 will trigger the system if REG_SW_RST_CTRL set ."]
pub type TrigRstR = crate::FieldReader<u32>;
#[doc = "Field `trig_rst` writer - System reset trigger register Write 0x96969696 to enable the write of reset control Write 0x69696969 will trigger the system if REG_SW_RST_CTRL set ."]
pub type TrigRstW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - System reset trigger register Write 0x96969696 to enable the write of reset control Write 0x69696969 will trigger the system if REG_SW_RST_CTRL set ."]
    #[inline(always)]
    pub fn trig_rst(&self) -> TrigRstR {
        TrigRstR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - System reset trigger register Write 0x96969696 to enable the write of reset control Write 0x69696969 will trigger the system if REG_SW_RST_CTRL set ."]
    #[inline(always)]
    pub fn trig_rst(&mut self) -> TrigRstW<'_, RegSwRstTrigSpec> {
        TrigRstW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_sw_rst_trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_sw_rst_trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegSwRstTrigSpec;
impl crate::RegisterSpec for RegSwRstTrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_sw_rst_trig::R`](R) reader structure"]
impl crate::Readable for RegSwRstTrigSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_sw_rst_trig::W`](W) writer structure"]
impl crate::Writable for RegSwRstTrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_SW_RST_TRIG to value 0"]
impl crate::Resettable for RegSwRstTrigSpec {}
