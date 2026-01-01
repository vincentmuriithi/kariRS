#[doc = "Register `CTRLB` reader"]
pub struct R(crate::R<CTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLB` writer"]
pub struct W(crate::W<CTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ATTACH` reader - Attach"]
pub type ATTACH_R = crate::BitReader<bool>;
#[doc = "Field `ATTACH` writer - Attach"]
pub type ATTACH_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `GNAK` reader - Respond with NAK on all Endpoints"]
pub type GNAK_R = crate::BitReader<bool>;
#[doc = "Field `GNAK` writer - Respond with NAK on all Endpoints"]
pub type GNAK_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `GNAUTO` reader - Set GNAK Automatically after SETUP"]
pub type GNAUTO_R = crate::BitReader<bool>;
#[doc = "Field `GNAUTO` writer - Set GNAK Automatically after SETUP"]
pub type GNAUTO_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
#[doc = "Field `URESUME` reader - Send Upstream Resume"]
pub type URESUME_R = crate::BitReader<bool>;
#[doc = "Field `URESUME` writer - Send Upstream Resume"]
pub type URESUME_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLB_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Attach"]
    #[inline(always)]
    pub fn attach(&self) -> ATTACH_R {
        ATTACH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Respond with NAK on all Endpoints"]
    #[inline(always)]
    pub fn gnak(&self) -> GNAK_R {
        GNAK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set GNAK Automatically after SETUP"]
    #[inline(always)]
    pub fn gnauto(&self) -> GNAUTO_R {
        GNAUTO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Send Upstream Resume"]
    #[inline(always)]
    pub fn uresume(&self) -> URESUME_R {
        URESUME_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Attach"]
    #[inline(always)]
    #[must_use]
    pub fn attach(&mut self) -> ATTACH_W<0> {
        ATTACH_W::new(self)
    }
    #[doc = "Bit 1 - Respond with NAK on all Endpoints"]
    #[inline(always)]
    #[must_use]
    pub fn gnak(&mut self) -> GNAK_W<1> {
        GNAK_W::new(self)
    }
    #[doc = "Bit 2 - Set GNAK Automatically after SETUP"]
    #[inline(always)]
    #[must_use]
    pub fn gnauto(&mut self) -> GNAUTO_W<2> {
        GNAUTO_W::new(self)
    }
    #[doc = "Bit 3 - Send Upstream Resume"]
    #[inline(always)]
    #[must_use]
    pub fn uresume(&mut self) -> URESUME_W<3> {
        URESUME_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrlb](index.html) module"]
pub struct CTRLB_SPEC;
impl crate::RegisterSpec for CTRLB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrlb::R](R) reader structure"]
impl crate::Readable for CTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrlb::W](W) writer structure"]
impl crate::Writable for CTRLB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLB to value 0"]
impl crate::Resettable for CTRLB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
