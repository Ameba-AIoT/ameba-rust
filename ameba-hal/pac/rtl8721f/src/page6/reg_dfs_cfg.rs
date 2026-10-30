#[doc = "Register `REG_DFS_CFG` reader"]
pub type R = crate::R<RegDfsCfgSpec>;
#[doc = "Register `REG_DFS_CFG` writer"]
pub type W = crate::W<RegDfsCfgSpec>;
#[doc = "Field `r_dfs_stop` reader - When set to 1, stop to get DFS from BB and clear the DFS buffer"]
pub type RDfsStopR = crate::BitReader;
#[doc = "Field `r_dfs_stop` writer - When set to 1, stop to get DFS from BB and clear the DFS buffer"]
pub type RDfsStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dfs_read_mode` reader - When set to 1,means host read mode for debug"]
pub type RDfsReadModeR = crate::BitReader;
#[doc = "Field `r_dfs_read_mode` writer - When set to 1,means host read mode for debug"]
pub type RDfsReadModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dfs_rst_drop_cnt` writer - When set to 1, clear r_dfs_drop_cnt, one pulse"]
pub type RDfsRstDropCntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `r_dfs_pkt_len` reader - dfs packet length need Host to read, 4 bytes unit"]
pub type RDfsPktLenR = crate::FieldReader<u16>;
#[doc = "Field `r_dfs_en_intrpt` writer - When set to 1, enable generate DFS interrupt, one pulse"]
pub type RDfsEnIntrptW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dfs_en_intrpt` reader - When valid, enable generate dfs interrupt"]
pub type DfsEnIntrptR = crate::BitReader;
#[doc = "Field `dfs_buff_full` reader - dfs buffer is full"]
pub type DfsBuffFullR = crate::BitReader;
#[doc = "Field `r_chinfo_content_bmask` reader - byte mask of 4bytes content value that need to compare when set to 1,the corresponding byte need to compare"]
pub type RChinfoContentBmaskR = crate::FieldReader;
#[doc = "Field `r_chinfo_content_bmask` writer - byte mask of 4bytes content value that need to compare when set to 1,the corresponding byte need to compare"]
pub type RChinfoContentBmaskW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `r_dfs_drop_cnt` reader - drop dfs packets number when dfsbuffer is full"]
pub type RDfsDropCntR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - When set to 1, stop to get DFS from BB and clear the DFS buffer"]
    #[inline(always)]
    pub fn r_dfs_stop(&self) -> RDfsStopR {
        RDfsStopR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set to 1,means host read mode for debug"]
    #[inline(always)]
    pub fn r_dfs_read_mode(&self) -> RDfsReadModeR {
        RDfsReadModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 6:15 - dfs packet length need Host to read, 4 bytes unit"]
    #[inline(always)]
    pub fn r_dfs_pkt_len(&self) -> RDfsPktLenR {
        RDfsPktLenR::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bit 17 - When valid, enable generate dfs interrupt"]
    #[inline(always)]
    pub fn dfs_en_intrpt(&self) -> DfsEnIntrptR {
        DfsEnIntrptR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - dfs buffer is full"]
    #[inline(always)]
    pub fn dfs_buff_full(&self) -> DfsBuffFullR {
        DfsBuffFullR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - byte mask of 4bytes content value that need to compare when set to 1,the corresponding byte need to compare"]
    #[inline(always)]
    pub fn r_chinfo_content_bmask(&self) -> RChinfoContentBmaskR {
        RChinfoContentBmaskR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bits 23:31 - drop dfs packets number when dfsbuffer is full"]
    #[inline(always)]
    pub fn r_dfs_drop_cnt(&self) -> RDfsDropCntR {
        RDfsDropCntR::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - When set to 1, stop to get DFS from BB and clear the DFS buffer"]
    #[inline(always)]
    pub fn r_dfs_stop(&mut self) -> RDfsStopW<'_, RegDfsCfgSpec> {
        RDfsStopW::new(self, 0)
    }
    #[doc = "Bit 1 - When set to 1,means host read mode for debug"]
    #[inline(always)]
    pub fn r_dfs_read_mode(&mut self) -> RDfsReadModeW<'_, RegDfsCfgSpec> {
        RDfsReadModeW::new(self, 1)
    }
    #[doc = "Bit 5 - When set to 1, clear r_dfs_drop_cnt, one pulse"]
    #[inline(always)]
    pub fn r_dfs_rst_drop_cnt(&mut self) -> RDfsRstDropCntW<'_, RegDfsCfgSpec> {
        RDfsRstDropCntW::new(self, 5)
    }
    #[doc = "Bit 16 - When set to 1, enable generate DFS interrupt, one pulse"]
    #[inline(always)]
    pub fn r_dfs_en_intrpt(&mut self) -> RDfsEnIntrptW<'_, RegDfsCfgSpec> {
        RDfsEnIntrptW::new(self, 16)
    }
    #[doc = "Bits 19:22 - byte mask of 4bytes content value that need to compare when set to 1,the corresponding byte need to compare"]
    #[inline(always)]
    pub fn r_chinfo_content_bmask(&mut self) -> RChinfoContentBmaskW<'_, RegDfsCfgSpec> {
        RChinfoContentBmaskW::new(self, 19)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`reg_dfs_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reg_dfs_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegDfsCfgSpec;
impl crate::RegisterSpec for RegDfsCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_dfs_cfg::R`](R) reader structure"]
impl crate::Readable for RegDfsCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`reg_dfs_cfg::W`](W) writer structure"]
impl crate::Writable for RegDfsCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REG_DFS_CFG to value 0x007a_0000"]
impl crate::Resettable for RegDfsCfgSpec {
    const RESET_VALUE: u32 = 0x007a_0000;
}
