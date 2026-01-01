#[doc = "Register `EXTENDED` reader"]
pub struct R(crate::R<EXTENDED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTENDED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTENDED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTENDED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTENDED` writer"]
pub struct W(crate::W<EXTENDED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTENDED_SPEC>;
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
impl From<crate::W<EXTENDED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTENDED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BOOTRST` reader - Boot Reset vector Enabled"]
pub type BOOTRST_R = crate::BitReader<bool>;
#[doc = "Field `BOOTRST` writer - Boot Reset vector Enabled"]
pub type BOOTRST_W<'a, const O: u8> = crate::BitWriter<'a, u8, EXTENDED_SPEC, bool, O>;
#[doc = "Field `BOOTSZ` reader - Select boot size"]
pub type BOOTSZ_R = crate::FieldReader<u8, BOOTSZ_A>;
#[doc = "Select boot size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BOOTSZ_A {
    #[doc = "0: Boot Flash size=1024 words Boot address=$0C00"]
    _1024W_0C00 = 0,
    #[doc = "1: Boot Flash size=512 words Boot address=$0E00"]
    _512W_0E00 = 1,
    #[doc = "2: Boot Flash size=256 words Boot address=$0F00"]
    _256W_0F00 = 2,
    #[doc = "3: Boot Flash size=128 words Boot address=$0F80"]
    _128W_0F80 = 3,
}
impl From<BOOTSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: BOOTSZ_A) -> Self {
        variant as _
    }
}
impl BOOTSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BOOTSZ_A {
        match self.bits {
            0 => BOOTSZ_A::_1024W_0C00,
            1 => BOOTSZ_A::_512W_0E00,
            2 => BOOTSZ_A::_256W_0F00,
            3 => BOOTSZ_A::_128W_0F80,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_1024W_0C00`"]
    #[inline(always)]
    pub fn is_1024w_0c00(&self) -> bool {
        *self == BOOTSZ_A::_1024W_0C00
    }
    #[doc = "Checks if the value of the field is `_512W_0E00`"]
    #[inline(always)]
    pub fn is_512w_0e00(&self) -> bool {
        *self == BOOTSZ_A::_512W_0E00
    }
    #[doc = "Checks if the value of the field is `_256W_0F00`"]
    #[inline(always)]
    pub fn is_256w_0f00(&self) -> bool {
        *self == BOOTSZ_A::_256W_0F00
    }
    #[doc = "Checks if the value of the field is `_128W_0F80`"]
    #[inline(always)]
    pub fn is_128w_0f80(&self) -> bool {
        *self == BOOTSZ_A::_128W_0F80
    }
}
#[doc = "Field `BOOTSZ` writer - Select boot size"]
pub type BOOTSZ_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, EXTENDED_SPEC, u8, BOOTSZ_A, 2, O>;
impl<'a, const O: u8> BOOTSZ_W<'a, O> {
    #[doc = "Boot Flash size=1024 words Boot address=$0C00"]
    #[inline(always)]
    pub fn _1024w_0c00(self) -> &'a mut W {
        self.variant(BOOTSZ_A::_1024W_0C00)
    }
    #[doc = "Boot Flash size=512 words Boot address=$0E00"]
    #[inline(always)]
    pub fn _512w_0e00(self) -> &'a mut W {
        self.variant(BOOTSZ_A::_512W_0E00)
    }
    #[doc = "Boot Flash size=256 words Boot address=$0F00"]
    #[inline(always)]
    pub fn _256w_0f00(self) -> &'a mut W {
        self.variant(BOOTSZ_A::_256W_0F00)
    }
    #[doc = "Boot Flash size=128 words Boot address=$0F80"]
    #[inline(always)]
    pub fn _128w_0f80(self) -> &'a mut W {
        self.variant(BOOTSZ_A::_128W_0F80)
    }
}
impl R {
    #[doc = "Bit 0 - Boot Reset vector Enabled"]
    #[inline(always)]
    pub fn bootrst(&self) -> BOOTRST_R {
        BOOTRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Select boot size"]
    #[inline(always)]
    pub fn bootsz(&self) -> BOOTSZ_R {
        BOOTSZ_R::new((self.bits >> 1) & 3)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Reset vector Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn bootrst(&mut self) -> BOOTRST_W<0> {
        BOOTRST_W::new(self)
    }
    #[doc = "Bits 1:2 - Select boot size"]
    #[inline(always)]
    #[must_use]
    pub fn bootsz(&mut self) -> BOOTSZ_W<1> {
        BOOTSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "No Description.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extended](index.html) module"]
pub struct EXTENDED_SPEC;
impl crate::RegisterSpec for EXTENDED_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [extended::R](R) reader structure"]
impl crate::Readable for EXTENDED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extended::W](W) writer structure"]
impl crate::Writable for EXTENDED_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTENDED to value 0"]
impl crate::Resettable for EXTENDED_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
