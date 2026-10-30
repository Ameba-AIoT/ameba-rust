#[doc = "Register `DSR7_L` reader"]
pub type R = crate::R<Dsr7LSpec>;
#[doc = "Register `DSR7_L` writer"]
pub type W = crate::W<Dsr7LSpec>;
#[doc = "Field `DSR7_L_DSI` reader - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr7LDsiR = crate::FieldReader<u32>;
#[doc = "Field `DSR7_L_DSI` writer - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr7LDsiW<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `DSR7_L_DSC` reader - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr7LDscR = crate::FieldReader<u16>;
#[doc = "Field `DSR7_L_DSC` writer - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
pub type Dsr7LDscW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr7_l_dsi(&self) -> Dsr7LDsiR {
        Dsr7LDsiR::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr7_l_dsc(&self) -> Dsr7LDscR {
        Dsr7LDscR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - Destination scatter interval.Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr7_l_dsi(&mut self) -> Dsr7LDsiW<'_, Dsr7LSpec> {
        Dsr7LDsiW::new(self, 0)
    }
    #[doc = "Bits 20:31 - Destination scatter count. Destination contiguous transfer count between successive scatter boundaries. Note: if user not define DMAC_DST_SCA_EN, it only can be readed."]
    #[inline(always)]
    pub fn dsr7_l_dsc(&mut self) -> Dsr7LDscW<'_, Dsr7LSpec> {
        Dsr7LDscW::new(self, 20)
    }
}
#[doc = "Channel 7 low-32 bits Destination Scatter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsr7_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr7_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dsr7LSpec;
impl crate::RegisterSpec for Dsr7LSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsr7_l::R`](R) reader structure"]
impl crate::Readable for Dsr7LSpec {}
#[doc = "`write(|w| ..)` method takes [`dsr7_l::W`](W) writer structure"]
impl crate::Writable for Dsr7LSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSR7_L to value 0"]
impl crate::Resettable for Dsr7LSpec {}
