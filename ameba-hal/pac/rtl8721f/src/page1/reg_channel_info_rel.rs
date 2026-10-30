#[doc = "Register `REG_CHANNEL_INFO_REL` reader"]
pub type R = crate::R<RegChannelInfoRelSpec>;
#[doc = "Register `REG_CHANNEL_INFO_REL` writer"]
pub type W = crate::W<RegChannelInfoRelSpec>;
#[doc = "Field `BIT_CHINFO_POLL` writer - channel info release polling,pulse"]
pub type BitChinfoPollW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_REG_DBGDDL` reader - Dummy reg"]
pub type BitRegDbgddlR = crate::FieldReader;
#[doc = "Field `BIT_REG_DBGDDL` writer - Dummy reg"]
pub type BitRegDbgddlW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `BIT_CHINFO_ADDR` reader - channel info write address,8 bytes unit"]
pub type BitChinfoAddrR = crate::FieldReader;
#[doc = "Field `BIT_CHINFO_NUM` reader - channel info packet number in csi buffer"]
pub type BitChinfoNumR = crate::FieldReader;
#[doc = "Field `BIT_CHINFO_STR` reader - channel info read address,8 bytes unit"]
pub type BitChinfoStrR = crate::FieldReader;
impl R {
    #[doc = "Bits 1:7 - Dummy reg"]
    #[inline(always)]
    pub fn bit_reg_dbgddl(&self) -> BitRegDbgddlR {
        BitRegDbgddlR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:15 - channel info write address,8 bytes unit"]
    #[inline(always)]
    pub fn bit_chinfo_addr(&self) -> BitChinfoAddrR {
        BitChinfoAddrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - channel info packet number in csi buffer"]
    #[inline(always)]
    pub fn bit_chinfo_num(&self) -> BitChinfoNumR {
        BitChinfoNumR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - channel info read address,8 bytes unit"]
    #[inline(always)]
    pub fn bit_chinfo_str(&self) -> BitChinfoStrR {
        BitChinfoStrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - channel info release polling,pulse"]
    #[inline(always)]
    pub fn bit_chinfo_poll(&mut self) -> BitChinfoPollW<'_, RegChannelInfoRelSpec> {
        BitChinfoPollW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Dummy reg"]
    #[inline(always)]
    pub fn bit_reg_dbgddl(&mut self) -> BitRegDbgddlW<'_, RegChannelInfoRelSpec> {
        BitRegDbgddlW::new(self, 1)
    }
}
#[doc = "CHANNEL INFO REALSE\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_channel_info_rel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_channel_info_rel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegChannelInfoRelSpec;
impl crate::RegisterSpec for RegChannelInfoRelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_channel_info_rel::R`](R) reader structure"]
impl crate::Readable for RegChannelInfoRelSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_channel_info_rel::W`](W) writer structure"]
impl crate::Writable for RegChannelInfoRelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_CHANNEL_INFO_REL to value 0"]
impl crate::Resettable for RegChannelInfoRelSpec {}
