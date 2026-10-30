#[doc = "Register `tag_2` reader"]
pub type R = crate::R<Tag2Spec>;
#[doc = "Register `tag_2` writer"]
pub type W = crate::W<Tag2Spec>;
#[doc = "Field `tag_2` reader - Tag register bit\\[32*2+31:32*2\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
pub type Tag2R = crate::FieldReader<u32>;
#[doc = "Field `tag_2` writer - Tag register bit\\[32*2+31:32*2\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
pub type Tag2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Tag register bit\\[32*2+31:32*2\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
    #[inline(always)]
    pub fn tag_2(&self) -> Tag2R {
        Tag2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Tag register bit\\[32*2+31:32*2\\] After a complete GCM operation, a tag will be generated. The tag will stored in this register. In addition, because GCM can be done in segments, tag can be used as input when GCM is started again. The tag order depends on chnx_aes_reg.config.byte_swap bit. If byte_swap = 0, tag = {tag_3\\[7:0\\], tag_3\\[15:8\\], tag_3\\[23:16\\], tag_3\\[31:24\\], tag_2\\[7:0\\], tag_2\\[15:8\\], tag_2\\[23:16\\], tag_2\\[31:24\\], tag_1\\[7:0\\], tag_1\\[15:8\\], tag_1\\[23:16\\], tag_1\\[31:24\\], tag_0\\[7:0\\], tag_0\\[15:8\\], tag_0\\[23:16\\], tag_0\\[31:24\\]} If byte_swap = 1, tag = {tag_3, tag_2, tag_1, tag_0}"]
    #[inline(always)]
    pub fn tag_2(&mut self) -> Tag2W<'_, Tag2Spec> {
        Tag2W::new(self, 0)
    }
}
#[doc = "Tag register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`tag_2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tag_2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tag2Spec;
impl crate::RegisterSpec for Tag2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tag_2::R`](R) reader structure"]
impl crate::Readable for Tag2Spec {}
#[doc = "`write(|w| ..)` method takes [`tag_2::W`](W) writer structure"]
impl crate::Writable for Tag2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets tag_2 to value 0"]
impl crate::Resettable for Tag2Spec {}
