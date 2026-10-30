#[doc = "Register `REG_FLC_CTRL` reader"]
pub type R = crate::R<RegFlcCtrlSpec>;
#[doc = "Register `REG_FLC_CTRL` writer"]
pub type W = crate::W<RegFlcCtrlSpec>;
#[doc = "Field `BIT_FLC_RPC` reader - Number of Rx PKT in the interval which is specified by FLC_TRPC. The Rx PKT type is specified by 0x6AE\\[2:0\\]"]
pub type BitFlcRpcR = crate::FieldReader;
#[doc = "Field `BIT_FLC_PKTTH` reader - Rx PKT number threshold"]
pub type BitFlcPktthR = crate::FieldReader;
#[doc = "Field `BIT_FLC_PKTTH` writer - Rx PKT number threshold"]
pub type BitFlcPktthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BIT_FLC_CDFEN` reader - Count Data Frame (1: Enable, 0:Disable)"]
pub type BitFlcCdfenR = crate::BitReader;
#[doc = "Field `BIT_FLC_CDFEN` writer - Count Data Frame (1: Enable, 0:Disable)"]
pub type BitFlcCdfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FLC_CCFEN` reader - Count Control Frame (1: Enable, 0:Disable)"]
pub type BitFlcCcfenR = crate::BitReader;
#[doc = "Field `BIT_FLC_CCFEN` writer - Count Control Frame (1: Enable, 0:Disable)"]
pub type BitFlcCcfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FLC_CMFEN` reader - Count Management Frame (1: Enable, 0:Disable)"]
pub type BitFlcCmfenR = crate::BitReader;
#[doc = "Field `BIT_FLC_CMFEN` writer - Count Management Frame (1: Enable, 0:Disable)"]
pub type BitFlcCmfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_TORRD` reader - It specifies the time-out duration. (unit: 32 ms)"]
pub type BitTorrdR = crate::FieldReader;
#[doc = "Field `BIT_TORRD` writer - It specifies the time-out duration. (unit: 32 ms)"]
pub type BitTorrdW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `BIT_FLC_MODSL` reader - 1:Timer Mode 0:Counter Mode"]
pub type BitFlcModslR = crate::BitReader;
#[doc = "Field `BIT_FLC_MODSL` writer - 1:Timer Mode 0:Counter Mode"]
pub type BitFlcModslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_FLC_RPCT_V1` reader - When write \"1\" to this bit, FLC_TRPC will be enabled."]
pub type BitFlcRpctV1R = crate::BitReader;
#[doc = "Field `BIT_FLC_RPCT_V1` writer - When write \"1\" to this bit, FLC_TRPC will be enabled."]
pub type BitFlcRpctV1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Number of Rx PKT in the interval which is specified by FLC_TRPC. The Rx PKT type is specified by 0x6AE\\[2:0\\]"]
    #[inline(always)]
    pub fn bit_flc_rpc(&self) -> BitFlcRpcR {
        BitFlcRpcR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Rx PKT number threshold"]
    #[inline(always)]
    pub fn bit_flc_pktth(&self) -> BitFlcPktthR {
        BitFlcPktthR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Count Data Frame (1: Enable, 0:Disable)"]
    #[inline(always)]
    pub fn bit_flc_cdfen(&self) -> BitFlcCdfenR {
        BitFlcCdfenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Count Control Frame (1: Enable, 0:Disable)"]
    #[inline(always)]
    pub fn bit_flc_ccfen(&self) -> BitFlcCcfenR {
        BitFlcCcfenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Count Management Frame (1: Enable, 0:Disable)"]
    #[inline(always)]
    pub fn bit_flc_cmfen(&self) -> BitFlcCmfenR {
        BitFlcCmfenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 24:29 - It specifies the time-out duration. (unit: 32 ms)"]
    #[inline(always)]
    pub fn bit_torrd(&self) -> BitTorrdR {
        BitTorrdR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - 1:Timer Mode 0:Counter Mode"]
    #[inline(always)]
    pub fn bit_flc_modsl(&self) -> BitFlcModslR {
        BitFlcModslR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - When write \"1\" to this bit, FLC_TRPC will be enabled."]
    #[inline(always)]
    pub fn bit_flc_rpct_v1(&self) -> BitFlcRpctV1R {
        BitFlcRpctV1R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:15 - Rx PKT number threshold"]
    #[inline(always)]
    pub fn bit_flc_pktth(&mut self) -> BitFlcPktthW<'_, RegFlcCtrlSpec> {
        BitFlcPktthW::new(self, 8)
    }
    #[doc = "Bit 16 - Count Data Frame (1: Enable, 0:Disable)"]
    #[inline(always)]
    pub fn bit_flc_cdfen(&mut self) -> BitFlcCdfenW<'_, RegFlcCtrlSpec> {
        BitFlcCdfenW::new(self, 16)
    }
    #[doc = "Bit 17 - Count Control Frame (1: Enable, 0:Disable)"]
    #[inline(always)]
    pub fn bit_flc_ccfen(&mut self) -> BitFlcCcfenW<'_, RegFlcCtrlSpec> {
        BitFlcCcfenW::new(self, 17)
    }
    #[doc = "Bit 18 - Count Management Frame (1: Enable, 0:Disable)"]
    #[inline(always)]
    pub fn bit_flc_cmfen(&mut self) -> BitFlcCmfenW<'_, RegFlcCtrlSpec> {
        BitFlcCmfenW::new(self, 18)
    }
    #[doc = "Bits 24:29 - It specifies the time-out duration. (unit: 32 ms)"]
    #[inline(always)]
    pub fn bit_torrd(&mut self) -> BitTorrdW<'_, RegFlcCtrlSpec> {
        BitTorrdW::new(self, 24)
    }
    #[doc = "Bit 30 - 1:Timer Mode 0:Counter Mode"]
    #[inline(always)]
    pub fn bit_flc_modsl(&mut self) -> BitFlcModslW<'_, RegFlcCtrlSpec> {
        BitFlcModslW::new(self, 30)
    }
    #[doc = "Bit 31 - When write \"1\" to this bit, FLC_TRPC will be enabled."]
    #[inline(always)]
    pub fn bit_flc_rpct_v1(&mut self) -> BitFlcRpctV1W<'_, RegFlcCtrlSpec> {
        BitFlcRpctV1W::new(self, 31)
    }
}
#[doc = "FW LPS Condition PKT Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_flc_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_flc_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegFlcCtrlSpec;
impl crate::RegisterSpec for RegFlcCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_flc_ctrl::R`](R) reader structure"]
impl crate::Readable for RegFlcCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_flc_ctrl::W`](W) writer structure"]
impl crate::Writable for RegFlcCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_FLC_CTRL to value 0"]
impl crate::Resettable for RegFlcCtrlSpec {}
