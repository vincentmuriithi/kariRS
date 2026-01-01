#[doc = "Register `VERSION_NUM` reader"]
pub struct R(crate::R<VERSION_NUM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERSION_NUM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERSION_NUM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERSION_NUM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VERSION_NUM` writer"]
pub struct W(crate::W<VERSION_NUM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VERSION_NUM_SPEC>;
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
impl From<crate::W<VERSION_NUM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VERSION_NUM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VERSION_NUM` reader - Version Number"]
pub type VERSION_NUM_R = crate::FieldReader<u8, VERSION_NUM_A>;
#[doc = "Version Number\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VERSION_NUM_A {
    #[doc = "2: Revision A"]
    REV_A = 2,
    #[doc = "3: Revision B"]
    REV_B = 3,
}
impl From<VERSION_NUM_A> for u8 {
    #[inline(always)]
    fn from(variant: VERSION_NUM_A) -> Self {
        variant as _
    }
}
impl VERSION_NUM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VERSION_NUM_A> {
        match self.bits {
            2 => Some(VERSION_NUM_A::REV_A),
            3 => Some(VERSION_NUM_A::REV_B),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `REV_A`"]
    #[inline(always)]
    pub fn is_rev_a(&self) -> bool {
        *self == VERSION_NUM_A::REV_A
    }
    #[doc = "Checks if the value of the field is `REV_B`"]
    #[inline(always)]
    pub fn is_rev_b(&self) -> bool {
        *self == VERSION_NUM_A::REV_B
    }
}
#[doc = "Field `VERSION_NUM` writer - Version Number"]
pub type VERSION_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u8, VERSION_NUM_SPEC, u8, VERSION_NUM_A, 8, O>;
impl<'a, const O: u8> VERSION_NUM_W<'a, O> {
    #[doc = "Revision A"]
    #[inline(always)]
    pub fn rev_a(self) -> &'a mut W {
        self.variant(VERSION_NUM_A::REV_A)
    }
    #[doc = "Revision B"]
    #[inline(always)]
    pub fn rev_b(self) -> &'a mut W {
        self.variant(VERSION_NUM_A::REV_B)
    }
}
impl R {
    #[doc = "Bits 0:7 - Version Number"]
    #[inline(always)]
    pub fn version_num(&self) -> VERSION_NUM_R {
        VERSION_NUM_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Version Number"]
    #[inline(always)]
    #[must_use]
    pub fn version_num(&mut self) -> VERSION_NUM_W<0> {
        VERSION_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device Identification Register (Version Number)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [version_num](index.html) module"]
pub struct VERSION_NUM_SPEC;
impl crate::RegisterSpec for VERSION_NUM_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [version_num::R](R) reader structure"]
impl crate::Readable for VERSION_NUM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [version_num::W](W) writer structure"]
impl crate::Writable for VERSION_NUM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VERSION_NUM to value 0"]
impl crate::Resettable for VERSION_NUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
