#[doc = "Register `LINERR` reader"]
pub struct R(crate::R<LINERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LINERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LINERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LINERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LINERR` writer"]
pub struct W(crate::W<LINERR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LINERR_SPEC>;
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
impl From<crate::W<LINERR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LINERR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LBERR` reader - Bit Error Flag"]
pub type LBERR_R = crate::BitReader<bool>;
#[doc = "Field `LBERR` writer - Bit Error Flag"]
pub type LBERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINERR_SPEC, bool, O>;
#[doc = "Field `LCERR` reader - Checksum Error Flag"]
pub type LCERR_R = crate::BitReader<bool>;
#[doc = "Field `LCERR` writer - Checksum Error Flag"]
pub type LCERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINERR_SPEC, bool, O>;
#[doc = "Field `LPERR` reader - Parity Error Flag"]
pub type LPERR_R = crate::BitReader<bool>;
#[doc = "Field `LPERR` writer - Parity Error Flag"]
pub type LPERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINERR_SPEC, bool, O>;
#[doc = "Field `LSERR` reader - Synchronization Error Flag"]
pub type LSERR_R = crate::BitReader<bool>;
#[doc = "Field `LSERR` writer - Synchronization Error Flag"]
pub type LSERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINERR_SPEC, bool, O>;
#[doc = "Field `LFERR` reader - Framing Error Flag"]
pub type LFERR_R = crate::BitReader<bool>;
#[doc = "Field `LFERR` writer - Framing Error Flag"]
pub type LFERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINERR_SPEC, bool, O>;
#[doc = "Field `LOVERR` reader - Overrun Error Flag"]
pub type LOVERR_R = crate::BitReader<bool>;
#[doc = "Field `LOVERR` writer - Overrun Error Flag"]
pub type LOVERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINERR_SPEC, bool, O>;
#[doc = "Field `LTOERR` reader - Frame Time Out Error Flag"]
pub type LTOERR_R = crate::BitReader<bool>;
#[doc = "Field `LTOERR` writer - Frame Time Out Error Flag"]
pub type LTOERR_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINERR_SPEC, bool, O>;
#[doc = "Field `LABORT` reader - Abort Flag"]
pub type LABORT_R = crate::BitReader<bool>;
#[doc = "Field `LABORT` writer - Abort Flag"]
pub type LABORT_W<'a, const O: u8> = crate::BitWriter<'a, u8, LINERR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Bit Error Flag"]
    #[inline(always)]
    pub fn lberr(&self) -> LBERR_R {
        LBERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Checksum Error Flag"]
    #[inline(always)]
    pub fn lcerr(&self) -> LCERR_R {
        LCERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error Flag"]
    #[inline(always)]
    pub fn lperr(&self) -> LPERR_R {
        LPERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Synchronization Error Flag"]
    #[inline(always)]
    pub fn lserr(&self) -> LSERR_R {
        LSERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Framing Error Flag"]
    #[inline(always)]
    pub fn lferr(&self) -> LFERR_R {
        LFERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    pub fn loverr(&self) -> LOVERR_R {
        LOVERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Frame Time Out Error Flag"]
    #[inline(always)]
    pub fn ltoerr(&self) -> LTOERR_R {
        LTOERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Abort Flag"]
    #[inline(always)]
    pub fn labort(&self) -> LABORT_R {
        LABORT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lberr(&mut self) -> LBERR_W<0> {
        LBERR_W::new(self)
    }
    #[doc = "Bit 1 - Checksum Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lcerr(&mut self) -> LCERR_W<1> {
        LCERR_W::new(self)
    }
    #[doc = "Bit 2 - Parity Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lperr(&mut self) -> LPERR_W<2> {
        LPERR_W::new(self)
    }
    #[doc = "Bit 3 - Synchronization Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lserr(&mut self) -> LSERR_W<3> {
        LSERR_W::new(self)
    }
    #[doc = "Bit 4 - Framing Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lferr(&mut self) -> LFERR_W<4> {
        LFERR_W::new(self)
    }
    #[doc = "Bit 5 - Overrun Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn loverr(&mut self) -> LOVERR_W<5> {
        LOVERR_W::new(self)
    }
    #[doc = "Bit 6 - Frame Time Out Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ltoerr(&mut self) -> LTOERR_W<6> {
        LTOERR_W::new(self)
    }
    #[doc = "Bit 7 - Abort Flag"]
    #[inline(always)]
    #[must_use]
    pub fn labort(&mut self) -> LABORT_W<7> {
        LABORT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LIN Error Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [linerr](index.html) module"]
pub struct LINERR_SPEC;
impl crate::RegisterSpec for LINERR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [linerr::R](R) reader structure"]
impl crate::Readable for LINERR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [linerr::W](W) writer structure"]
impl crate::Writable for LINERR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LINERR to value 0"]
impl crate::Resettable for LINERR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
