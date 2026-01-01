#[doc = "Register `TCCR1D` reader"]
pub struct R(crate::R<TCCR1D_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR1D_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR1D_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR1D_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR1D` writer"]
pub struct W(crate::W<TCCR1D_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR1D_SPEC>;
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
impl From<crate::W<TCCR1D_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR1D_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OC1AU` reader - Timer/Counter1 Output Compare U-pin Enable for Channel A"]
pub type OC1AU_R = crate::BitReader<bool>;
#[doc = "Field `OC1AU` writer - Timer/Counter1 Output Compare U-pin Enable for Channel A"]
pub type OC1AU_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1D_SPEC, bool, O>;
#[doc = "Field `OC1AV` reader - Timer/Counter1 Output Compare V-pin Enable for Channel A"]
pub type OC1AV_R = crate::BitReader<bool>;
#[doc = "Field `OC1AV` writer - Timer/Counter1 Output Compare V-pin Enable for Channel A"]
pub type OC1AV_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1D_SPEC, bool, O>;
#[doc = "Field `OC1AW` reader - Timer/Counter1 Output Compare W-pin Enable for Channel A"]
pub type OC1AW_R = crate::BitReader<bool>;
#[doc = "Field `OC1AW` writer - Timer/Counter1 Output Compare W-pin Enable for Channel A"]
pub type OC1AW_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1D_SPEC, bool, O>;
#[doc = "Field `OC1AX` reader - Timer/Counter1 Output Compare X-pin Enable for Channel A"]
pub type OC1AX_R = crate::BitReader<bool>;
#[doc = "Field `OC1AX` writer - Timer/Counter1 Output Compare X-pin Enable for Channel A"]
pub type OC1AX_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1D_SPEC, bool, O>;
#[doc = "Field `OC1BU` reader - Timer/Counter1 Output Compare U-pin Enable for Channel B"]
pub type OC1BU_R = crate::BitReader<bool>;
#[doc = "Field `OC1BU` writer - Timer/Counter1 Output Compare U-pin Enable for Channel B"]
pub type OC1BU_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1D_SPEC, bool, O>;
#[doc = "Field `OC1BV` reader - Timer/Counter1 Output Compare V-pin Enable for Channel B"]
pub type OC1BV_R = crate::BitReader<bool>;
#[doc = "Field `OC1BV` writer - Timer/Counter1 Output Compare V-pin Enable for Channel B"]
pub type OC1BV_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1D_SPEC, bool, O>;
#[doc = "Field `OC1BW` reader - Timer/Counter1 Output Compare W-pin Enable for Channel B"]
pub type OC1BW_R = crate::BitReader<bool>;
#[doc = "Field `OC1BW` writer - Timer/Counter1 Output Compare W-pin Enable for Channel B"]
pub type OC1BW_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1D_SPEC, bool, O>;
#[doc = "Field `OC1BX` reader - Timer/Counter1 Output Compare X-pin Enable for Channel B"]
pub type OC1BX_R = crate::BitReader<bool>;
#[doc = "Field `OC1BX` writer - Timer/Counter1 Output Compare X-pin Enable for Channel B"]
pub type OC1BX_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR1D_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Timer/Counter1 Output Compare U-pin Enable for Channel A"]
    #[inline(always)]
    pub fn oc1au(&self) -> OC1AU_R {
        OC1AU_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output Compare V-pin Enable for Channel A"]
    #[inline(always)]
    pub fn oc1av(&self) -> OC1AV_R {
        OC1AV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output Compare W-pin Enable for Channel A"]
    #[inline(always)]
    pub fn oc1aw(&self) -> OC1AW_R {
        OC1AW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer/Counter1 Output Compare X-pin Enable for Channel A"]
    #[inline(always)]
    pub fn oc1ax(&self) -> OC1AX_R {
        OC1AX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer/Counter1 Output Compare U-pin Enable for Channel B"]
    #[inline(always)]
    pub fn oc1bu(&self) -> OC1BU_R {
        OC1BU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer/Counter1 Output Compare V-pin Enable for Channel B"]
    #[inline(always)]
    pub fn oc1bv(&self) -> OC1BV_R {
        OC1BV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer/Counter1 Output Compare W-pin Enable for Channel B"]
    #[inline(always)]
    pub fn oc1bw(&self) -> OC1BW_R {
        OC1BW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Timer/Counter1 Output Compare X-pin Enable for Channel B"]
    #[inline(always)]
    pub fn oc1bx(&self) -> OC1BX_R {
        OC1BX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Timer/Counter1 Output Compare U-pin Enable for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn oc1au(&mut self) -> OC1AU_W<0> {
        OC1AU_W::new(self)
    }
    #[doc = "Bit 1 - Timer/Counter1 Output Compare V-pin Enable for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn oc1av(&mut self) -> OC1AV_W<1> {
        OC1AV_W::new(self)
    }
    #[doc = "Bit 2 - Timer/Counter1 Output Compare W-pin Enable for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn oc1aw(&mut self) -> OC1AW_W<2> {
        OC1AW_W::new(self)
    }
    #[doc = "Bit 3 - Timer/Counter1 Output Compare X-pin Enable for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn oc1ax(&mut self) -> OC1AX_W<3> {
        OC1AX_W::new(self)
    }
    #[doc = "Bit 4 - Timer/Counter1 Output Compare U-pin Enable for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn oc1bu(&mut self) -> OC1BU_W<4> {
        OC1BU_W::new(self)
    }
    #[doc = "Bit 5 - Timer/Counter1 Output Compare V-pin Enable for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn oc1bv(&mut self) -> OC1BV_W<5> {
        OC1BV_W::new(self)
    }
    #[doc = "Bit 6 - Timer/Counter1 Output Compare W-pin Enable for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn oc1bw(&mut self) -> OC1BW_W<6> {
        OC1BW_W::new(self)
    }
    #[doc = "Bit 7 - Timer/Counter1 Output Compare X-pin Enable for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn oc1bx(&mut self) -> OC1BX_W<7> {
        OC1BX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter1 Control Register D\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr1d](index.html) module"]
pub struct TCCR1D_SPEC;
impl crate::RegisterSpec for TCCR1D_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr1d::R](R) reader structure"]
impl crate::Readable for TCCR1D_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr1d::W](W) writer structure"]
impl crate::Writable for TCCR1D_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR1D to value 0"]
impl crate::Resettable for TCCR1D_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
