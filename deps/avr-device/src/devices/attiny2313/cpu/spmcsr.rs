#[doc = "Register `SPMCSR` reader"]
pub struct R(crate::R<SPMCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPMCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPMCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPMCSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPMCSR` writer"]
pub struct W(crate::W<SPMCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPMCSR_SPEC>;
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
impl From<crate::W<SPMCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPMCSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPMEN` reader - Store Program Memory Enable"]
pub type SPMEN_R = crate::BitReader<bool>;
#[doc = "Field `SPMEN` writer - Store Program Memory Enable"]
pub type SPMEN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPMCSR_SPEC, bool, O>;
#[doc = "Field `PGERS` reader - Page Erase"]
pub type PGERS_R = crate::BitReader<bool>;
#[doc = "Field `PGERS` writer - Page Erase"]
pub type PGERS_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPMCSR_SPEC, bool, O>;
#[doc = "Field `PGWRT` reader - Page Write"]
pub type PGWRT_R = crate::BitReader<bool>;
#[doc = "Field `PGWRT` writer - Page Write"]
pub type PGWRT_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPMCSR_SPEC, bool, O>;
#[doc = "Field `RFLB` reader - Read Fuse and Lock Bits"]
pub type RFLB_R = crate::BitReader<bool>;
#[doc = "Field `RFLB` writer - Read Fuse and Lock Bits"]
pub type RFLB_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPMCSR_SPEC, bool, O>;
#[doc = "Field `CTPB` reader - Clear Temporary Page Buffer"]
pub type CTPB_R = crate::BitReader<bool>;
#[doc = "Field `CTPB` writer - Clear Temporary Page Buffer"]
pub type CTPB_W<'a, const O: u8> = crate::BitWriter<'a, u8, SPMCSR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Store Program Memory Enable"]
    #[inline(always)]
    pub fn spmen(&self) -> SPMEN_R {
        SPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    pub fn pgers(&self) -> PGERS_R {
        PGERS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Page Write"]
    #[inline(always)]
    pub fn pgwrt(&self) -> PGWRT_R {
        PGWRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read Fuse and Lock Bits"]
    #[inline(always)]
    pub fn rflb(&self) -> RFLB_R {
        RFLB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear Temporary Page Buffer"]
    #[inline(always)]
    pub fn ctpb(&self) -> CTPB_R {
        CTPB_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Store Program Memory Enable"]
    #[inline(always)]
    #[must_use]
    pub fn spmen(&mut self) -> SPMEN_W<0> {
        SPMEN_W::new(self)
    }
    #[doc = "Bit 1 - Page Erase"]
    #[inline(always)]
    #[must_use]
    pub fn pgers(&mut self) -> PGERS_W<1> {
        PGERS_W::new(self)
    }
    #[doc = "Bit 2 - Page Write"]
    #[inline(always)]
    #[must_use]
    pub fn pgwrt(&mut self) -> PGWRT_W<2> {
        PGWRT_W::new(self)
    }
    #[doc = "Bit 3 - Read Fuse and Lock Bits"]
    #[inline(always)]
    #[must_use]
    pub fn rflb(&mut self) -> RFLB_W<3> {
        RFLB_W::new(self)
    }
    #[doc = "Bit 4 - Clear Temporary Page Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn ctpb(&mut self) -> CTPB_W<4> {
        CTPB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Store Program Memory Control and Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spmcsr](index.html) module"]
pub struct SPMCSR_SPEC;
impl crate::RegisterSpec for SPMCSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [spmcsr::R](R) reader structure"]
impl crate::Readable for SPMCSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spmcsr::W](W) writer structure"]
impl crate::Writable for SPMCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPMCSR to value 0"]
impl crate::Resettable for SPMCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
