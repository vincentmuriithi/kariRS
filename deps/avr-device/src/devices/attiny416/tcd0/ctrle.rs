#[doc = "Register `CTRLE` reader"]
pub struct R(crate::R<CTRLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRLE` writer"]
pub struct W(crate::W<CTRLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRLE_SPEC>;
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
impl From<crate::W<CTRLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCEOC` reader - synchronize end of cycle strobe"]
pub type SYNCEOC_R = crate::BitReader<bool>;
#[doc = "Field `SYNCEOC` writer - synchronize end of cycle strobe"]
pub type SYNCEOC_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLE_SPEC, bool, O>;
#[doc = "Field `SYNC` reader - synchronize strobe"]
pub type SYNC_R = crate::BitReader<bool>;
#[doc = "Field `SYNC` writer - synchronize strobe"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLE_SPEC, bool, O>;
#[doc = "Field `RESTART` reader - Restart strobe"]
pub type RESTART_R = crate::BitReader<bool>;
#[doc = "Field `RESTART` writer - Restart strobe"]
pub type RESTART_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLE_SPEC, bool, O>;
#[doc = "Field `SCAPTUREA` reader - Software Capture A Strobe"]
pub type SCAPTUREA_R = crate::BitReader<bool>;
#[doc = "Field `SCAPTUREA` writer - Software Capture A Strobe"]
pub type SCAPTUREA_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLE_SPEC, bool, O>;
#[doc = "Field `SCAPTUREB` reader - Software Capture B Strobe"]
pub type SCAPTUREB_R = crate::BitReader<bool>;
#[doc = "Field `SCAPTUREB` writer - Software Capture B Strobe"]
pub type SCAPTUREB_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLE_SPEC, bool, O>;
#[doc = "Field `DISEOC` reader - Disable at end of cycle"]
pub type DISEOC_R = crate::BitReader<bool>;
#[doc = "Field `DISEOC` writer - Disable at end of cycle"]
pub type DISEOC_W<'a, const O: u8> = crate::BitWriter<'a, u8, CTRLE_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - synchronize end of cycle strobe"]
    #[inline(always)]
    pub fn synceoc(&self) -> SYNCEOC_R {
        SYNCEOC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - synchronize strobe"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restart strobe"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Software Capture A Strobe"]
    #[inline(always)]
    pub fn scapturea(&self) -> SCAPTUREA_R {
        SCAPTUREA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Software Capture B Strobe"]
    #[inline(always)]
    pub fn scaptureb(&self) -> SCAPTUREB_R {
        SCAPTUREB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable at end of cycle"]
    #[inline(always)]
    pub fn diseoc(&self) -> DISEOC_R {
        DISEOC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - synchronize end of cycle strobe"]
    #[inline(always)]
    #[must_use]
    pub fn synceoc(&mut self) -> SYNCEOC_W<0> {
        SYNCEOC_W::new(self)
    }
    #[doc = "Bit 1 - synchronize strobe"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<1> {
        SYNC_W::new(self)
    }
    #[doc = "Bit 2 - Restart strobe"]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<2> {
        RESTART_W::new(self)
    }
    #[doc = "Bit 3 - Software Capture A Strobe"]
    #[inline(always)]
    #[must_use]
    pub fn scapturea(&mut self) -> SCAPTUREA_W<3> {
        SCAPTUREA_W::new(self)
    }
    #[doc = "Bit 4 - Software Capture B Strobe"]
    #[inline(always)]
    #[must_use]
    pub fn scaptureb(&mut self) -> SCAPTUREB_W<4> {
        SCAPTUREB_W::new(self)
    }
    #[doc = "Bit 7 - Disable at end of cycle"]
    #[inline(always)]
    #[must_use]
    pub fn diseoc(&mut self) -> DISEOC_W<7> {
        DISEOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control E\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrle](index.html) module"]
pub struct CTRLE_SPEC;
impl crate::RegisterSpec for CTRLE_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ctrle::R](R) reader structure"]
impl crate::Readable for CTRLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrle::W](W) writer structure"]
impl crate::Writable for CTRLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLE to value 0"]
impl crate::Resettable for CTRLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
