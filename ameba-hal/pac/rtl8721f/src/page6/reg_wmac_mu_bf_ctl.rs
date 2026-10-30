#[doc = "Register `REG_WMAC_MU_BF_CTL` reader"]
pub type R = crate::R<RegWmacMuBfCtlSpec>;
#[doc = "Register `REG_WMAC_MU_BF_CTL` writer"]
pub type W = crate::W<RegWmacMuBfCtlSpec>;
#[doc = "Field `BIT_WMAC_MU_BF_MYAID` reader - When I'm one of Bfees, my AID12 to identify STA_info in NDPA"]
pub type BitWmacMuBfMyaidR = crate::FieldReader<u16>;
#[doc = "Field `BIT_WMAC_MU_BF_MYAID` writer - When I'm one of Bfees, my AID12 to identify STA_info in NDPA"]
pub type BitWmacMuBfMyaidW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `BIT_WMAC_MU_BFRPTSEG_SEL` reader - Beamforming report segment length select 2'h00 :3895 bytes(also means csi segmentation enable),2'h01:7991bytes, 2'h10:11454bytes"]
pub type BitWmacMuBfrptsegSelR = crate::FieldReader;
#[doc = "Field `BIT_WMAC_MU_BFRPTSEG_SEL` writer - Beamforming report segment length select 2'h00 :3895 bytes(also means csi segmentation enable),2'h01:7991bytes, 2'h10:11454bytes"]
pub type BitWmacMuBfrptsegSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BIT_WMAC_RETXBFRPTSEQ_UPD` reader - Update sequence number and not set retry bit when retransmit beamforming report."]
pub type BitWmacRetxbfrptseqUpdR = crate::BitReader;
#[doc = "Field `BIT_WMAC_RETXBFRPTSEQ_UPD` writer - Update sequence number and not set retry bit when retransmit beamforming report."]
pub type BitWmacRetxbfrptseqUpdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_WMAC_INVLD_BFPRT_CHK` reader - Check invalid case of received beamforming report"]
pub type BitWmacInvldBfprtChkR = crate::BitReader;
#[doc = "Field `BIT_WMAC_INVLD_BFPRT_CHK` writer - Check invalid case of received beamforming report"]
pub type BitWmacInvldBfprtChkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_BFRPT_PARA_V1` reader - CSI report parameter of selected user ID BFee {bw\\[1:0\\],cb\\[1:0\\],ng\\[1:0\\],nr\\[2:0\\],nc\\[2:0\\]} ;"]
pub type BitBfrptParaV1R = crate::FieldReader<u16>;
#[doc = "Field `BIT_BFRPT_PARA_USERID_SEL_V1` reader - User ID of associated Bfee, MU: 2~7 ,SU:0~1"]
pub type BitBfrptParaUseridSelV1R = crate::FieldReader;
#[doc = "Field `BIT_BFRPT_PARA_USERID_SEL_V1` writer - User ID of associated Bfee, MU: 2~7 ,SU:0~1"]
pub type BitBfrptParaUseridSelV1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:11 - When I'm one of Bfees, my AID12 to identify STA_info in NDPA"]
    #[inline(always)]
    pub fn bit_wmac_mu_bf_myaid(&self) -> BitWmacMuBfMyaidR {
        BitWmacMuBfMyaidR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:13 - Beamforming report segment length select 2'h00 :3895 bytes(also means csi segmentation enable),2'h01:7991bytes, 2'h10:11454bytes"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfrptseg_sel(&self) -> BitWmacMuBfrptsegSelR {
        BitWmacMuBfrptsegSelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Update sequence number and not set retry bit when retransmit beamforming report."]
    #[inline(always)]
    pub fn bit_wmac_retxbfrptseq_upd(&self) -> BitWmacRetxbfrptseqUpdR {
        BitWmacRetxbfrptseqUpdR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Check invalid case of received beamforming report"]
    #[inline(always)]
    pub fn bit_wmac_invld_bfprt_chk(&self) -> BitWmacInvldBfprtChkR {
        BitWmacInvldBfprtChkR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:28 - CSI report parameter of selected user ID BFee {bw\\[1:0\\],cb\\[1:0\\],ng\\[1:0\\],nr\\[2:0\\],nc\\[2:0\\]} ;"]
    #[inline(always)]
    pub fn bit_bfrpt_para_v1(&self) -> BitBfrptParaV1R {
        BitBfrptParaV1R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
    #[doc = "Bits 29:31 - User ID of associated Bfee, MU: 2~7 ,SU:0~1"]
    #[inline(always)]
    pub fn bit_bfrpt_para_userid_sel_v1(&self) -> BitBfrptParaUseridSelV1R {
        BitBfrptParaUseridSelV1R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - When I'm one of Bfees, my AID12 to identify STA_info in NDPA"]
    #[inline(always)]
    pub fn bit_wmac_mu_bf_myaid(&mut self) -> BitWmacMuBfMyaidW<'_, RegWmacMuBfCtlSpec> {
        BitWmacMuBfMyaidW::new(self, 0)
    }
    #[doc = "Bits 12:13 - Beamforming report segment length select 2'h00 :3895 bytes(also means csi segmentation enable),2'h01:7991bytes, 2'h10:11454bytes"]
    #[inline(always)]
    pub fn bit_wmac_mu_bfrptseg_sel(&mut self) -> BitWmacMuBfrptsegSelW<'_, RegWmacMuBfCtlSpec> {
        BitWmacMuBfrptsegSelW::new(self, 12)
    }
    #[doc = "Bit 14 - Update sequence number and not set retry bit when retransmit beamforming report."]
    #[inline(always)]
    pub fn bit_wmac_retxbfrptseq_upd(&mut self) -> BitWmacRetxbfrptseqUpdW<'_, RegWmacMuBfCtlSpec> {
        BitWmacRetxbfrptseqUpdW::new(self, 14)
    }
    #[doc = "Bit 15 - Check invalid case of received beamforming report"]
    #[inline(always)]
    pub fn bit_wmac_invld_bfprt_chk(&mut self) -> BitWmacInvldBfprtChkW<'_, RegWmacMuBfCtlSpec> {
        BitWmacInvldBfprtChkW::new(self, 15)
    }
    #[doc = "Bits 29:31 - User ID of associated Bfee, MU: 2~7 ,SU:0~1"]
    #[inline(always)]
    pub fn bit_bfrpt_para_userid_sel_v1(
        &mut self,
    ) -> BitBfrptParaUseridSelV1W<'_, RegWmacMuBfCtlSpec> {
        BitBfrptParaUseridSelV1W::new(self, 29)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_wmac_mu_bf_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_wmac_mu_bf_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegWmacMuBfCtlSpec;
impl crate::RegisterSpec for RegWmacMuBfCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_wmac_mu_bf_ctl::R`](R) reader structure"]
impl crate::Readable for RegWmacMuBfCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_wmac_mu_bf_ctl::W`](W) writer structure"]
impl crate::Writable for RegWmacMuBfCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_WMAC_MU_BF_CTL to value 0"]
impl crate::Resettable for RegWmacMuBfCtlSpec {}
