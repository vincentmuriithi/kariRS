#[doc = "Register `TCCR3C` reader"]
pub struct R(crate::R<TCCR3C_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCCR3C_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCCR3C_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCCR3C_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCCR3C` writer"]
pub struct W(crate::W<TCCR3C_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCCR3C_SPEC>;
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
impl From<crate::W<TCCR3C_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCCR3C_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Res` reader - Reserved"]
pub type RES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `Res` writer - Reserved"]
pub type RES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, TCCR3C_SPEC, u8, u8, 5, O>;
#[doc = "Field `FOC3C` reader - Force Output Compare for Channel C"]
pub type FOC3C_R = crate::BitReader<bool>;
#[doc = "Field `FOC3C` writer - Force Output Compare for Channel C"]
pub type FOC3C_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR3C_SPEC, bool, O>;
#[doc = "Field `FOC3B` reader - Force Output Compare for Channel B"]
pub type FOC3B_R = crate::BitReader<bool>;
#[doc = "Field `FOC3B` writer - Force Output Compare for Channel B"]
pub type FOC3B_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR3C_SPEC, bool, O>;
#[doc = "Field `FOC3A` reader - Force Output Compare for Channel A"]
pub type FOC3A_R = crate::BitReader<bool>;
#[doc = "Field `FOC3A` writer - Force Output Compare for Channel A"]
pub type FOC3A_W<'a, const O: u8> = crate::BitWriter<'a, u8, TCCR3C_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Reserved"]
    #[inline(always)]
    pub fn res(&self) -> RES_R {
        RES_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 5 - Force Output Compare for Channel C"]
    #[inline(always)]
    pub fn foc3c(&self) -> FOC3C_R {
        FOC3C_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Force Output Compare for Channel B"]
    #[inline(always)]
    pub fn foc3b(&self) -> FOC3B_R {
        FOC3B_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Force Output Compare for Channel A"]
    #[inline(always)]
    pub fn foc3a(&self) -> FOC3A_R {
        FOC3A_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn res(&mut self) -> RES_W<0> {
        RES_W::new(self)
    }
    #[doc = "Bit 5 - Force Output Compare for Channel C"]
    #[inline(always)]
    #[must_use]
    pub fn foc3c(&mut self) -> FOC3C_W<5> {
        FOC3C_W::new(self)
    }
    #[doc = "Bit 6 - Force Output Compare for Channel B"]
    #[inline(always)]
    #[must_use]
    pub fn foc3b(&mut self) -> FOC3B_W<6> {
        FOC3B_W::new(self)
    }
    #[doc = "Bit 7 - Force Output Compare for Channel A"]
    #[inline(always)]
    #[must_use]
    pub fn foc3a(&mut self) -> FOC3A_W<7> {
        FOC3A_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer/Counter3 Control Register C\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tccr3c](index.html) module"]
pub struct TCCR3C_SPEC;
impl crate::RegisterSpec for TCCR3C_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [tccr3c::R](R) reader structure"]
impl crate::Readable for TCCR3C_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tccr3c::W](W) writer structure"]
impl crate::Writable for TCCR3C_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCCR3C to value 0"]
impl crate::Resettable for TCCR3C_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
