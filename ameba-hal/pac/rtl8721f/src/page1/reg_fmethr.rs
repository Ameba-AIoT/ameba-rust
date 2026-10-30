#[doc = "Register `REG_FMETHR` reader"]
pub type R = crate::R<RegFmethrSpec>;
#[doc = "Register `REG_FMETHR` writer"]
pub type W = crate::W<RegFmethrSpec>;
#[doc = "Field `BIT_FW_MSG` reader - User Defined Message"]
pub type BitFwMsgR = crate::FieldReader<u32>;
#[doc = "Field `BIT_FW_MSG` writer - User Defined Message"]
pub type BitFwMsgW<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
#[doc = "Field `BIT_FWMSG_INTPS` reader - Toggle this bit will generate C2H interrupt to Host"]
pub type BitFwmsgIntpsR = crate::BitReader;
#[doc = "Field `BIT_FWMSG_INTPS` writer - Toggle this bit will generate C2H interrupt to Host"]
pub type BitFwmsgIntpsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:30 - User Defined Message"]
    #[inline(always)]
    pub fn bit_fw_msg(&self) -> BitFwMsgR {
        BitFwMsgR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - Toggle this bit will generate C2H interrupt to Host"]
    #[inline(always)]
    pub fn bit_fwmsg_intps(&self) -> BitFwmsgIntpsR {
        BitFwmsgIntpsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:30 - User Defined Message"]
    #[inline(always)]
    pub fn bit_fw_msg(&mut self) -> BitFwMsgW<'_, RegFmethrSpec> {
        BitFwMsgW::new(self, 0)
    }
    #[doc = "Bit 31 - Toggle this bit will generate C2H interrupt to Host"]
    #[inline(always)]
    pub fn bit_fwmsg_intps(&mut self) -> BitFwmsgIntpsW<'_, RegFmethrSpec> {
        BitFwmsgIntpsW::new(self, 31)
    }
}
#[doc = "Firmware Message Mail Box Register\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_fmethr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_fmethr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFmethrSpec;
impl crate::RegisterSpec for RegFmethrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_fmethr::R`](R) reader structure"]
impl crate::Readable for RegFmethrSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_fmethr::W`](W) writer structure"]
impl crate::Writable for RegFmethrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FMETHR to value 0"]
impl crate::Resettable for RegFmethrSpec {}
